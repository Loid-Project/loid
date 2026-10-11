# Type System

Loid has a dependent-type system.

It relies on two foundational super-types: `atom` and `type`.
They are both children of the `unknown` type by formality.

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

Loid targets JavaScript, as such, it isn't capable of doing quantum computation.
However, a simulated (and one day real) quantum bit type is built into Loid.
It is the `qbit` type.

Qubits are declared with the `q` prefix:

Consider:

```Rust
let q0: qbit = q|0>;
let q1: qbit = q|1>;
let qo: qbit = q|+>:
let qm: qbit = q|->;
```

Qubits also have registers with the `qreg` constructor type:

```Rust
let rwg: qreg<3> = qreg(0, 0, 0);
```

Gates are ordinary functions in the standard library, most gates are supported and their APIs can be found at later chapters of this book.

Measurements return classical bits and mutate the registers.

```Rust
let outcome: bit = q.measure(reg, 0);
```

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
    Success: { data: string },
    Error  : { code: int, msg: string },
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
    Connected: { ip: string },
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

When a concrete class inherits another concrete class, its methods may override the parent's.

Loid requires explicit override on any method that shadows an inherited signature.

Consider:

```Rust
class Vehicle {
    inst pub fn describe() -> string {
        return "a vehicle";
    }
}

class Car inherits Vehicle {
    override inst pub fn describe() -> string {
        return "a car";
    }
}
```

Overridden methods must always be marked by the `override` keyword.

A subclass may return a subtype of parent's return type as covariance.
So: `Vehicle.describe -> Vehicle` can become `Car.describe -> Car`.
This is sound, as `Car` is a `Vehicle`.

Visibility may only widen, but cannot be narrowed:
So: `priv -> prot -> pub` is fine, but `pub -> prot -> priv` is not.

`abstract` methods (explained next) must be overridden.

As Loid compiles to JavaScript, static methods can be overridden if you so desire.

##### Final Methods

Methods can be marked by final on concrete classes.
This means that they cannot be overridden in their children.
Consider:

```Rust
class Vehicle {
    final inst pub fn describe() -> string {
        return "a vehicle";
    }
}

class Car inherits Vehicle {
    // this fails to compile
    override inst pub fn describe() -> string {
        return "a car";
    }
}
```

##### Class Derivations and Mixins

TO-DO, use `mixes` keyword, and maybe `derives` keyword.

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

An interface may inherit from one or more interfaces.
Absorbing their methods and contracts (as long as there are no conflicts).
Conflicts in visibility, types, and so on will cause compilation errors.

Consider:

```Rust
interface IReader {
    pub read: () -> string;
}

interface IWriter {
    pub write: (string) -> void;
}

interface IReadWriter inherits IReader, IWriter {
    pub flush: () -> void;
}
```

So, `IReadWriter` also enforced `read` and `write` alongside `flush`.


#### Default Interface Methods

Interfaces can have default methods.
These default methods can access any fields that are enforced by the interface.

Consider:
```Rust
interface ILoggable {
    default pub log: () -> void = λ => io.print("logging");
}

interface IMetric {
    pub record: (float -> void);
}

class Sensor implements ILoggable, IMetric {
    inst pub fn record(v: float) -> void {
        io.print(v);
    }
}

let s: Sensor = Sensor.construct();

s.log() // "logging"
```

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

Typestate on structs works via type parameters rather than depends blocks.
A struct carries its state as a type argument, and `impl` blocks are specialized per state.

Consider:

```Rust
struct File<S> {
    pub path: string,
}

impl File<Closed> {
    pub fn open(path: string) -> File<Open> {
        return File<Open> { path: path };
    }
}

impl File<Open> {
    pub fn read_line() -> string {
        return "data";
    }

    pub fn close() -> File<Closed> {
        return File<Closed> { path: this.path };
    }
}
```

Calling methods on a wrong state will result in a compile error.

An useful type function built into Loid is `Typestated<A, S>`:
`Typestated<A, S>` - the typestated struct `A` in state `S`.
Say: `Typestated<File, Open>`.


### Integrating Classes into the Type System

Because Loid's types are first-class and classes lower to JS classes with attached `__typerep` data, the type system exposes reflection over classes through a set of useful utility types and comptime type functions.

#### Reflection Types

`AbstractClass<A>` - the type of an abstract class named `A`. Values of this type may be subclassed but not instantiated.
Say in `AbstractClass<Animal>` the `Animal` is an abstract class.

`ConcreteClass<A>` - the type of a concrete class `A`.

`ProxyClassOf<A>` - the type of a proxy class wrapping `A`.

