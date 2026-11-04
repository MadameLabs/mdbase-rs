//! `mdbase-local`: a linha de comando direta do motor (`mdbase_command::DirectArgs`)
//! sobre uma coleção local. Projeto da Madame Labs, não do mdbase-dev.

use clap::Parser;
use mdbase_command::{execute_args, DirectArgs};

fn main() {
    let args = DirectArgs::parse();
    let pretty = args.pretty;
    let result = execute_args(args);
    let text = if pretty {
        serde_json::to_string_pretty(&result.value)
    } else {
        serde_json::to_string(&result.value)
    }
    .expect("resultado serializa");
    println!("{text}");
    std::process::exit(result.exit_code);
}
