# Functions

Functions in Loid are first-class and work like functions in JavaScript.

## Function Syntax

### Basics

Generally functions in Loid are declared as follows:

```Rust
fn magical(x: Int, y: Int) -> Int {
    return x + y;
}
```

While lambdas are declared as follows:

```Rust
let func        : (Int, Int) -> Int = lambda (x: Int, y: Int) -> Int => x + y;
let func_cooler : (Int, Int) -> Int = λ (x: Int, y: Int) -> Int => x + y;
```

### Variadic Function Parameters

TO-DO (disallowed, but will allow for a type-system thing to define something similar)

## Function Types

TO-DO

## Function Signatures

TO-DO

## Decorators

TO-DO
(split decorators that have actual changes and effects from decorators who just ensure of things?)
(maybe place the decorators who ensure of things as effects as described in Types?)

Loid provides decorators for functions to allow for more complex kinds of computing.

### Memoization and Caching

Consider the function `fib` that computes the `Fibonacci` sequence.

```Rust
fn fib(n: Int) -> Int {
    return match n {
        0 | 1 => n,
        _     => fib(n - 1) + fib(n - 2),
    }
}
```

Given:

```Rust
@cached
fn fib(n: Int) -> Int { ... }
```

Every input output will be cached, regardless of the function's purity.
This stores the output information in a hash table, for efficient lookup and fast returns.

```Rust
@memoized
fn fib(n: Int) -> Int { ... }
```

`@memoized` is the same thing as `@cached`, just an alias.

### Lazy Evaluation

Loid generally does not do lazy evaluation, unless required.

Consider:

```Rust
@lazy
fn fib(n: Int) -> Int { ... }
```

Now, when doing this:

```Rust
let fib3: Int = fib(3);
```

This will not be evaluated until the variable `fib3` itself is used somewhere.
Or whatever value it may be pointing at gets used.
Essentially enforces storing of a lazy object.

#### Eager Evaluation

Consider a more complex function that is lazy:

```Rust
@lazy
fn mystery(n: Int, c: SomeClass) -> Int { ... }
```

Let's say that created an instance of `SomeClass` is expensive.

When it comes to the standard Lazy decorator, this will still be evaluated before the function is called.

Consider:

```Rust
let mysterious: Int = mystery(5, SomeClass.construct());
```

This will still create the `SomeClass` instance before `mysterious` is every used.

This is because Loid uses eager evaluation, where it evaluates all arguments always before the function is called.

This behaviour however, can be avoided by making something `@totally_lazy`.

```Rust
@totally_lazy
fn mystery(n: Int, c: SomeClass) -> Int { ... }
```

You can also do this in-line instead.

Say, for every call to `mystery`, only in one do you want `SomeClass` itself to be lazy, you can do this inline as follows:

```Rust
@lazy
fn mystery(n: Int, c: SomeClass) -> Int { ... }
let mysterious: Int = mystery(5, SomeClass.construct@(@lazy)());
```

This makes the actual construct method itself lazy.
You can use multiple decorators there as well:

```Rust
let mysterious: Int = mystery(5, SomeClass.construct@(@lazy, @other_decorator)());
```

You can also make mystery itself `@totally_lazy` even if it is just `@lazy`:

```Rust
let mysterious: Int = mystery@(@totally_lazy)(5, SomeClass.construct(@lazy)());
```

### Aliasing as Lazy for Clarity's Sake

TO-Do

### Functional Purity

You can enforce functional purity in Loid functions through the `@pure` tag.
This also applies to code blocks of any kind.

```Rust
@pure
fn fib(n: Int) -> Int { ... }
```

If the Loid compiler sees impurities in this code, it will not allow for compilation.

### Using Multiple Decorators

Multiple decorators can be used easily in Loid.

Consider:

```Rust
@pure, @memoized
fn fib(n: Int) -> Int { ... }
```

## Pure Functions

TO-DO

## Functional Programming

TO-DO
