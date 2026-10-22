# Basic Syntax

## Variables

In Loid, you declare variables with one of the following keywords: `var`, `let`, `const`, or `global`.

### `var`

Variables declared with `var` are variables and can be changed.
Consider:

```TypeScript
var n: int = 1;
n = n + 1;
```

### `let`

Variables declared with `let` are constants and cannot have their value be changed.
However, they can hold references to mutable values.

```Rust
let n: int = 1;
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
fn f(n: int, m: int) -> int {
    return n + m;
}
```

### Partial Application

All functions in loid are partially applied.

Consider the function `f`:

```Rust
fn f(n: int, m: int, l: int, g: int) -> int {
    return n + m + l + g;
}
```

This function can be applied in various ways:

```Rust
let f1: (int, int) -> int = f(1, 2, _, _); // now it takes in the l and g parameters.
let f2: (int, int) -> int = f(1, _, _, 2); // now it takes in the m and l parameters.
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

For loops in Loid are essentially always for-each loops.

Consider:

```Rust
for (let i: int; i ### [1, 2, 3, 4, 5]) {
    io.print(i);
}
```

(the `###` is a placeholder until a good syntax to replace it is decided upon)
(it is yet to be decided to allow for standard for loops where you declare an iterator variable)

### Premature Breaking/Continuing and Labelling

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

TO-DO

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

```Rust
#repeat n {
    io.print("hello");
}
```

### Deferring and the `defer` Keyword

In Loid, one can defer a line or block of code to run at the end of the current block of code.

Consider:

```Odin
fn some_bad_function(db: Database) -> void {
    db.open();
    defer db.close();

    // some db manipulations

    // at the end of this code block, the database will close
}
```

This can be done to blocks of code as well.

Consider:

```Odin
fn some_bad_function(db: Database) -> void {
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
fn (example) -> void {
    #defer 10 {
        io.print("first");
    }
    #defer 9 {
        io.print("second");
    }
}
```

## Shorthands

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
result = #let { let x: int = 10;
                let y: int = 15; }
                in x + y; // this is 25.
```

The block defined will simply create a namespace.

There is a shorthand as well:

TO-DO (decide on shorthand notation)

## The `directive` `#` syntax

TO-DO (basically as in Odin)

## Decorators

TO-DO

### @pure

TO-DO (enforces functional purity onto a block, this example will only cover how to use this on functions)

### @cached

TO-DO (memoizes functions, only able to be used on function blocks not on any block)

### More

TO-DO

## The Semicolon

The semicolon in Loid isn't required.
However, highly encouraged as it makes it clearer where things start and end.

If the language does not find a semicolon, it'll know to go to the next line.

Revolutionary, I know.
