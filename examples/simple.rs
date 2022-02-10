use applause::{applause, dispatch};

applause! {
    // Define a Parser called Cli
    Cli {};

    commands {
        // Create a Cmd1 subcommand that takes no arguments
        // and calls do_cmd1() from its DispatchExpr
        Cmd1{} => { do_cmd1() },
    };
}

// do_cmd1 as called from the DispatchExpr
fn do_cmd1() {
    println!("Doing cmd1");
}

fn main() {
    // Parse the arguments and dispatch
    dispatch!(Cli);
}
