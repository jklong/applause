use std::fmt::Debug;

use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    braced,
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Paren},
    Attribute, Block, Field, Fields, FieldsNamed, FnArg, Ident, Token, Type,
};

mod kw {
    syn::custom_keyword!(trait_args);
    syn::custom_keyword!(commands);
}

#[derive(Debug, Parse, PartialEq)]
pub(crate) struct ParserDef {
    #[call(Attribute::parse_outer)]
    attrs: Vec<Attribute>,
    ident: Ident,

    #[brace]
    #[postfix(Token![;])]
    _brace_token: Brace,

    #[inside(_brace_token)]
    #[parse_terminated(Field::parse_named)]
    pub fields: Punctuated<Field, Token![,]>,
}

#[derive(Debug, Parse)]
pub(crate) struct TraitArgs {
    #[prefix(kw::trait_args)]
    #[postfix(Token![;])]
    #[paren]
    _paren: Paren,

    #[inside(_paren)]
    #[parse_terminated(FnArg::parse)]
    pub args: Punctuated<FnArg, Token![,]>,
}

#[derive(Debug, Parse)]
pub(crate) struct Commands {
    #[prefix(kw::commands)]
    #[postfix(Token![;])]
    #[brace]
    _brace: Brace,

    #[inside(_brace)]
    #[parse_terminated(CmdHandler::parse)]
    commands: Punctuated<CmdHandler<Variant2>, Token![,]>,
}

#[derive(Debug, Parse, Clone)]
pub(crate) struct CmdHandler<T>
where
    T: CmdVariant + Parse + Debug,
{
    enum_variant: T,
    #[prefix(Option<Token![=>]> as fat_arrow)]
    #[parse_if(fat_arrow.is_some())]
    handler: Option<Block>,
}

pub(crate) trait CmdVariant: Debug + Parse {
    /// Convert this CmdVariant into a destructuring pattern
    fn to_pat(&self) -> TokenStream;
}

/// A cut down Variant struct that doesn't take a determinant. Created
/// as the determinant in syn::Variant was colliding with the => syntax for our
/// DSL
#[derive(Debug, Parse, Clone)]
pub(crate) struct Variant2 {
    #[call(Attribute::parse_outer)]
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    #[call(parse_fields)]
    pub fields: Fields,
}

impl ToTokens for Variant2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let id = &self.ident;
        let fields = &self.fields;

        let tok = quote! {
            #(#attrs)*
            #id #fields
        };
        tokens.append_all(tok);
    }
}

/// Parse the fields part of a Variant into a Fields enum for Variant2.
fn parse_fields(input: syn::parse::ParseStream) -> syn::Result<Fields> {
    if input.peek(syn::token::Brace) {
        let content;
        let brace_token = braced!(content in input);
        let named = content.parse_terminated(Field::parse_named)?;
        Ok(Fields::Named(FieldsNamed { brace_token, named }))
    } else if input.peek(syn::token::Paren) {
        Ok(Fields::Unnamed(input.parse()?))
    } else {
        Ok(Fields::Unit)
    }
}

impl<'a> CmdVariant for Variant2 {
    fn to_pat(&self) -> TokenStream {
        let ident = &self.ident;
        let fields = self.fields.to_owned();
        match fields {
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|f| f.ident.to_owned());
                quote! { #ident {#(#field_idents)*} }
            }

            Fields::Unnamed(_fields) => {
                quote! {#ident(handler)}
            }

            _ => unimplemented!(),
        }
    }
}

impl CmdHandler<Variant2> {
    fn get_enum_variant(&self) -> TokenStream {
        let variant = &self.enum_variant;
        quote! {
            #variant
        }
    }

