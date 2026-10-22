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
for (let i : int; i ### [1, 2, 3, 4, 5]) {
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

TO-DO (used for repeating a certain block of code a certain integer (literal) number of times)

### The `#repeat` keyword

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

TO-DO (you can define which defer-layer happens first by giving it an integer value with the `#defer` keyword. Larger values go first).

## Shorthands

### The `with` keyword

TO-DO (same as in Nix, it resolves a namespace)
(it can also implicitly turn a struct object into a namespace of its keys and values, but does not work for implementations upon said struct)

```Typescript
with some_namespace {
    // block of code doing stuff
};
```

### The `#let` `in` syntax

TO-DO (standard `let` `in` syntax in Nix, but uses a `directive` instead)

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
