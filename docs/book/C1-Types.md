# Type System

Loid has a dependent-type system.

It relies on two foundational super-types: `atom` and `type`.

## Atoms and the `atom` Type

Atoms are primitive values that carry no internal API.
Similar to primitives in other programming languages.

Atoms are the lowest-level memory structures in Loid.
They do not have methods or properties attached to them; all interactions occur through built-in standard library functions.

### Integers (`int`)

Integers are a base-10 signed 64-bit integer.
It operates in a strict range of $(-2^{53}+1,2^{53}-1)$.

To declare an integer one does:
```Rust
let n: int = 15;
let m: int = -100;
```

Underscores can be used for readability:
```Rust
let o: int = 1_000_000;
```

Integer operations always yield an integer.
Integer and `float` operations always yield a `float`.

Key functions for integers:
`add(x: int, y: int)`, `div(x: int, y: int)`, `mod(x: int, y: int)`, ...

#### Integer Bases

In Loid, one can define integers to be of any base, but they will be treated as base-10 unless otherwise specified.

Consider:

```Rust
let n: int = 2b1001001101;
```

This is an integer but it is in binary (base-2).

#### Big integers (`bigint`)

The `bigint` type refers to a 128-bit integer over a 64-bit integer.

```Rust
let b: bigint = 99999999999999999999999;
```

### Float & Double (`float` & `double`)

`float` and `double` represent all floating-point numbers.
They also include special literal states like `nan`, `infinity`, `neg_infinity`.

One can declare a float simply:

```Rust
let pi: float = 3.14159;
```

`nan` in Loid is unordered (not equal to itself) and represents the non-existence of a number.

### Bool (`bool`)

Represent boolean logic states of `true` and `false`.

```Rust
let is_active: bool = true;
let is_dead  : bool   = false;
```

### Character (`char`)

Represents a single UTF-8 unicode character.
Uses a Racket-style prefix of `#\`.

For example:
```Rust
let a: char = #\A;
```

### Void (`void`)

Represents nothing.
Used the same as other `void` types.

For example in functions:
```Rust
fn print_thing() -> void { ... }
```

### Imaginary and Complex Numbers (`imag` & `complex`)

The `imag` type represents imaginary numbers, floats, and integers, `complex` represents complex numbers of floats and imaginary numbers.

Consider:
```Rust
let n: imag = 15i;
let m: imag = 10.54;
let c: complex = 12 + 15i;
```

### Bits, Trits, and Qubits (`bit`, `trit`, `qbit`)

#### Bits and Trits

Atomic types `bit` and `trit` refer to:
- `bit`: 1 or 0
- `trit`: 0, 1, or 2

#### Qubits:

TO-DO

## Complex Types

Complex types in Loid are types that descend from the `type` type.

Unlike atoms, these can have complex structures and often have built-in methods rather than relying on the standard library.

### Strings (`string`)

One defines strings as follows:
```Rust
let s: string = "Hello";
let f: string = f"2 + 2 = \(2 + 2)"; // f-strings
let m: string = // multiline strings, by default are f-strings
    >Line one
    >Line two \(f)
```

#### Manipulation

There are many shorthands for manipulating strings in Loid.

```Rust
"hello" + " world"; // "hello world"
"hello" * 2; // "hellohello"
"hello" - "l"; // "heo"
```

### Regex

Loid supports regex expressions the same as JavaScript.

It has a builtin `regex` type:
```Rust
let pattern: regex = /[^0-9]/;
```

### Lists and Arrays (`list` and `array`)

Loid differentiates between dynamically sized lists and statically sized, dependently-typed arrays.

### Lists (`list<T>`)

Loid lists are dynamic, homogenous collections of items.

Consider:
```Rust
let xs: list<int> = [1, 2, 3, 4, 5];
let ys: int[] = [1, 2, 3]; // shorthand

let one_to_ten: int[] = list.to(1, 10);
let one_til_ten: int[] = list.til(1, 10);

io.print(xs.at(0).to_string());

xs.for_each(λ (x: int, i: int) -> void => io.print(x.to_string()));

xs.set(0, 99);
xs.push(6);
```

### Arrays

Loid arrays are fixed-size collections where the length `N` is a part of the type signature itself.
In Loid, this is called an *Immutably Dependent Type* (explained at the end of this chapter).

Passing an array of size 4 to a function expecting an array of size 3 will result in a compile-time error.

```Rust
let arr1: array<int, 3> = [1, 2, 3];
let arr2: int[3] = [1, 2, 3]; // shorthand

let bad_arr: int[2] = [1, 2, 3]; // this fails, as this is an array of length 3 expecting length 2
```

### Tuple

Tuples in Loid are fixed-size heterogenous collections of values.

They are strictly immutable.

Consider:
```Rust
let t : tuple<int, string> = tuple(1, "one");
let t2: [string, int]      = ["age", 25]; // shorthand for tuples
```

### Block

The `block` type represents any un-evaluated chunk of code.

Declaring `block`s:
```Rust
def block cool_block {
    let n: int = 0;
    io.print(n + n + n);
}
```

## Structural & Object-Oriented Types

### Structs (`struct`)

A `struct` is Loid is an algebraic data structure similar to JavaScript objects and Rust structs.

Struct definitions:
```Rust
def struct Person {
    name: string,
    age : int,
}

