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

## Simple With Args

[simple_with_args.rs](simple_with_args.rs) show how `clap` arguments are
defined at both the top level within the _ParserDef_ block and in subcommands in
the _Commands_ block.

```console
$ cargo run -q --example simple_with_args -- -n 2 cmd1 foo
Got num: 2
Dispatching to subcommand...
I am foo
```

## Attrs

[attrs.rs](attrs.rs) shows how attributes are passed through to be used as normal.
This includes doc comments which can be used by `clap`.

```console
$ cargo run -q --example attrs # Test that attributes and docstrings are preserved
attrs_example 

USAGE:
    attrs <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    cmd1    Subcommand 1
    help    Print this message or the help of the given subcommand(s)
```

```console
$ cargo run -q --example attrs -- 1 # Test that the alias for cmd1 works
cmd1
```

## Chain

[chain.rs](chain) shows how dispatches can easily be chained to sub-subcommands.

```console
$ # Top level help
$ cargo run -q --example chain
chain 

USAGE:
    chain <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    foo     
    help    Print this message or the help of the given subcommand(s)
```

```console
$ # subcommand help
$ cargo run -q --example chain -- help foo
chain-foo 

USAGE:
    chain foo <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    bar     Do a bar
    baz     Do a baz
    help    Print this message or the help of the given subcommand(s)
```

```console
$ # Dispatch to sub-subcommands
$ cargo run -q --example chain -- foo bar
Doing bar
$ cargo run -q --example chain -- foo baz
Doing baz
```
