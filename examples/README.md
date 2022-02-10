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
