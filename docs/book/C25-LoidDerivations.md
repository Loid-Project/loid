# Derivations of Loid

Loid is a large and complex language.
And often, it is highly preferred to define parts of your codebase in a subset of the Loid programming language.

Loid has first-class support for this by the existence of the Loid Derivations, which are programming languages that are subsets of the Loid programming language.

## `LoidT`

Loid's type system is in itself a turing complete programming language.

It can be typed separately from a normal Loid file in a `.loidt` file.

## `LoidF`

Loid allows for pure functional programming.

And for some specific pieces of code, pure functions are highly preferred.

The `.loidf` subset of the Loid language is for entirely pure functional programming.

## `LoidX`

## `NLoid`

Dialect of Loid for compiling Loid code to Nix code.

## `JLoid`

Dialect of Loid for compiling Loid code cleanly and directly into JavaScript.
Isn't the same as the standard JavaScript target, but rather becomes a language that is directly compatible with JavaScript but with a slightly alternative syntax.
