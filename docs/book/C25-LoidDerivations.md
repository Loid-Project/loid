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

## `NLoid`

A purely-functional dialect of Loid for compiling Loid code to Nix code.

TO-DO (documentation)

## `QLoid`

Quantum Computing emulation in Loid.

Very interesting idea to build on.

TO-DO (when we are not working on the main language)