let person: Person = Person {
    name: "Logan",
    age : 20,
};
```

#### Struct Implementations

Implementations on structs allow you to attack an `impl` block onto a struct.

This gives a struct methods it can use on the objects in it.

```Rust
def struct Person {
    name: string,
    age : int,
}

impl Person {
    fn age_up() -> int {
        this.age = this.age + 1;
        return this.age;
    }
}

let person: Person = Person {
    name: "Larry",
    age : 20,
};

io.print(person.age); // 20

person.age_up();

io.print(person.age); // 21
```

#### Struct Derivations

Structs can derive one another:

```Rust
def struct Person {
    name: string,
    age : int,
}

def struct Employee derives Person {
    job : string,
}

/*
This is the same as:
*/

def struct Employee derives Person {
    name: string,
    age : int,
    job : string,
}
```

Structs can derive multiple structs as well:

```Rust
def struct Employee {
    name: string,
    age : int,
    job : string,
}

def struct Boss {
    employee_count: int,
}

def struct EmployedBoss {
    good_boss: bool,
}
```

This is the same as:

```Rust
def struct EmployedBoss {
    name          : string,
    age           : int,
    job           : string,
    employee_count: int,
    good_boss     : bool,
}
```

#### Deriving Struct Implementations

Struct implementations can also be derived

```Rust
def struct Person {
    name: string,
    age : int,
}

impl Person {
    fn age_up() -> int {
        this.age = this.age + 1;
        return this.age;
    }
}

def struct Employee derives Person {
    job: string,
}
```

One can also override implementations

```Rust
def struct Person {
    name: string,
    age : int,
}

impl Person {
    fn age_up() -> int {
        this.age = this.age + 1;
        return this.age;
    }
}

def struct Employee derives Person {
    job: string,
}

impl Employee {
    override fn age_up() -> {
        this.age = this.age + 2; // you age more when employed
    }
}

```

### Enums

Loid enums work similar to Rust enums.

One can define enums with payloads, or just as unions.

```Rust
enum Element {
    Earth,
    Water,
    Air,
    Fire,
}

// with payload:
enum Result<T, E> {
    Ok : T,
    Err: E,
}
```

Enums can also have Implementations on them.
TO-DO.

These can be used for state machines and error handling.

Loid's `match` statement can be used to unwrap and handle enums easily.
The `match` statement works on structs and strings as well.
More on this later.

### Traits

Traits in Loid work the same as in Rust.

They are composable, reusable, implementation blocks, for all essential purposes.
These can be bolted onto structs or enums.

```Rust

```

### Classes

There are two kinds of classes in Loid.
Concrete Classes and Abstract Classes.

#### Concrete Classes

Concrete Classes in Loid work like standard classes in any other programming language.

Consider:

```Rust
class Vehicle {
    inst pub vehicle_type: string;

    pub construct(vehicle_type: string) -> this {
        this.vehicle_type = vehicle_type;

        return this;
    }
}
```

To make fields or methods public you use the `pub` keyword.
For private use the `priv` keyword.
For protected use the `prot` keyword.

Instance fields and methods are declared with the `inst` keywords and statics with the `static` keyword.

##### Concrete Class Inheritance

Inheritance in concrete classes works similarly to other languages.

In Loid, the keyword is `inherits`, as `extends` is kept for another use case.

Consider:

```Rust
class Vehicle {
    inst pub vehicle_type: string;

    pub construct(vehicle_type: string) -> this {
        this.vehicle_type = vehicle_type;

        return this;
    }
}

class Car inherits Vehicle {
    inst pub company: string;

    pub construct(vehicle_type: string, company: string) -> this {
        super(vehicle_type);

        this.company = company;

        return this;
    }
}
```

#### Abstract Classes

TO-DO `abstract`, `override`

#### Proxy Classes

TO-DO `proxy_class` type and `proxies` keyword

### Interfaces

TO-DO
`implements keyword`, examples of interfaces.

TO-DO

### Typestated Classes and Interfaces

TO-DO

### Integrating Classes into the Type System

TO-DO
`abstract_class`, `proxy_class`, `concrete_class`, `child_class`
`typestated<a>` modifier
`children<a>`, `child_of<a>`, `interface<a>`, etc...

### Namespaces

TO-DO

## Functional Programming

TO-DO

### Functions

TO-DO

### Function Types

TO-DO

## Meta Programming

TO-DO

### Symbol Type

TO-DO

### Operator Type

TO-DO

### Blocks in More Depth

## Advanced Types

TO-DO

### Literal Types

TO-DO

### Conditional Types

TO-DO

### Union Types

TO-DO

### The `unknown` type

TO-DO

### The `never` type

TO-DO

## Built-in and Monadic Types

TO-DO

### Option, Maybe, Either, Result

TO-DO

#### Option

TO-DO

#### Maybe

TO-DO

#### Either

TO-DO

#### Result

TO-DO

### Sets

TO-DO

### Promises

TO-DO
`when`, `await` keywords

## Immutably Dependent Types

TO-DO

## Dynamically Dependent Types

TO-DO

## Runtime/Compile-time Testing

TO-DO
