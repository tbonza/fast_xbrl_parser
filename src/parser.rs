
use std::collections::HashMap;

struct Dimension {
  key_ns : String,
  key_value : String,
  member_ns : String,
  member_value : String,
}

struct Unit {
  unit_type : String,
  unit_value : String
}

struct Period {
  period_type : String,
  period_value : String
}

struct FactItem {
  id : String,
  prefix: String,
  name : String,
  value : String,
  decimals : String,
  context_ref : Option<String>,
  unit_ref : Option<String>,
  dimensions : Vec<Dimension>,
  units : Vec<Unit>,
  periods : Vec<Period>
}

struct DimensionTableRow {
  cik : Option<String>,
  accession_number : Option<String>,
  xml_name : String,
  context_ref : String,
  axis_prefix : String,
  axis_tag : String,
  member_prefix : String,
  member_tag : String,
}

impl DimensionTableRow {
    fn default() -> DimensionTableRow {
        DimensionTableRow {
            cik : None,
            accession_number : None,
            xml_name : "".to_string(),
            context_ref : "".to_string(),
            axis_prefix : "".to_string(),
            axis_tag : "".to_string(),
            member_prefix : "".to_string(),
            member_tag : "".to_string(),
        }
    }
}

struct FactTableRow {
  cik : Option<String>,
  accession_number : Option<String>,
  xml_name : String,
  context_ref : Option<String>,
  tag : String,
  value : String,
  prefix : String,
  period_start : Option<String>,
  period_end : Option<String>,
  point_in_time : Option<String>,
  unit : Option<String>,
  num_dim : u32,
}

fn facts_to_table(facts : Vec<FactItem>, input_details : InputDetails)
    -> Vec<FactTableRow> {

    let mut table_rows : Vec<FactTableRow> = Vec::new();

    for fact in facts {

        let mut row = FactTableRow::default();
        row.cik = input_details.cik.clone();
        row.accession_number = input_details.accession_number.clone();
        row.xml_name = input_details.xml_name.clone();
        row.context_ref = fact.context_ref.clone();
        row.tag = fact.name.clone();
        row.prefix = fact.prefix.clone();
        row.num_dim = fact.dimensions.len() as u32;
        row.value = fact.value.clone();

        // Periods are extracted into three different columns.
        for period in &fact.periods {
            match period.period_type.as_str() {
                "startDate" => row.period_start = Some(period.period_value.clone()),
                "endDate" => row.period_end = Some(period.period_value.clone()),
                "instant" => row.point_in_time = Some(period.period_value.clone()),
                _ => {}
            }
        };

        // The units are converted into a single string
        if fact.units.len() > 0 {
            let tmp = fact.units.clone()
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" || ");

            row.unit = Some(tmp.clone());
        }
        
        table_rows.push(row);
    }

    table_rows;
}