    fn get_impl(&self) -> Option<TokenStream> {
        match &self.handler {
            Some(closure) => Some(quote! {
              #closure
            }),
            None => match &self.enum_variant.fields {
                // If a closure is not specified and it's an unnamed field, try to treat it as a subcmd dispatch.
                // Assume that the clap args struct was generated by this macro and implements a basic xRun trait with a receiver only
                // i.e. there was no trait_args section used
                Fields::Unnamed(fu) => {
                    let handler_path = match &fu.unnamed.first().unwrap().ty {
                        Type::Path(tp) => Some(tp),
                        _ => None,
                    };

                    let ts = handler_path.map(|tp| {
                        // Use the type path that was given as the unnamed argument
                        // to guess the appropriate trait and enum paths
                        let mut cmd_enum_path = tp.path.clone();
                        let mut trait_path = tp.path.clone();

                        let enum_ps = cmd_enum_path.segments.last_mut().unwrap();
                        enum_ps.ident = format_ident!("{}Cmd", self.enum_variant.ident);

                        let trait_ps = trait_path.segments.last_mut().unwrap();
                        trait_ps.ident = format_ident!("{}Run", self.enum_variant.ident);

                        // Then call the trait's run method with the handler's subcmd
                        quote! {
                            <#cmd_enum_path as #trait_path>::run(&handler.cmd)
                        }
                    });

                    ts
                }
                _ => None,
            },
        }
    }
}

#[derive(Debug, Parse)]
pub(crate) struct ClapArgsWithSubs {
    parser: ParserDef,
    #[peek(kw::trait_args)]
    trait_args: Option<TraitArgs>,
    commands: Commands,
}

impl ToTokens for ClapArgsWithSubs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _type_alias = self.type_alias();
        let _clap_struct = self.parser_struct();
        let _dispatch_trait = self.dispatch_trait();
        let _subcmd_enum = self.subcmd_enum();
        let _subcmd_impl = self.subcmd_impl();

        tokens.append_all(quote! {
            #_type_alias // The outward-facing type alias
            #_clap_struct // The struct deriving ::clap::Parser

            #_dispatch_trait // The trait used for dispatch

            #_subcmd_enum // The enum listing the subcommands
            #_subcmd_impl // impl of the dispatch trait for the command enum

        })
    }
}

impl Commands {
    fn to_variants(&self) -> Vec<TokenStream> {
        let cmds = self.commands.clone();
        cmds.into_iter().map(|ch| ch.get_enum_variant()).collect()
    }

    fn impls(&self) -> Vec<TokenStream> {
        let cmds = self.commands.clone();
        cmds.into_iter()
            .map(|ch| {
                let variant_pat = &ch.enum_variant.to_pat();
                match &ch.get_impl() {
                    Some(ts) => quote! {
                        #variant_pat => #ts
                    },
                    None => {
                        quote! { #variant_pat => {unimplemented!();} }
                    }
                }
            })
            .collect()
    }
}

impl ClapArgsWithSubs {
    fn struct_ident(&self) -> Ident {
        format_ident!("{}Parser", self.parser.ident)
    }

    fn subcmd_enum_ident(&self) -> Ident {
        format_ident!("{}Cmd", self.parser.ident)
    }

    fn trait_ident(&self) -> Ident {
        format_ident!("{}Run", self.parser.ident)
    }

    pub(crate) fn parser_struct(&self) -> TokenStream {
        let name = self.struct_ident();
        let attrs = &self.parser.attrs;
        let trait_ident = self.trait_ident();
        let fields = self.parser.fields.iter();
        quote! {
            #[derive(::applause::clap::Parser, Debug)]
            #(#attrs)*
            pub struct #name<T>
            where T: ::applause::clap::Subcommand + #trait_ident  {
                #[clap(subcommand)]
                pub cmd: T,
                #(#fields),*
            }
        }
    }

    pub(crate) fn type_alias(&self) -> TokenStream {
        let name = &self.parser.ident;
        let subcmd_enum = self.subcmd_enum_ident();
        let struct_ident = self.struct_ident();

        quote! {
            pub type #name = #struct_ident<#subcmd_enum>;
        }
    }

