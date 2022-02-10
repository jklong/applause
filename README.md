# applause

Applause is a basic DSL that allows for easy construction of [Clap] `App`s that
make use of subcommands.

It magicks away the endless `match`es and `enum`s to define subcommands,
arguments and handlers in favour of a simple dispatch system while passing as
much as possible back to `clap` via its `derive` API.

## DSL Syntax

The `applause` DSL is simple and aims to mimic normal Rust syntax.

The definition of the DSL syntax in [Rust Reference
Notation][RustReference] is as follows. Where syntactical items are as defined
in the Reference, the appropriate page is linked.

See [Examples](examples) for complete examples.

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
> &nbsp;&nbsp; [IDENTIFIER] (
> [_EnumItemTuple_][RREnum] |&nbsp;
> [_EnumItemStruct_][RREnum]
> )\
> &nbsp;&nbsp; _DispatchExpr_<sup>?</sup>\
> &nbsp;&nbsp; `,`
>
> _DispatchExpr_ :\
> &nbsp;&nbsp; `=>` [_BlockExpression_]

### ParserDef

`TODO`

### DispatchParams

`TODO`

### Commands

`TODO: Rewrite more to do with mapping to clap uses`

The syntax of a _CommandDef_ is Rust-like but does not behave like any of the
Rust syntactical constructs it resembles.

The IDENTIFIER, _EnumItemTuple_ and _EnumItemStruct_ are used to generate an
item in an `enum`. This `enum` is processed by `clap` via the `clap::Subcommand`
derive macro to define the valid subcommands.

#### DispatchExpr

Each subcommand may also have an associated
_DispatchExpr_. This is inserted into the dispatcher for that subcommand's match arm.

For example, a subcommand defined as

```rust
commands {
    Foo(Foo) => { handler.foo(); },
};
```

would produce a match arm like

```rust
match &self {
    Foo(handler) => { handler.foo(); },
}
```

Note that the enum variant is automatically destructured into `handler`, which can be referenced in the _DispatchExpr_. This is mainly useful for referring to `clap` structs defined outside `applause`.

#### Struct-like Enum Variants

A struct-like enum variant is automatically destructured to its named fields. A _CommandDef_ of

```rust
Foo {name: String} => { println!("Saying foo to {}", name) },
```

would produce a match arm of

```rust
match &self {
    // SNIP
    Foo {name} => { println!("Saying foo to {}", name) }
    // SNIP
}
```

`TODO: Talk about how clap treats these as args for the subcommand`

#### Chaining Dispatches

As a shorthand, if no _BlockExpression_ is specified for an enum variant with an
unnamed field, for example `Foo(commands::Foo),`,  `applause` will assume that the contained type is another
`applause` generated `Parser`. It will then attempt to invoke its dispatch method with no other parameters.

This means a command line such as `my_exe foo bar` can be handled by

```rust
// main.rs - Top level
mod commands;
applause!{
    Cli{};

    commands {
        Foo(commands::Foo),
    };
}
```

```rust
// commands.rs - Subcommands
applause!{
    Foo{};

    commands {
        Bar(Bar) => { /* Do a bar */ },
    }
}
```

## Helper Macros

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
