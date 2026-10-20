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

### For Loops

### Premature Breaking/Continuing and Labelling

### Switch Statements

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

## Shorthands

### The `with` keyword

### The `#let` `in` syntax
