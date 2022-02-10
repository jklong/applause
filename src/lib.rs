// Re-export the main proc macro
pub use applause_proc_macros::clap_args_with_subcommands;

// Declarative helper macros
#[macro_export]
macro_rules! dispatch_args {
    ($ty:ty => $($args:expr),*) => {
        <$ty as ::clap::Parser>::parse().cmd.run($($args),*);
    };
}

#[macro_export]
macro_rules! dispatch {
    ($ty:ty => $($args:expr),*) => {
        <$ty as ::clap::Parser>::parse().cmd.run($($args),*);
    };
    ($ty:ty) => {
        dispatch_args!($ty => );
    };
}

#[macro_export]
macro_rules! parse_args {
    ($ty:ty) => {
        <$ty as ::clap::Parser>::parse()
    };
}
