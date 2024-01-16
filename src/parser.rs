
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