fn parse_xml_to_facts(raw_xml : String) -> Vec<FactItem> {

    // -- Parse the XML --
    let xml_tree = roxmltree::Document::parse(
        raw_xml.as_str()
        ).expect("Invalid XML document."); 

    // -- Get elements out of XML --
    let elem = xml_tree
        .root_element()
        .children()
        .filter(|e| e.node_type() == roxmltree::NodeType::Element);

    // -- Process the context elements --

    let mut units: HashMap<String,Vec<Unit>> = HashMap::new();
    let mut periods: HashMap<String,Vec<Period>> = HashMap::new();
    let mut dimensions: HashMap<String,Vec<Dimension>> = HashMap::new();

    // --- Process the unit elements ---
    let unit_ele = elem.clone().filter(|e| e.tag_name().name() == "unit");
    '_unit_loop: for (_i, child) in unit_ele.enumerate() {
        let id = child.attribute("id").unwrap_or("");
        let measure_nodes = child
            .descendants()
            .filter(|e| e.tag_name().name() == "measure");

        for (_i, m_ele) in measure_nodes.enumerate() {
            let name = m_ele.parent().unwrap().tag_name().name();
            let value = m_ele.text().unwrap_or("");
            units.entry(id.to_string())
                .or_default()
                .push(Unit {
                    unit_type : name.to_string(),
                    unit_value : value.to_string()
                });
        }
    }

    // --- Process the context elements ---
    
    let context_ele = elem.clone().filter(|e| e.tag_name().name() == "context");
    '_context_loop: for (_i, child) in context_ele.enumerate() {

        let id = child.attribute("id").unwrap_or("");

        let node_desc = child
            .children()
            .filter(|e| e.node_type() == roxmltree::NodeType::Element);

        // loop over descendants and process the different types of elements
        for (_i, child_ele) in node_desc.enumerate() {
            match child_ele.tag_name().name() {
                "period" => {
                    let to_keep = ["instant", "startDate", "endDate"];
                    let node_desc_filtered = child_ele
                        .descendants()
                        .filter(|e| to_keep.contains(&e.tag_name().name()));

                    for (_i, child_ele_filtered) in node_desc_filtered.enumerate() {
                        let value = child_ele_filtered.text().unwrap_or("");
                        let name = child_ele_filtered.tag_name().name();
                        let _namespace = child_ele_filtered
                            .tag_name()
                            .namespace().unwrap_or("");

                        periods.entry(id.to_string())
                            .or_default()
                            .push(Period {
                                period_type : name.to_string(),
                                period_value : value.to_string()
                            });
                    }
                }
                "entity" => {
                    let to_keep = ["explicitMember"];
                    let node_desc_filtered = child_ele
                        .descendants()
                        .filter(|e| to_keep.contains(&e.tag_name().name()));

                    for (_i, child_ele_filtered) in node_desc_filtered.enumerate() {
                        let value = child_ele_filtered.text().unwrap_or("");
                        let _name = child_ele_filtered.tag_name().name();
                        let _namespace = child_ele_filtered
                            .tag_name().namespace().unwrap_or("");
                        if child_ele_filtered.has_attribute("dimension") {
                            let dimension_raw = child_ele_filtered
                                .attribute("dimension").unwrap();
                            let dimension_split = dimension_raw
                                .split(":").collect::<Vec<&str>>();
                            let dimension_ns = dimension_split[0];
                            let dimension_value = dimension_split[1];

                            let value_split = value
                                .split(":").collect::<Vec<&str>>();
                            let key_ns = value_split[0];
                            let key_value = value_split[1];

                            dimensions.entry(id.to_string())
                                .or_default()
                                .push(Dimension {
                                    key_ns : dimension_ns.to_string(),
                                    key_value : dimension_value.to_string(),
                                    member_ns : key_ns.to_string(),
                                    member_value : key_value.to_string()
                                });

                        }
                    }
                }
                _ => {}
            }
        }
    }

    // -- Process the fact elements --
    
    let mut facts: Vec<FactItem> = Vec::new();

    let non_fact_ele = ["context", "unit", "xbrl", "schemaRef"];
    let fact_ele = elem
        .clone()
        .filter(|e| !&non_fact_ele
            .contains(&e
                .tag_name()
                .name()) && e.tag_name().namespace().is_some());

    // loop over fact_ele using enumerate
    '_fact_loop: for (_i, child) in fact_ele.enumerate() {
        let id = child.attribute("id").unwrap_or(""); // Issue here
        let name: String = child.tag_name().name().to_string();
        let namespace: String = child
            .tag_name()
            .namespace().unwrap_or("").to_string();
        let prefix = child.lookup_prefix(namespace.as_str()).unwrap_or(""); 
        let context_ref = &child.attribute("contextRef");
        let unit_ref = &child.attribute("unitRef");
        let decimals = child.attribute("decimals").unwrap_or("");
        let value = child.text().unwrap_or("");

        // Sanitize the value
        let clean_value = sanitize::html(value.to_string().clone());

        let mut fact_dimensions : Vec<Dimension> = Vec::new();
        let mut fact_units : Vec<Unit> = Vec::new();
        let mut fact_periods : Vec<Period>= Vec::new();

        // Look up the units 
        if unit_ref.is_some() {
            let unit_ref_value = unit_ref.unwrap().to_string();
            if units.contains_key(&unit_ref_value) {
                fact_units = units.get(&unit_ref_value)
                    .expect("Unit not found").clone();
            }
        }

        // Look up the dimensions
        if context_ref.is_some() {
            let context_ref_value = context_ref.unwrap().to_string();
            if dimensions.contains_key(&context_ref_value) {
                fact_dimensions = dimensions
                    .get(&context_ref_value)
                    .expect("Dimension not found").clone();
            }
        }

        // Look up the periods
        if context_ref.is_some() {
            let context_ref_value = context_ref.unwrap().to_string();
            if periods.contains_key(&context_ref_value) {
                fact_periods = periods
                    .get(&context_ref_value)
                    .expect("Period not found").clone();
            }
        }

        // Push to output vector
        
        facts.push(FactItem {
            id : id.to_string(),
            prefix: prefix.to_string(),
            name : name.to_string(),
            value : clean_value,
            decimals : decimals.to_string(),
            context_ref : context_ref.map(str::to_string),
            unit_ref : unit_ref.map(str::to_string),
            units : fact_units,
            dimensions : fact_dimensions,
            periods : fact_periods,
            ..FactItem::default()
        });
    }

    facts;
}







