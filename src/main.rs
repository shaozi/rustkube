extern crate handlebars;
extern crate clap;
extern crate serde_json;

use clap::App;
use handlebars::Handlebars;
use serde_json::Value;
use std::fs;

fn main() {
    let matches = App::new("Kubenetes template yaml parser")
        .version("1.0")
        .author("Jingshao Chen")
        .about("make kubernetes yaml templates")
        .args_from_usage(
            "-v, --variables=[JSON] 'Set JSON variables'
            <YAMLFILE>              'Sets the input yaml file to use'",
        )
        .get_matches();

    let variables = matches.value_of("variables").unwrap_or("{}");
    let values: Value =
        serde_json::from_str(variables).expect("variables must be a json formatted string");
    let yaml_file = matches.value_of("YAMLFILE").unwrap();

    let file_content = fs::read_to_string(yaml_file).expect("Failed to read file");
    let reg = Handlebars::new();
    let result = reg
        .render_template(&file_content, &values)
        .expect("Template render error");
    println!("{}", result);
}
