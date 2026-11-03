# Derivations of Loid

Loid is a large and complex language.
And often, it is highly preferred to define parts of your codebase in a subset of the Loid programming language.

Loid has first-class support for this by the existence of the Loid Derivations, which are programming languages that are subsets of the Loid programming language.

## Types of Derivations and Naming Notation

TO-DO

## How Derivations are Written and to be Used

TO-DO

## Writing New Derivations

TO-DO

## `LoidT`

Loid's type system is in itself a turing complete programming language.

It can be typed separately from a normal Loid file in a `.loidt` file.

These cannot be compiles to anything, it is simply for types. These can be imported into normal Loid files however.

TO-DO (documentation)

## `LoidF`

Loid allows for pure functional programming.

And for some specific pieces of code, pure functions are highly preferred.

The `.loidf` subset of the Loid language is for entirely pure functional programming.

TO-DO (documentation)

## `LoidX`

Loid to JSX

TO-DO

## `NLoid`

Dialect of Loid for compiling Loid code to Nix code.

TO-DO (documentation)

## `JLoid`

Dialect of Loid for compiling Loid code cleanly and directly into JavaScript.
Isn't the same as the standard JavaScript target, but rather becomes a language that is directly compatible with JavaScript but with a slightly alternative syntax.

TO-DO (documentation)

## `CLoid`

Dialect of Loid that compiles to pure `C` code

TO-DO (documentation)

## `CSSoid`

CSS in Loid

TO-DO

## `Lispoid`

Lisp-like Loid

TO-DO

## `NecraLoid`

Warfarme inspired dialect of Loid that compiles to standard Loid

TO-DO (literally just some fun stuff that I'll write when I have nothing else to do, this is completely irrelevant)
