extern crate argparse;

use argparse::{ArgumentParser, List, Store};
use libgit;

fn main() {
    let mut subcommand: String = String::from("");
    let mut args: Vec<String> = vec![];

    {
        let mut ap = ArgumentParser::new();

        // Parse subcommand - e.g. init, add, commit, etc.
        ap.refer(&mut subcommand).required().add_argument(
            "command",
            Store,
            "Command to run - e.g. \"init\", \"add\"",
        );

        // Parse all extra args
        ap.refer(&mut args)
            .add_argument("arguments", List, "Extra arguments for command");
        ap.parse_args_or_exit();
    }

    match subcommand.as_str() {
        "init" => libgit::init(),
        _ => println!("Bad command"),
    }
}
