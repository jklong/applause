# applause

Applause is a basic DSL that allows for easy construction of [Clap] `App`s that
make use of subcommands.

It magicks away the endless `match`es and `enum`s to define subcommands,
arguments and handlers in favour of a simple dispatch system while passing as
much as possible back to `clap` via its `derive` API.

## DSL Syntax

The `applause` DSL is simple and aims to mimic normal Rust syntax.

The DSL defines:

* The fields of the `struct` that will derive [clap::Parser][Parser].

* An optional list of additional parameters that will be passed to the dispatcher.

* A list of subcommands. Each subcommand may have an associated
_BlockExpression_ which is inserted into the dispatcher as a handler for that subcommand.

The definition of the DSL syntax in [Rust Reference
Notation][RustReference] is as follows. Where syntactical items are as defined
in the Reference, the appropriate page is linked.

See the [Examples](#Examples) section of this document for complete examples.

<!-- TODO: Review the Rust Reference repo to find a better way to do this -->

> **<sup>Syntax</sup>**\
> _Applause_ :\
> &nbsp;&nbsp; _ParserDef_ \
> &nbsp;&nbsp; _DispatchParams_<sup>?</sup>\
> &nbsp;&nbsp; _Commands_
>
> _ParserDef_ :\
> &nbsp;&nbsp; [IDENTIFIER]&nbsp;
> `{` [_StructFields_]<sup>?</sup> `}`
> `;`
>
> _DispatchParams_ :\
> &nbsp;&nbsp; `dispatch_params`
> `(` [_FunctionParams_] `)`
> `;`
>
>
> _Commands_ :\
> &nbsp;&nbsp; `commands`&nbsp;
> `{` _CommandDef_ <sup>+</sup> `}`
> `;`
>
> _CommandDef_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [IDENTIFIER] (
> [_EnumItemTuple_][RREnum] |&nbsp;
> [_EnumItemStruct_][RREnum]
> )
> ( `=>` [_BlockExpression_] )<sup>?</sup>
> `,`
>

## Examples

[Clap]: https://github.com/clap-rs/clap
[RustReference]: https://doc.rust-lang.org/reference/notation.html
[IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
[_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
[_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
[_FunctionParams_]: https://doc.rust-lang.org/reference/items/functions.html
[_StructFields_]: https://doc.rust-lang.org/reference/items/structs.html
[RREnum]: https://doc.rust-lang.org/reference/items/enumerations.html
[Parser]: https://docs.rs/clap/latest/clap/trait.Parser.html
[_BlockExpression_]:
    https://doc.rust-lang.org/reference/expressions/block-expr.html
