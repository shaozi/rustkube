extern crate clap;
extern crate handlebars;
extern crate serde_json;
extern crate serde_yaml;

use clap::App;
use handlebars::Handlebars;
use serde_json::Value;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn main() {
    let matches = App::new("Kubenetes template yaml parser")
        .version("1.0")
        .author("Jingshao Chen")
        .about("make kubernetes yaml templates")
        .args_from_usage(
            "-v, --variables=[JSON] 'JSON variables as a string'
            -f, --var-file=[FILENAME] 'variables json or yaml file'
            <YAMLFILE>              'Sets the input yaml file to use'",
        )
        .get_matches();

    let variables = matches.value_of("variables").unwrap_or("{}");
    let var_filename = matches.value_of("var-file").unwrap_or("");

    let vars_in_args: Value =
        serde_json::from_str(variables).expect("variables must be a json formatted string");

    let ext =
        get_extension_from_filename(&var_filename).expect("input file name need an extension");

    if ext != "json" && ext != "yaml" && ext != "yml" {
        panic!("Don't know how to handle input var file - must have json or yaml extension in file name");
    }
    let var_file = File::open(var_filename).expect("file cannot be opened");
    let reader = BufReader::new(var_file);
    let mut vars = if ext == "json" {
        serde_json::from_reader(reader).expect("file can not be parsed")
    } else {
        serde_yaml::from_reader(reader).expect("file can not be parsed")
    };

    merge(&mut vars, &vars_in_args);

    let yaml_file = matches.value_of("YAMLFILE").unwrap();

    let file_content = fs::read_to_string(yaml_file).expect("Failed to read file");
    let reg = Handlebars::new();
    let result = reg
        .render_template(&file_content, &vars)
        .expect("Template render error");
    println!("{}", result);
}
