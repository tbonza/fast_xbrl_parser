# FXP

Fork of [TiesdeKok/fast_xbrl_parser](https://github.com/TiesdeKok/fast_xbrl_parser),
simplifying package to just a command line interface for now. The input will be
an XBRL document, and the output will be in [JSON Lines](https://jsonlines.org/) 
format.

Proposed usage is:

```
$ input_file_path="path-to-your-xbrl-document.xml"
$ output_file_path="path-to-parsed-xbrl-facts.jsonl"
$ cat $input_file_path | fxp > $output_file_path
```
