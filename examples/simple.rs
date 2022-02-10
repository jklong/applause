use applause::{applause, dispatch};

applause! {
    Cli {};

    commands {
        Cmd1{} => { do_cmd1() },
    };
}

fn do_cmd1() {
    println!("Doing cmd1");
}

fn main() {
    dispatch!(Cli);
}