    pub(crate) fn trait_args(&self) -> Punctuated<FnArg, Token![,]> {
        let receiver: FnArg = syn::parse_str("&self").expect("Could not create reciever");
        let parsed_args = &self.trait_args;
        let empty: Punctuated<FnArg, Token![,]> = Punctuated::new();

        // Grab the parsed args or default to an empty Punctuated
        let mut args = parsed_args
            .as_ref()
            .map(|ta| ta.args.clone())
            .unwrap_or_else(|| empty);
        args.insert(0, receiver);

        args
    }

    pub(crate) fn dispatch_trait(&self) -> TokenStream {
        let name = self.trait_ident();
        let args = self.trait_args();
        quote! {
            pub trait #name: ::core::fmt::Debug {
                fn run(#args) -> ();
            }
        }
    }

    pub(crate) fn subcmd_enum(&self) -> TokenStream {
        let enum_ident = self.subcmd_enum_ident();
        let enum_vars = self.commands.to_variants();

        quote! {
                #[derive(::applause::clap::Subcommand, Debug)]
                pub enum #enum_ident {
                    #(#enum_vars),*
                }
        }
    }

    pub(crate) fn subcmd_impl(&self) -> TokenStream {
        let impls = self.commands.impls();
        let trait_ident = self.trait_ident();
        let trait_args = self.trait_args();
        let enum_ident = self.subcmd_enum_ident();

        quote! {
            #[allow(unused_variables)]
            impl #trait_ident for #enum_ident {
                fn run(#trait_args) {
                    match &self {
                        #(
                            Self::#impls
                        ),*
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use proc_macro2::Span;
    use quote::quote;

    use super::*;

    #[test]
    fn parserdef_empty() {
        let tokens = quote! {
            Test {};
        };
        let wanted = ParserDef {
            attrs: vec![],
            ident: Ident::new("Test", Span::call_site()),
            _brace_token: Brace::default(),
            fields: Punctuated::new(),
        };

        let got = syn::parse2::<ParserDef>(tokens).unwrap();

        assert_eq!(wanted, got);
    }

    #[test]
    fn parserdef_struct_attrs() {
        let tokens = quote! {
            #[test_attr]
            Test {};
        };

        let attr = syn::parse_quote! {#[test_attr]};

        let wanted = ParserDef {
            attrs: vec![attr],
            ident: Ident::new("Test", Span::call_site()),
            _brace_token: Brace::default(),
            fields: Punctuated::new(),
        };

        let got = syn::parse2(tokens).unwrap();

        assert_eq!(wanted, got);
    }

    #[test]
    fn parserdef_fields() {
        let tokens = quote! {
            Test {
                test: String,
            };
        };

        let mut fields = Punctuated::new();
        let f = Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            ident: Some(Ident::new("test", Span::call_site())),
            colon_token: Some(Token![:](Span::call_site())),
            ty: syn::parse_str("String").unwrap(),
        };
        fields.push(f);
        fields.push_punct(syn::token::Comma::default());

        let wanted = ParserDef {
            attrs: vec![],
            ident: Ident::new("Test", Span::call_site()),
            _brace_token: syn::token::Brace::default(),
            fields,
        };

        let got = syn::parse2(tokens).unwrap();

        assert_eq!(wanted, got);
    }

    #[test]
    fn parserdef_field_attrs() {
        let tokens = quote! {
            Test {
                #[field_attr(ff)]
                test: String,
            };
        };

        let attr = syn::parse_quote! {#[field_attr(ff)]};
        // TODO: Refactor to a helper
        let mut fields = Punctuated::new();
        let f = Field {
            attrs: vec![attr],
            vis: syn::Visibility::Inherited,
            ident: Some(Ident::new("test", Span::call_site())),
            colon_token: Some(Token![:](Span::call_site())),
            ty: syn::parse_str("String").unwrap(),
        };
        fields.push(f);
        fields.push_punct(syn::token::Comma::default());

        let wanted = ParserDef {
            attrs: vec![],
            ident: Ident::new("Test", Span::call_site()),
            _brace_token: syn::token::Brace::default(),
            fields,
        };

        let got = syn::parse2(tokens).unwrap();

        assert_eq!(wanted, got);
    }
}
