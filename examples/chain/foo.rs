use applause::applause;

applause! {
    Foo {};

    commands {
        /// Do a bar
        Bar{} => { bar() },
        /// Do a baz
        Baz{} => { baz() },
    };
}

fn bar() {
    println!("Doing bar");
}

fn baz() {
    println!("Doing baz");
}
