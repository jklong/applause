use applause::{applause, dispatch};
mod foo;

applause! {
    #[clap(name = "chain")]
    Cli {};

    commands {
        Foo(foo::Foo),
    };
}

fn main() {
    dispatch!(Cli);
}
