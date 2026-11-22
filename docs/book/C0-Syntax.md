# Basic Syntax

## Variables

In Loid, you declare variables with one of the following keywords: `var`, `let`, `const`, or `global`.

### `var`

Variables declared with `var` are variables and can be changed.
Consider:

```TypeScript
var n: Int = 1;
n = n + 1;
```

### `let`

Variables declared with `let` are constants and cannot have their value be changed.
However, they can hold references to mutable values.

```Rust
let n: Int = 1;
n = n + 1; // compiler error
```

### `const`

Variables declared with `consts` are called constants and can only hold values that can be evaluated at compile time.
Thus, either literals, or calls to pure functions, or references to immutable objects.

More on this later.

### `global`

Variables declared with `global` go in the globals list.
These are always constants and follow the same rules as `const` variables.
More on these later.

## Functions

In Loid to declare functions one generally follows this example:

```Rust
fn f(n: Int, m: Int) -> Int {
    return n + m;
}
```

### Partial Application

All functions in loid are partially applied.

Consider the function `f`:

```Rust
fn f(n: Int, m: Int, l: Int, g: Int) -> Int {
    return n + m + l + g;
}
```

This function can be applied in various ways:

```Rust
let f1: (Int, Int) -> Int = f(1, 2, _, _); // now it takes in the l and g parameters.
let f2: (Int, Int) -> Int = f(1, _, _, 2); // now it takes in the m and l parameters.
```

### Lambda Functions

Lambda functions in Loid work as follows:

```Odin
let f: (Int, Int) -> Int = lambda (x: Int, y: Int) -> Int { return x + y; };
```

Or:

```Odin
let f: (Int, Int) -> Int = lambda (x: Int, y: Int) -> Int => x + y;
```

## Control Flow

### If Statements

`if` statements in Loid work the same as in other programming languages.

Consider:

```Rust
if (true) {
    // do something
}
```

#### If/Else

```Rust
if (true) {
    // do something
} else {
    // do something
}
```

#### If/ElseIf

```Rust
if (true) {
    // do something
} else if (true) {
    // do something
}
```

#### If/ElseIf/Else

```Rust
if (true) {
    // do something
} else if (true) {
    // do something else
} else {
    // fallthrough case
}
```

### While Loops

While loops in Loid work like any other language.

Consider:

```Rust
while (true) {
    // do something
}
```

### For Loops

Standard for loops work in Loid as in most other languages.

```Rust
for (let i: Int = 0; i < 5; i++) {
    io.print(i);
}
```

### For Each Loops

For loops can be for-each loops.

Consider:

```Odin
#for (let i; [1, 2, 3, 4, 5]) {
    io.print(i);
}
```

### Premature Breaking/Continuing and Labelling

TO-DO

#### While Loops

In Loid, you can break out or continue a while loop with `break` and `continue` keywords.

However, Loid understands scoping.

You can tag while loops with strings. This is used to give them an ID.

Consider:

```Odin
#while "outer" (true) {
    #while "inner" (true) {
        break "outer";
    }
}
```

Note: to use this feature, you need to use the `#while` keyword instead.

This means that the outer while loop'll break.

If there is no string given, it'll simply break out of the last loop you're in.

Consider:

```Odin
#while "outer" (true) {
    #while "inner" (true) {
        break;
        // same as typing: break "inner";
    }
}
```

This'll only break out of the inner loop.

Or for standard while loops:

```Rust
while (true) {
    while (true) {
        break;
    }
}
```

Just breaks out of the inner loop again.

#### For Loops

TO-DO

#### Hybrids

TO-DO

### Switch Statements

There are no `switch` statements in standard Loid.
All of the said functionality is built into the pattern matching system with the `match` keyword.
More on that in the chapter on pattern matching.

However, they do exist in `JavaScript` and in `JLoid`, as such, in `Loid` identifiers cannot be named `switch`.

### The `repeat` keyword

The `repeat` keyword repeats a block of code a fixed number of times given by an integer literal (or by a value determined at compile time):

```Rust
repeat 3 {
    io.print("hello");
}
```

### The `#repeat` keyword

The `#repeat` is a directive form and allows for an expression to determine the count itself:

It'll try to determine at comptime, but if it doesn't, it just does so at runtime.

```Odin
#repeat n {
    io.print("hello");
}
```

This is less safe, and generally not recommended for production.

### Deferring and the `defer` Keyword

In Loid, one can defer a line or block of code to run at the end of the current block of code.

Consider:

```Odin
fn some_bad_function(db: Database) -> Void {
    db.open();
    defer db.close();

    // some db manipulations

    // at the end of this code block, the database will close
}
```

This can be done to blocks of code as well.

Consider:

```Odin
fn some_bad_function(db: Database) -> Void {
    db.open();
    defer {
        db.close();
        io.print("closed the database");
    }

    // some db manipulations

    // at the end of this code block, the database will close
}
```

#### Defer Identities

You can assign priority to deferred blocks with the `#defer` directive.

The standard `defer` always has the lowest priority.

Consider:

```Odin
fn (example) -> Void {
    #defer 2 {
        io.print("first");
    }
    #defer 1 { // runs before the defer above it
        io.print("second");
    }
}
```

These defer keyword are harder to predict, and as such generally not recommended for production.

## Shorthands

TO-DO (shorthands isn't a good name, give it a better name)

### The `with` keyword

The `with` keyword brings contents of a namespace or a struct (that is implicitly to be transformed into a namespace) into the current scope.

Essentially resolving it, similar to Nix.

Consider:

```TypeScript
with some_namespace {
    do_something(); // say do_something was apart of some_namespace
}
some_namespace.do_something();

let p: Person = Person { name: "Alice", age: 30 };
with p {
    io.print(name); // prints "Alice"
}
```

### The `#let` `in` syntax

The `#let`...`in` expression binds names locally and evaluates a body.

Consider:

```Odin
let result: Int = #let { let x: Int = 10;
                         let y: Int = 15; }
                         in x + y; // this is 25.
```

The block defined will simply create a namespace.

There is a shorthand as well:

TO-DO (decide on shorthand notation)

## The `where` Keyword.

The `where` keyword is a syntactic sugar over the `let` and `in` syntax.

Consider:

```Rust
let n: Int = x + y where { let x: Int = 10; let y: Int = 11; };
```

## The Ternary Operator.

Consider:

```Odin
let n: Int = if (true) do 10 // doesn't require the ";" if using do and expecting an else or else if.
             else do 15;
let m: Int = if (true) {
                10;
             } else {
                10;
             }
```

## The `do` Keyword

The `do` keyword in Loid has two main uses.
Allowing for one-liners.
And running code blocks.

### One-liner Statements

Consider:

```Rust
while (true) do io.print("sleepy");

if (true) do io.print("hungry");
else if (true) do io.print("hungry more");
else do io.print("not hungry");
```

### Running Code Blocks

Consider:

```Rust
let b: Block = {
    io.print("play!");
};

do b; // prints "play!"

// or:

do {
    io.print("stop playing!");
} // prints "stop playing!"
```

### Both?

You can do both actually, technically:

```Rust
if (true) do {
    io.print("hungry");
}
```

Just no reason to.
Unless you're evil.

## The `directive` `#` syntax

TO-DO (basically as in Odin)

## Decorators

These are explored in more depth in <a href="./C2-Functions.md">Chapter 2</a>.

## The Semicolon

The semicolon in Loid is required at the end of almost everything outside of the end of code blocks (`{ ... }`).
