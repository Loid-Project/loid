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

## `Loiderem`

Loiderem is to be a purely functional programming language that is a subset of Loid and is designed with the singular purpose of theorem proving.

It will include a very basic addition of the IO monad, and no other monads.

Loiderem will be used strictly as a proof assistant and have an IO to print out occasional results.

This will be implemented further down in time, but for now, Loid can technically serve as a proof assistant, it'll just be very annoying as it isn't designed for specifically that.

Due to this, we are okay making Loid less capable of proof-assistant-like actions at times, as Loiderem is to be used for this.

## `LoidT`

Loid's type system is in itself a turing complete programming language.

It can be typed separately from a normal Loid file in a `.loidt` file.

These cannot be compiles to anything, it is simply for types. These can be imported into normal Loid files however.

TO-DO (documentation)

TO-DO (do we even want this)

## `LoidF`

Loid allows for pure functional programming.

And for some specific pieces of code, pure functions are highly preferred.

The `.loidf` subset of the Loid language is for entirely pure functional programming.

TO-DO (documentation)

## `NLoid`

Dialect of Loid for compiling Loid code to Nix code.

TO-DO (documentation)

## `JLoid`

Dialect of Loid for compiling Loid code cleanly and directly into JavaScript (CoffeeScript-like clarity of what the JavaScript will look like)
Isn't the same as the standard JavaScript target, but rather becomes a language that is directly compatible with JavaScript but with a slightly alternative syntax.

TO-DO (documentation)

## `CLoid`

Dialect of Loid that compiles to pure `C` code

TO-DO (documentation) (written WAY later, no need to ever actually care about this until SO MUCH LATER)

## `CSSoid`

CSS in Loid

TO-DO (also WAY later)

## `Lispoid`

Lisp-like Loid

TO-DO (interesting concept, might expand).

## `NecraLoid`

Warfarme inspired dialect of Loid that compiles to standard Loid

TO-DO (literally just some fun stuff that I'll write when I have nothing else to do, this is completely irrelevant)
