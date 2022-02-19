use applause::{applause, dispatch};

applause! {
    // Define the parser with additional attributes for clap to parse
    #[clap(name = "attrs_example")]
    Cli {};

    commands{
        // Define the subcommand with a docstring for help text as well
        // as clap attributes
        /// Subcommand 1
        #[clap(alias = "1")]
        Cmd1{} => { println!("cmd1") },
    };
}

fn main() {
    dispatch!(Cli)
}