`ChildClass<A>` - the type of a class that inherits from `A`.

#### Type-Level Utility Functions

These are not apart of the base language, but rather exist in the standard library's `typing` module.
Many of these however, are hardcoded into the language rather than constructed from Loid's type system.

`Children<A>` - compile-time list of all classes that inherit from the class `A`.
`ChildOf<A, B>` - boolean function that returns true if `A` is a child of `B`.
`Interface<A>` - the interface portion of `A`. The `Interface<A>` is a struct of the union of all interfaces that the class inherits.

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

This section from the type system will only go into types that are useful for metaprogramming.
However, the chapter on metaprogramming will go into far more depth and detail on how the loid language actually works.

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

### The `never` type

The `never` type is a bottom type.

I.E.:
```
never <: T  <: unknown
```

The `never` type can only be used as a return type for a function that terminates.

```Rust
fn panic(msg: string) -> never {
    io.print("fatal: " + msg);
    sys.exit(1);
}
```

## Built-in and Monadic Types

Loid omits the infamous programming concept of `null`.

Instead of missing values with the `null` type, Loid relies on monadic types.

These types enforce that the developer handles both the "success" and "failure" tracks explicitly.

### Option, Maybe, Either, Result

#### Option

The `Option<T>` type represents a value that might be present.

It has two variants: `Some` and `None`.

Consider:

```Rust
struct User {
    username: string,
    email   : Option<string>,
}

let u: User = User {
    username: "LoidFan69",
    email   : Option.Some("fan@proton.me"),
}

match u.email {
    Some { e } => io.print("sending to: " + e);
    None       => io.print("who is you");
}
```

#### Maybe

`Maybe` is an alias of `Option`.

#### Either

The `Either<L, R>` monad is a two-variant sum enum type used for when a type is either one thing or another.

Consider:
```Rust
enum Either<L, R> {
    Left: L,
    Right: R,
}
```

Constructors:
```Rust
let l: Either<int, string> = Either.Left(42);
let r: Either<int, string> = Either.Right("hi");
```

Combinators:
```Rust
impl Either<L, R> {
    pub fn is_left() -> bool;
    pub fn is_right() -> bool;

    pub fn left() -> Option<L>;
    pub fn right() -> Option<R>;

    pub fn unwrap_left() -> L;
    pub fn unwrap_right() -> R;

    pub fn map<R2, F: (R) -> R2>(f: F) -> Either<L, R2>;
    pub fn map_left<L2, F: (L) -> L2>(f: F) -> Either<L2, R>;

    pub fn swap() -> Either<R, L>;

    pub fn fold<T, FL: (L) -> T, FR: (R) -> T>(fl: FL, fr: FR) -> T;
}
```

#### Result

`Result` is a wrapper around `Either`.

More on the `?` operator, `Result` type (and the general monadic error handling system) in the Error Handling chapter.

### Sets

Loid has both a runtime collection type for sets and a type-level set representation.

#### Runtime Sets

These are instances of the `Set` class, which stores an unique list of elements:

This is the Set API:

```Rust
class Set<T> {
    pub fn new() -> Set<T>;
    pub fn of(items: list<T>) -> Set<T>;

    pub fn add(item: T) -> bool;        // true if newly inserted
    pub fn remove(item: T) -> bool;
    pub fn has(item: T) -> bool;

    pub fn union(other: Set<T>) -> Set<T>;
    pub fn intersect(other: Set<T>) -> Set<T>;
    pub fn difference(other: Set<T>) -> Set<T>;
    pub fn symmetric_difference(other: Set<T>) -> Set<T>;

    pub fn is_subset(other: Set<T>) -> bool;
    pub fn is_superset(other: Set<T>) -> bool;

    pub fn size() -> int;
    pub fn to_list() -> list<T>;

    pub fn for_each<F: (T) -> void>(f: F) -> void;
}
```

These sets are unordered, deduplicated, and support O(1) membership.

#### Type-Level Sets

This is a type-level set used for keys and unions of literals.

Consider:
```TypeScript
type K = set<"a" | "b" | "c">;

// You can add elements to this type set:
// and do various other operations

type L = K.add<"d">;          // { "a", "b", "c", "d" }
type M = K.remove<"a">;       // { "b", "c" }
type N = K.union<set<"d">>;    // { "a", "b", "c", "d" }
type O = K.intersect<set<"b" | "x">>; // { "b" }
```

Type-level sets such as this must be finite.
The use cases for these will be explained later on.

This is also a good time to mention that in Loid, the type system itself is purely functional.
The type system does not allow for side effects, despite it serving as a almost a full programming language of its own, in some regards.
This is to allow for stronger and safer types.

