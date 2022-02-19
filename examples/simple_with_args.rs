use applause::{applause, parse_args};

applause! {
    // Define a Parser called Cli that takes an optional "num" argument
    // identified by -n or --num
    Cli {
        #[clap(long, short)]
        num: usize,
    };

    commands {
        // Define a Subcommand that takes a positional parameter called "name"
        Cmd1 { name: String } => { println!("I am {}", name); },
    };
}

fn main() {
    let cli = parse_args!(Cli);
    println!("Got num: {}", cli.num);
    println!("Dispatching to subcommand...");
    cli.cmd.run();
}
