use applause::{applause, dispatch};

applause! {
    #[clap(name = "attrs_example")]
    Cli {};

    commands{
        /// Subcommand 1
        #[clap(alias = "1")]
        Cmd1{} => { println!("cmd1") },
    };
}

fn main() {
    dispatch!(Cli)
}
