extern crate mustache;
extern crate docopt;

use docopt::Docopt;
use std::fs::File;
use std::io::Read;
use std::io;
use std::env;
use std::collections::hash_map::HashMap;

static USAGE : &'static str = "
rustache
Template with environment variables

Usage:
  rustache <file>
";

fn env_vars_hash_map() -> HashMap<String, String> {
    env::vars().collect()
}

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    let filename = args.get_str("<file>");

    let mut open_file = File::open(filename).ok().expect(&format!("File {} not found", filename));

    let mut template_content = String::new();
    open_file.read_to_string(&mut template_content).ok().expect(&format!("Not able to read the content of: {}", filename));

    let template = mustache::compile_str(&template_content);
    template.render(&mut io::stdout(), &env_vars_hash_map()).unwrap();
}