### Promises

Loid's asynchronous model is Promise-based, mirroring JavaScript but with full type level tracking.

#### Async Functions

Consider:
```Rust
async fn fetch_user(id: int) -> Promise<User> {
    let res = await http.get("/users/" + (int.to_string <| id>));
    return User.from_json(res.body);
}
```

These work the same way as in TypeScript.

#### Await

The `await` keyword works the same as in JavaScript.

It waits until an asynchronous function is done doing its calculations.

Consider:
```TypeScript
await fetch_user(1);
```

The code will simply wait here until the user is fetched.
The `await` keyword only works in asynchronous functions or blocks of code.

#### Async Blocks

For the purposes of metaprogramming, one can define an `async` type block.

```Rust
let some_block: AsyncBlock = async {
    ...
}
```

These can be worked with and ran same as other blocks, but these will simply be asynchronous.

#### After Keyword

TO-DO
Waits for multiple jobs and then runs that block of code
```Rust
async { ... } after async { ... }
```

#### When Keyword

TO-DO
Runs a background task (daemon) in the program.

#### Assign After Keyword

TO-DO
```Rust
let [a, b, c] = assign after [
    fetch_user(1),
    fetch_user(2),
    fetch_user(3),
];
```

#### Promise<T> Combinators and API

TO-DO

#### Deferred Lazy Execution in Async Blocks via Future<T>

TO-DO

#### Promise Cancellation

TO-DO

## As Keyword

TO-DO

### Checked `as` Conversion

TO-DO

### Unchecked `as!` Conversion

TO-DO

### Pattern Matching as Narrowing

TO-DO

### Numeric Conversions

TO-DO

### String Conversions

TO-DO

## Satisfies Keyword

TO-DO

### Exhaustiveness

TO-DO

### Runtime Behavior

TO-DO, shouldn't be any, but just in case there might be any.

## First-Class Types

TO-DO

### The `Type<T>` Wrapper

TO-DO

### Reflections

TO-DO
`TypeRep`.

### `Type.of`

TO-DO

### Passing Types to Functions

TO-DO

### Returning Types from Functions

TO-DO

### Comptime Types vs Runtime Types

TO-DO

#### Type Identity

TO-DO

#### Type Limitations

TO-DO

## Immutably Dependent Types

TO-DO

### Arrays

TO-DO

### Dependent Operations

TO-DO
```TypeScript
fn head<T, N: int>(xs: array<T, N+1>) -> T {
    return xs[0];
}
```

### Matrices

TO-Do

### Compile-Time Arithmetic

TO-DO

### Limits

TO-DO

## Union Types

TO-DO

### Basic Unions

TO-DO

### Discriminated Unions

TO-DO

### Non-Discriminated Unions

TO-DO

### `keyof` and `valueof`

TO-DO

### Union Algebra

TO-DO

### Narrowing Rules

TO-DO

## Dynamically Dependent Types

TO-DO

### Refined Runtime Values

TO-DO

### Smart Constructors

TO-DO

### Flow Narrowing

TO-DO

### Limits

TO-DO

### Dynamic Dependency Use Cases

TO-DO

## Conditional Types

TO-DO

### Distribution Over Unions

TO-DO

### Non-Distributive Conditionals

TO-DO

### `infer`

To-DO

## Logical Types

### Boolean Logic on Types

TO-DO

### Loopy and Recursive Types

TO-DO

### Where to Use These

TO-DO

## Runtime/Compile-time Testing

TO-DO

### Runtime Assertions Through `assert`

TO-DO

### `comptime assert`

TO-DO

### `test` blocks

TO-DO

### `comptime test` blocks

TO-DO

### Type-Level Tests

TO-DO

### Property-Based Testing

TO-DO

### Fuzzing

TO-DO

### Test Attributes

TO-DO

### Coverage

TO-DO

## The Unknown Type

The `unknown` type is the top type.

Everything in Loid technically descends from the `unknown` type.

Anything can be assigned to a variable of `unknown` type.
However, no operations can be done on an `unknown` variable.
Nor can it be passed into a function, or done anything with.

The `unknown` type can either be a return value for a function, or an instantiating type for a variable.

The `unknown` variable can be cast or pattern matched to another type, and then used (in-line).

This exists mostly for the reasons of formality and is generally advised against for usage unless you have a deep understanding of what you are doing.

Any `unknown`s will get a compiler warning.

## Foreign Types

TO-DO

### The `foreign` Keyword

TO-DO

### Foreign Functions

TO-DO

### Foreign Modules

TO-DO

### Safety

TO-DO

### Notes

TO-DO
