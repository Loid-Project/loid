# Type System

Loid has a dependent-type system.

It relies on two foundational super-types: `atom` and `type`.

Types in Loid are first-class.

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
block cool_block {
    let n: int = 0;
    io.print(n + n + n);
}
```

## Structural & Object-Oriented Types

### Structs (`struct`)

A `struct` is Loid is an algebraic data structure similar to JavaScript objects and Rust structs.

Struct definitions:
```Rust
struct Person {
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
struct Person {
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
struct Person {
    name: string,
    age : int,
}

struct Employee derives Person {
    job : string,
}

/*
This is the same as:
*/

struct Employee derives Person {
    name: string,
    age : int,
    job : string,
}
```

Structs can derive multiple structs as well:

```Rust
struct Employee {
    name: string,
    age : int,
    job : string,
}

struct Boss {
    employee_count: int,
}

struct EmployedBoss {
    good_boss: bool,
}
```

This is the same as:

```Rust
struct EmployedBoss {
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
struct Person {
    name: string,
    age : int,
}

impl Person {
    fn age_up() -> int {
        this.age = this.age + 1;
        return this.age;
    }
}

struct Employee derives Person {
    job: string,
}
```

One can also override implementations

```Rust
struct Person {
    name: string,
    age : int,
}

impl Person {
    fn age_up() -> int {
        this.age = this.age + 1;
        return this.age;
    }
}

struct Employee derives Person {
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

These can be used for state machines and error handling.

Loid's `match` statement can be used to unwrap and handle enums easily.
The `match` statement works on structs and strings as well.
More on this later.


#### Enum Implementations

Enums can have implementations, just like structs.

Consider:
```Rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next_state() -> TrafficLight {
        return match this {
            Red    => TrafficLight.Green.
            Yellow => TrafficLight.Red,
            Green  => TrafficLight.Yellow,
        };
    }

    fn can_drive() -> bool {
        return match this {
            Green => true,
            _     => false, // the underscore is a commonly used keyword in Loid and is a catch-all.
            // more on the underscore in a later chapter
        };
    }
}

let light: TrafficLight = TrafficLight.Red;

io.print(light.can_drive().to_string()); // false
```

#### Payload Enum Implementations

When variants carry data payloads, the `impl` block acts as the unified API for extracting and operating on that data safely.

You must use the `match` keyword to unwrap the payload.

```Rust
enum NetworkResponse {
    Success { data: string },
    Error   { code: int, msg: string },
    Loading,
}

impl NetworkResponse {
    fn is_resolved() -> bool {
        return match this {
            Loading => false,
            _       => true,
        };
    }

    fn unwrap_or(fallback: string) -> string {
        return match this {
            Success { data } => data,
            Error { msg }    => "Failed with: " + msg,
            Loading          => fallback,
        };
    }
}
```

#### Mutating Enum State with Implementations

```Rust
enum Connection {
    Disconnected,
    Connecting,
    Connected { ip: string },
}

impl Connection {
    fn connect(target_ip: string) -> void {
        this = Connection.Connected { ip: target_ip };
    }

    fn disconnect() -> void {
        this = Connection.Disconnected;
    }
}

let conn: Connection = Connection.Disconnected;

conn.connect("192.168.1.1");
```

### Traits

Traits in Loid work the same as in Rust.

They are composable, reusable, implementation blocks, for all essential purposes.
These can be bolted onto structs.

Traits do NOT work with classes nor enums.

```Rust
trait Loggable {
    fn log_state() -> void {
        io.print("state logged as: " + time.now().to_string());
    }
}

struct Server {
    ip  : string,
    port: int,
}

impl Loggable for Server;

let s: Server = Server { ip: "127.0.0.1", port: 8080 };

s.log_state();
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

##### Overriding

TO-DO

#### Abstract Classes

Abstract classes are blueprints for concrete classes.
They cannot be directly instantiated and may contain `abstract` methods with signatures and no bodies.

As such, they are simply blueprints.

They work the same as in languages like TypeScript.

Consider:
```Rust
abstract class Animal {
    inst pub name: string;

    pub construct(name: string) -> this {
        this.name = name;

        return this;
    }

    abstract inst pub speak() -> string;
}
```

When a class inherits an abstract class, it must `override` the abstract methods to fulfill them.

Consider:
```Rust
class Cat inherits Animal {
    pub construct(name: string) -> this {
        super(name);

        return this;
    }

    override inst pub speak() -> string {
        return "Meow";
    }
}
```

#### Proxy Classes

Proxy classes are a common way to have security and encapsulation.
They are a common way to structure OOP programs.

In Loid, proxy classes have first-class support, as proxy classes are too vital to proper programmatic security.

Proxy classes wrap existing concrete classes and strictly dictate which fields and methods are exposed to the class it proxies for.

They also allow you to intercept, block, and rewrite method calls without altering the original object or having to implement complicated systems for catching cases.

Proxies use the `proxy`, `proxies`, and `overwrite` keywords.

Consider:

```Rust
class Database {
    inst pub url: string;

    pub construct(url: string) -> this {
        this.url = url;

        return this;
    }

    inst pub read() -> string {
        return "some sensitive data";
    }

