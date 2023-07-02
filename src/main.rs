#![allow(unused)]

use clap::Parser;

/// Nut - fast, scalable, distributed revision control system. For tracing changes of resources outside of repository via the files in the repository. Each change trakced as a version via repository file change, that can be later referenced by the other party in reliable way.
#[derive(Parser)]
struct Cli {
    /// The command to execute
    command: String,
}

fn main() {
    let args = Cli::parse();

    if (args.command == "init") {
        println!("nut init not implemented");
        return;
    }

    if (args.command == "log") {
        println!("nut log not implemented");
        return;
    }

    if (args.command == "commit") {
        println!("nut commit not implemented");
        return;
    }

    if (args.command == "push") {
        println!("nut push not implemented");
        return;
    }

    if (args.command == "cat-file") {
        println!("nut cat-file not implemented");
        return;
    }

    println!("nut unknown command");
}
