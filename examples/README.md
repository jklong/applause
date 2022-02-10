# Examples

## Simple

[simple.rs](simple.rs) shows the usage of `applause` at its most basic. It
defines a parser with a single subcommand with no other args. The _DispatchExpr_
for the subcommand calls a predefined function.

It then uses the `dispatch!` helper macro to parse and dispatch to the function.

```console
$ cargo run -q --example simple -- cmd1
Doing cmd1
```

## Attrs

[attrs](attrs.rs) shows how attributes are passed through to be used as normal.
This includes doc comments which can be used by `clap`.

```console
$ cargo run -q --example attrs
attrs_example 

USAGE:
    attrs <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    cmd1    Subcommand 1
    help    Print this message or the help of the given subcommand(s)
```