    inst pub drop_tables() -> void {
        io.print("tables totally dropped");
    }
}

proxy ReadOnlyDB proxies Database {
    pub url: string; // exposes url as-is

    pub read: () -> string; // exposes read as-is

    overwrite pub drop_tables() -> void {
        io.print("security issue: no drop permissions");
    } // you can also just not have a drop_tables exposed if you want at all
}

let db: Database = Database.construct("localhost");

let safe_db: ReadonlyDB = ReadonlyDB.construct(db);

safe_db.drop_tables(); // prints the message instead of actually dropping tables
```

### Interfaces

Interfaces in OOP enforce behavioral contracts on classes.

It must implement every field that an interface defines.
Private, Public, or Protected.

Interfaces use the `implements` keyword.

```Rust
// by notation always capitalize and start with "I"
interface IConnectTable {
    pub connect: (string) -> bool;
    pub disconnect: () -> void;
}

interface ILoggable {
    pub log: () -> void;
}

class NetworkNode implements IConnectable, ILoggable {
    inst pub is_active: bool;

    pub construct() -> this {
        this.is_active = false;

        return this;
    }

    inst pub connect(ip: string) -> bool {
        this.is_active = true;

        return true;
    }

    inst pub disconnect() -> void {
        this.is_active = false;
    }

    inst pub log() -> void {
        io.print("node status: " + this.is_active.to_string());
    }
}
```

A class can implement multiple interfaces as long as they do not conflict.

#### Interface Inheritance

TO-DO
The `inherits` keyword

### Typestated Classes and Interfaces

Typestate-Oriented Programming guarantees that methods are called in the right order and work in the right methodology.

Typestating is defined at the `interface` level and implemented by a `typestated` class.

Consider:
```Rust
typestated interface IFileOps {
    pub file_name: string;
    pub construct: (string) -> this;

    pub open: () -> void;

    depends (open) {
        pub read_line: () -> string;
        pub close: () -> string;
    }
    // these methods cannot be called until `open` is executed.
    // an error here will be notified at comp-time and stopped
    // if the compiler cannot detect the error then it will optionally break
    // at runtime. This is behavior that must be specified by the programmer himself.
    // As such, using typestated classes should be done carefully and cleanly.
    // This is to allow the compiler to always find out how it'll execute.
    // More on this later

    // If there is a better way to deal with this behavior I'd appreciate help planning it
}

typestated class SafeFile implements IFileOps {
    inst pub file_name: string;

    pub construct(name: string) -> this {
        this.file_name = name;

        return this;
    }

    inst pub open() -> void {
        io.print("opened");
    }

    inst pub read_line() -> string {
        return "data";
    }

    inst pub close() -> void {
        io.print("closed");
    }
}

let file: SafeFile = SafeFile.construct("data.txt");
// file.read_line() // FAILURE (compile error)
file.open();
file.read_line(); // SUCCESS
```

### Typestated Structs

TO-DO (not depends based but rather a different struct per the exact chosen state)
(basically a tagged union wrapper i guess)

### Integrating Classes into the Type System

TO-DO
`abstract_class`, `proxy_class`, `concrete_class`, `child_class`
`typestated<a>` modifier
`children<a>`, `child_of<a>`, `interface<a>`, etc...
`uniq_type<a>` type too.

### Namespaces

Namespaces act as isolated environments to prevent global scope pollution.
Lots of words to say: a namespace like any other language.

Consider:
```TypeScript
namespace Networking {
    pub class Router { ... }
    pub fn ping(ip: string) -> bool { ... }
    priv const DEFAULT_PORT: int = 8080;
}
```

You use namespaces with `import` and `use` keywords in other files.

More on this at a later chapter.

## Functional Programming

Loid uses functional programming for many things.
Functions are first-class (just like types in Loid, more on this later).

Functions also support currying and partial application by default (more on this also later).

Function example:
```Rust
fn calculate_area(w: float, h: float) -> float {
    return w * h;
}
```

### Lambda functions

Lambda functions are written with the `lambda` keyword.
The `λ` symbol is an alias for the `lambda` keyword.

Consider:
```Rust
let square: (int) -> int = λ (x: int) -> int => x * x;

let complex_lambda: (int) -> int = λ (x: int) -> int {
    let y: int = 10;

    return x * y;
};
```

Functions naturally supported partial application with the `_` keyword.
And piping with the `|>` and `<|` operators.

### Function Types

When specifying a function as a type (for params or variables) Loid uses a familiar arrow notation.
Or a more generic `function<>` type.

Consider:
```Rust
fn process_number(n: int, formatter: (int) -> string) -> string {
    return formatter(n);
}
```

In Loid `(int, string) -> string` is the same as `function<[int, string], string>`
And `(int) -> string` is the same as `function<[int], string>`.

## Meta Programming

Loid offers some metaprogramming capabilities.
They are designed to be highly explicit and avoiding magic.

### Symbol Type

Symbols are unique identifiers and are optimized, immutable, and evaluated at compile time.

They are declared with a `\` prefix:

```TypeScript
symbol \mut;
symbol \read_only;
```

They can simply serve as flags.

A symbol called `\mut` is equivalent to any other symbol `\mut`.

## Advanced Types

Leveraging Martin-Löf Type Theory, Loid's type system can restrict values through the type system itself.

### Literal Types

A literal type restricts a variable to a predefined set of vales.

It's similar to a lightweight enum.

Consider:
```Rust
type Status = typing.literal<"SUCCESS", "FAILURE", "PENDING">;
type BinaryInt = typing.literal<0, 1>;

let current_status: Status = "SUCCESS"; // Valid
// let current_status: Status = "UNKNOWN"; // COMPILE ERROR
```

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

## As Keyword

TO-DO

## Satisfies Keyword

TO-DO

## First-Class Types

TO-DO

## Immutably Dependent Types

TO-DO

## Dynamically Dependent Types

TO-DO

## Runtime/Compile-time Testing

TO-DO
