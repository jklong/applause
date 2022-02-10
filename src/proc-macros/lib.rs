mod clap_args;

use clap_args::ClapArgsWithSubs;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro]
pub fn clap_args_with_subcommands(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClapArgsWithSubs);

    input.to_token_stream().into()
}
