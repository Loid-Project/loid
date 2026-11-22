# Type System

Loid has a dependent-type system.

It relies on two foundational super-types: `Atom` and `Type`.
They are both children of the `Unknown` type by formality.

Types in Loid are first-class.

## Atoms and the `Atom` Type

Atoms are primitive values that carry no internal API.
Similar to primitives in other programming languages.

Atoms are the lowest-level memory structures in Loid.
They do not have methods or properties attached to them; all interactions occur through built-in standard library functions.

### Integers (`Int`)

Integers are a base-10 signed 64-bit integer.
It operates in a strict range of $(-2^{53}+1,2^{53}-1)$.

To declare an integer one does:

```Rust
let n: Int = 15;
let m: Int = -100;
```

Underscores can be used for readability:

```Rust
let o: Int = 1_000_000;
```

Integer operations always yield an integer.
Integer and `Float` operations always yield a `Float`.

Key functions for integers:
`add(x: Int, y: Int)`, `div(x: Int, y: Int)`, `mod(x: Int, y: Int)`, ...

#### Integer Bases

In Loid, one can define integers to be of any base, but they will be treated as base-10 unless otherwise specified.

Consider:

```Rust
let n: Int = 2b1001001101;
```

This is an integer but it is in binary (base-2).

#### Big integers (`BigInt`)

The `BigInt` type refers to a 128-bit integer over a 64-bit integer.

```Rust
let b: BigInt = 99999999999999999999999;
```

### Float & Double (`Float` & `Double`)

`Float` and `Double` represent all floating-point numbers.
They also include special literal states like `NaN`, `Infty`, `NegInfty`.

One can declare a float simply:

```Rust
let pi: Float = 3.14159;
```

`nan` in Loid is unordered (not equal to itself) and represents the non-existence of a number.

### Bool (`Bool`)

Represent boolean logic states of `true` and `false`.

```Rust
let is_active: Bool = true;
let is_dead  : Bool   = false;
```

### Character (`Char`)

Represents a single UTF-8 unicode character.
To create a character, you place it in `''`.

For example:

```Rust
let a: Char = 'A';
```

### Void (`Void`)

Represents nothing.
Used the same as other `Void` types.

For example in functions:

```Rust
fn print_thing() -> Void { ... }
```

### Imaginary and Complex Numbers (`Imag` & `Complex`)

The `imag` type represents imaginary numbers, floats, and integers, `Complex` represents complex numbers of floats and imaginary numbers.

Consider:

```Rust
let n: Imag = 15i;
let m: Imag = 10.54;
let c: Complex = 12 + 15i;
```

Essentially:
Any `Imag` is also `Complex`
Any `Int` is also `Imag`

But not vice-versa

### Bits, Trits, and Qubits (`Bit`, `tTit`, `QBit`)

#### Bits and Trits

Atomic types `Bit` and `Trit` refer to:

-   `Bit`: 1 or 0
-   `Trit`: 0, 1, or 2

TO-DO (needs more information)

#### Qubits:

Note, to write quantum code, you must use the `QLoid` derivative.
To understand what that means refer to the Loid Derivatives chapter.

Loid targets JavaScript, as such, it isn't capable of doing quantum computation.
However, a simulated (and one day real) quantum bit type is built into Loid.
It is the `QBit` type.

Qubits are declared with the `Q` prefix:

Consider:

```TypeScript
let q0: QBit = Q|0>;
let q1: QBit = Q|1>;
let qo: QBit = Q|+>:
let qm: QBit = Q|->;
```

Qubits also have registers with the `QReg` constructor type:

```Rust
let rwg: QReg<3> = QReg(0, 0, 0);
```

Gates are ordinary functions in the standard library, most gates are supported and their APIs can be found at later chapters of this book.

Measurements return classical bits and mutate the registers.

```Rust
let outcome: Bit = Q.measure(reg, 0);
```

## Complex Types

Complex types in Loid are types that descend from the `type` type.

Unlike atoms, these can have complex structures and often have built-in methods rather than relying on the standard library.

### Strings (`String`)

One defines strings as follows:

```Rust
let s: String = "Hello";
let f: String = f"2 + 2 = \(2 + 2)"; // f-strings
let m: String = // multiline strings, by default are f-strings
    >Line one
    >Line two \(f)
```

#### Manipulation

There are many shorthands for manipulating strings in Loid.

```Rust
"hello" + " world"; // "hello world"
"hello" * 2;        // "hellohello"
"hello" - "l";      // "heo"
```

### Regex

Loid supports regex expressions from its standard library.
Which includes the `Regex` type.

More on this in the standard library chapter.

### Lists and Arrays (`List` and `Array`)

Loid differentiates between dynamically sized lists and statically sized, dependently-typed arrays.

Explained right down below:

### Lists (`List<T>`)

Loid lists are dynamic, homogenous collections of items.

Consider:

```Rust
let xs: List<Int> = [1, 2, 3, 4, 5];
let ys: Int[] = [1, 2, 3]; // shorthand

let one_to_ten: Int[]  = List.to(1, 10);
let one_til_ten: Int[] = List.til(1, 10);

io.print(xs.at(0).to_string());

xs.for_each(λ (x: Int, i: Int) -> void => io.print(x.to_string()));

xs.set(0, 99);
xs.push(6);
```

### Arrays

Loid arrays are fixed-size collections where the length `N` is a part of the type signature itself.
In Loid, this is called an _Immutably Dependent Type_ (explained at the end of this chapter).

Passing an array of size 4 to a function expecting an array of size 3 will result in a compile-time error.

```Rust
let arr1: Array<Int, 3> = [1, 2, 3];
let arr2: Int[3] = [1, 2, 3]; // shorthand

let bad_arr: Int[2] = [1, 2, 3]; // this fails, as this is an array of length 3 expecting length 2
```

### Tuple

Tuples in Loid are fixed-size heterogenous collections of values.

They are strictly immutable.

Consider:

```Rust
let t : Tuple<Int, String> = tuple(1, "one");
```

### Block

The `Block` type represents any un-evaluated chunk of code.

Declaring `Block`s:

```Rust
let cool_block: Block = {
    let n: int = 0;

    io.print(n + n + n);
}
```

More on blocks will be explained in the metaprogramming section.

## Structural & Object-Oriented Types

### Structs (`struct`)

A `struct` is Loid is an algebraic data structure similar to JavaScript objects and Rust structs.

Struct definitions:

```Rust
struct Person {
    name: String,
    age : Int,
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
    name: String,
    age : Int,
}

impl Person {
    fn age_up() -> Int {
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
    name: String,
    age : Int,
}

struct Employee derives Person {
    job : String,
}

/*
This is the same as:
*/

struct Employee derives Person {
    name: String,
    age : Int,
    job : String,
}
```

Structs can derive multiple structs as well:

```Rust
struct Employee {
    name: String,
    age : Int,
    job : String,
}

struct Boss {
    employee_count: Int,
}

struct EmployedBoss {
    good_boss: Bool,
}
```

This is the same as:

```Rust
struct EmployedBoss {
    name           : String,
    age            : Int,
    job            : String,
    employee_count : Int,
    good_boss      : Bool,
}
```

#### Deriving Struct Implementations

Struct implementations can also be derived

```Rust
struct Person {
    name : String,
    age  : Int,
}

impl Person {
    fn age_up() -> Int {
        this.age = this.age + 1;
        return this.age;
    }
}

struct Employee derives Person {
    job: String,
}
```

One can also override implementations

```Rust
struct Person {
    name : String,
    age  : Int,
}

impl Person {
    fn age_up() -> Int {
        this.age = this.age + 1;
        return this.age;
    }
}

struct Employee derives Person {
    job: String,
}

impl Employee {
    override fn age_up() -> {
        this.age = this.age + 2; // you age more when employed
    }
}

```

#### Generics in Structs

TO-DO

#### Interactions of Generics with Inheritance

TO-DO

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
enum Result<T, E> { // this is how generics are written in Enums
    Ok  : T,
    Err : E,
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
            TrafficLight.Red    => TrafficLight.Green.
            TrafficLight.Yellow => TrafficLight.Red,
            TrafficLight.Green  => TrafficLight.Yellow,
        };
    }

    fn can_drive() -> Bool {
        return match this {
            TrafficLight.Green => true,
            _                  => false, // the underscore is a commonly used keyword in Loid and is a catch-all.
            // more on the underscore as a catch-all in a later chapter
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
    fn is_resolved() -> Bool {
        return match this {
            NetworkResponse.Loading => false,
            _                       => true,
        };
    }

    fn unwrap_or(fallback: String) -> String {
        return match this {
            NetworkResponse.Success { data } => data,
            NetworkResponse.Error   { msg }  => "Failed with: " + msg,
            NetworkResponse.Loading          => fallback,
        };
    }
}
```

#### Mutating Enum State with Implementations

```Rust
enum Connection {
    Disconnected,
    Connecting,
    Connected: { ip: String },
}

impl Connection {
    fn connect(target_ip: String) -> Void {
        this = Connection.Connected({ ip: target_ip });
    }

    fn disconnect() -> Void {
        this = Connection.Disconnected;
    }
}

let conn: Connection = Connection.Disconnected;

conn.connect("192.168.1.1");
```

#### Setting Enum Payloads

When creating an instance of an enum with a payload one can follow this kind of syntax:

```Rust
enum Connection {
    Disconnected,
    Connecting,
    Connected: String,
}

let conn: Connection = Connection.Connected("192.168.1.1");
```

#### Getting Enum Payloads

To get the payload from an Enum, you can simply access the value without calling it with `()`.
Keep in mind, enum types with payloads behave as if they were typestated, as such, it is important to either clarify to the compiler that it is at the right typestating point, or keep code pure to allow the compiler to catch this for itself.

Consider:

```Rust
enum Connection {
    Disconnected,
    Connecting,
    Connected: string,
}

let conn: Connection = Connection.Connected("192.168.1.1");

io.print(conn.Connected);
```

### Traits

Traits in Loid work the same as in Rust.

They are composable, reusable, implementation blocks, for all essential purposes.
These can be bolted onto structs.

Traits do NOT work with classes nor enums.

```Rust
trait Loggable {
    fn log_state() -> Void {
        io.print("state logged as: " + time.now().to_string());
    }
}

struct Server {
    ip  : String,
    port: Int,
}

impl Loggable for Server;

let s: Server = Server { ip: "127.0.0.1", port: 8080 };

s.log_state();
```

#### Implementing Multiple Traits

You can implement multiple traits onto a struct.

Consider a struct called `Cat` and traits `Walk`, `Play`, `Sleep`.

Then:

```Rust
impl Walk, Play, Sleep for Cat;
```

This'll implement these traits.

#### Trait Inheritance

Consider the following:

```Rust
trait Drawable {
    fn draw() -> Void { ... };
}

trait Resizable {
    fn resize(w: Int, h: Int) -> Void { ... };
}

trait UIElement inherits Drawable, Resizable {
    fn focus() -> Void { ... };
    fn blur()  -> Void { ... };
}

// these methods can also call their super methods

trait Animated inherits Drawable {
    fn redraw() -> Void {
        this.draw();
    }
}
```

Now the trait `UIElement` has the methods from `Drawable` and `Resizable`.

#### Overriding Trait Methods

When inheriting a method, you can override it.

Consider:

```Rust
trait Drawable {
    fn draw() -> Void { ... };
}

trait Animated inherits Drawable {
    override fn draw() -> Void {
        // something else
    }
}
```

Without the `override` keyword, the compilation will fail.

#### Overriding Traits

Consider a struct called `Button` and consider:

```Rust
impl UIElement for Button;
```

This gives us whatever the trait functions already had.

We can override these by writing this instead:

```Rust
impl UIElement for Button {
    override fn draw() -> Void {
        io.print("drawing");
    }
};
```

#### Handling Conflicts

Consider a struct `Cat` that implements `Walk, Play, Sleep`, and say both `Play` and `Walk` have functions named `meow`.

If two traits have a conflict in their namespace and are being both implemented into a struct, the compiler will fail.

However, this can be handled as follows:

```Rust
impl Walk, Play, Sleep for Cat {
    override fn meow() -> Void {
        Walk.meow(); // if you want to use the Walk version
        Play.meow(); // for the Play version
        // or write something of your own
    }
}
```

#### Interaction with Struct Derivations

When overriden Traits (or traits in general) interact with derived structs, this is the behaviour:

```Rust
trait Loggable {
    fn log_state() -> Void {
        io.print("log");
    }
}

struct Person {
    name: string,
}

impl Loggable for Person;

struct Employee derives Person {
    job: String.
}

let e: Employee = Employee { name: "Ada", job: "Computer Scientist" };
e.log_state(); // prints Person's default
```

So, a struct that derives another struct inherits the parent's trait implementation by default.

To override on a derived struct:

```Rust
impl Employee {
    override fn log_state() -> void {
        io.print(this.name + " (" + this.job + ")");
    }
}
```

However, it is important to note that:
Reimplementing an inherited trait (say `impl Loggable for Employee`) after derivation is always a compile error.
If Employee derives from two parents that both implement `Loggable`, the rule applies: the compiler requires an explicit override, unless they both implement Loggable in the same exact way.

#### Abstract Traits

Abstract traits are traits that don't have the full implementation.
As in:

```Rust
trait AnimalLike {
    abstract fn make_sound() -> Void;
    abstract fn get_named(s: String) -> Bool;
}
```

When implementing these, they have to be overriden:

```Rust
struct Cat {
    name: string,
}

impl AnimalLike for Cat {
    override fn make_sound() -> Void {
        io.print("meow");
    }

    override fn get_named(s: String) -> Bool {
        this.name = s;
        return true;
    }
}
```

Code failing to do this fails to compile.

#### Generics in Traits

Traits may be parameterized over types,just like structs and enums.

Consider:

```Rust
trait Container<T> {
    fn get(index: Int) -> T { ... };
    fn set(index: Int, value: T) -> Void { ... };
    fn len() -> Int { ... };
}
```

A generic trait is implemented on a generic struct by carrying the type parameter through the `impl` block.

```Rust
struct Stack<T> {
    items: List<T>,
}

impl<T> Container<T> for Stack<T>;
```

A trait may also introduce a type parameter that is not present on the struct, for example, to describe a conversion.

These are however less common and it is preferred to define the generic on the method itself rather than the trait.

As follows:

```Rust
trait ConvertsToInteger {
    abstract fn convert<T>(t: T) -> Int;
}
```

#### Interactions of Generics in Trait Inheritance

A generic trait can inherit from one or more generic traits.
Type parameters flow through the inheritance chain, and the child may fix, reorder, or extend the parent's parameters.

Consider passing parameters through unchanged:

```Rust
trait Iterable<T> {
    abstract fn next() -> T;
}

trait Collection<T> inherits Iterable<T> {
    abstract fn size() -> Int;
}
```

Parameters must be passed down.

##### Fixing Parameters

A child may bind a parent's parameter to a concrete type or to one of its own parameters:

```Rust
trait Readable<T> {
    fn read() -> T;
}

trait Writeable<T> {
    fn write(value: T) -> Void;
}

trait ReadWriteable<T> inherits Readable<T>, Writeable<T> {
    fn flush() ->Void;
}

// Here K and V are the child's parameters,
// and the parents are instantiated with those same parameters
trait KeyValue<K, B> inherits Readable<Option<V>>, Writeable<Tuple<K, V>> {
    fn contains_key(key: K) -> Bool;
}

// a trait that is specifically string-keyed
trait StringKeyword inherits KeyValue<String, V> {
    // StringKeyed<V> is a KeyValue<string, V>;
    // K is pinned to a string
}
```

##### Adding Parameters

A child may add new parameters that the parent never saw, provided that they are independent of the parent's:

Consider:

```Rust
trait Encoder<T> {
    fn encode(value: T) -> String;
}

trait CompressingEncoder<T, C> inherits Encoder<T> {
    fn compression_level() -> C;
}
```

##### Overriding Generic Methods

A child trait may override an inherited method, but the override must be signature-compatible module the trait's parameters.
Covariant return types are permitted, but the same `override` rules apply as for non-generic traits.

Consider:

```Rust
trait Stringify<T> {
    fn stringify(value: T) -> String {
        return value.to_string();
    }
}

trait PrettyStringify<T> inherits Stringify<T> {
    override fn stringify(value: T) -> String {
        return "<" + value.to_string() + ">";
    }
}
```

### Classes

There are two kinds of classes in Loid.
Concrete Classes and Abstract Classes.

#### Concrete Classes

Concrete Classes in Loid work like standard classes in any other programming language.

Consider:

```Coffee
class Vehicle {
    inst pub vehicle_type: String;

    pub construct(vehicle_type: String) -> TypeOf<this> {
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

```Coffee
class Vehicle {
    inst pub vehicle_type: String;

    pub construct(vehicle_type: String) -> TypeOf<this> {
        this.vehicle_type = vehicle_type;

        return this;
    }
}

class Car inherits Vehicle {
    inst pub company: String;

    pub construct(vehicle_type: String, company: String) -> TypeOf<this> {
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
    inst pub fn describe() -> String {
        return "a vehicle";
    }
}

class Car inherits Vehicle {
    override inst pub fn describe() -> String {
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

```Java
class Vehicle {
    final inst pub fn describe() -> String {
        return "a vehicle";
    }
}

class Car inherits Vehicle {
    // this fails to compile
    override inst pub fn describe() -> String {
        return "a car";
    }
}
```

##### Class Derivations and Mixins

TO-DO (use `mixes` keyword, and maybe `derives` keyword).

#### Abstract Classes

Abstract classes are blueprints for concrete classes.
They cannot be directly instantiated and may contain `abstract` methods with signatures and no bodies.

As such, they are simply blueprints.

They work the same as in languages like TypeScript.

Consider:

```Coffee
abstract class Animal {
    inst pub name: String;

    pub construct(name: String) -> TypeOf<t>his {
        this.name = name;

        return this;
    }

    abstract inst pub speak() -> String;
}
```

When a class inherits an abstract class, it must `override` the abstract methods to fulfill them.

Consider:

```Coffee
class Cat inherits Animal {
    pub construct(name: String) -> TypeOf<this> {
        super(name);

        return this;
    }

    override inst pub speak() -> String {
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

```Java
class Database {
    inst pub url: String;

    pub construct(url: String) -> TypeOf<this> {
        this.url = url;

        return this;
    }

    inst pub read() -> String {
        return "some sensitive data";
    }

    inst pub drop_tables() -> Void {
        io.print("tables totally dropped");
    }
}

proxy ReadOnlyDB proxies Database {
    pub url: String; // exposes url as-is

    pub read: () -> String; // exposes read as-is

    overwrite pub drop_tables() -> Void {
        io.print("security issue: no drop permissions");
    } // you can also just not have a drop_tables exposed if you want at all
}

let db: Database = Database.construct("localhost");

let safe_db: ReadonlyDB = ReadonlyDB.construct(db);

safe_db.drop_tables(); // prints the message instead of actually dropping tables
```

#### Children of Proxy Classes

Proxy classes can themselves be inherited, but every descendant of a proxy class must also be a proxy class.
The compiler enforces this.

Consider:

```Rust
proxy ReadOnlyDB proxies Database {
    pub url: String;
    pub read: () -> String;

    overwrite pub drop_tables() -> Void {
        io.print("security issue: no drop perms");
    }
}

proxy AuditedDB proxies Database inherits ReadOnlyDB {
    pub url: String;
    pub read: () -> String;

    overwrite pub drop_tables() -> Void {
        io.print("audio log: drop_tables was blocked");
    }
}
```

If a child declares `proxies <T>` explicitly, the type must match the proxied target (`Database` in this example above).
Otherwise the compiler will report a mismatch.
The `proxies` clause may be omitted on the child, in which case the target is inherited from the parent chain:

```Rust
proxy AuditDB inherits ReadOnlyDB { ... }
```

The exposure rules are cumulative down the chain.
A child exposes at least what its parent exposes, and may add further members of the wrapped class.
It may also `overwrite` any inherited members, which replaces that member's behaviour from the child (and its own descendants).

#### Interactions of Proxy Classes with `ConcreteClass<A>` / `ProxyClassOf<A>`

Because a proxy is technically a concrete class, any proxy may be passed where a `ConcreteClass<Database>` is expected.
`ProxyClassOf<A>` accepts any instance of any proxy chain targeting `A`.
`ProxyClass<A>` accepts the class itself (see the reflection types part of this chapter).
The compiler computes this set of admissible proxies from the inheritance chain of the proxy classes.

### Interfaces

Interfaces in OOP enforce behavioral contracts on classes.

It must implement every field that an interface defines.
Private, Public, or Protected.

Interfaces use the `implements` keyword.

By notation always capitalize and start with "I"

```Coffee
interface IConnectTable {
    pub connect: (string) -> Bool;
    pub disconnect: () -> Void;
}

interface ILoggable {
    pub log: () -> Void;
}

class NetworkNode implements IConnectable, ILoggable {
    inst pub is_active: Bool;

    pub construct() -> TypeOf<this> {
        this.is_active = false;

        return this;
    }

    inst pub connect(ip: String) -> Bool {
        this.is_active = true;

        return true;
    }

    inst pub disconnect() -> Void {
        this.is_active = false;
    }

    inst pub log() -> Void {
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

```Coffee
interface IReader {
    pub read: () -> String;
}

interface IWriter {
    pub write: (String) -> Void;
}

interface IReadWriter inherits IReader, IWriter {
    pub flush: () -> Void;
}
```

So, `IReadWriter` also enforced `read` and `write` alongside `flush`.

#### Default Interface Methods

Interfaces can have default methods.
These default methods can access any fields that are enforced by the interface.

Consider:

```Coffee
interface ILoggable {
    default pub log: () -> Void = λ => io.print("logging");
}

interface IMetric {
    pub record: ((Float) -> Void);
}

class Sensor implements ILoggable, IMetric {
    inst pub fn record(v: Float) -> Void {
        io.print(v);
    }
}

let s: Sensor = Sensor.construct();

s.log()
```

#### Inheritance of Interfaces with Default Methods

Inheritance them works the same as for other types of interfaces.
However, the default methods can be overridden by the `override` keyword.

Also, non-default methods can be made default methods the same way.

TO-DO (examples of how these'd work)

An interface inheritance carries default methods down to child interfaces exactly as it carries abstract signatures.
A child interface inherits every default method from its parents, and those defaults are available to any class that implements the child interface.

Consider the following code:

```Coffee
interface ILoggable {
    default pub log: () -> Void = λ () => io.print("logging");
}

interface IIdentifiable inherits ILoggable {
    pub id: () -> String;
}

class User implements Identifiable {
    inst pub id() -> String {
        return "user-42";
    }
}

let u: User = User.construct();

u.log();
u.id();
```

You can also override default methods like this:

```Coffee
interface ILoggable {
    default pub log: () -> Void = λ () => io.print("log");
}

interface IVerboseLoggable inherits ILoggable {
    override default pub log: () -> Void = λ () => io.print("VERBOSE: log");
}

class Service implements IVerboseLoggable { ... }

let s: Service = Service.construct();
s.log();
```

##### Promoting Abstract Methods to Default Methods

A child interface m ay supply a default body for a signature that a parent left abstract.

Consider:

```Coffee
interface IReader {
    pub read: () -> String;
}

interface IFileReader inherits IReader {
    pub open: (path: String) -> Void;

    default pub read: () -> String = λ () => "default content";
}
```

##### Handling Conflicts

When two parents both contribute a default method with the same name,t he child must resolve the conflict or get a compile error.

Consider:

```Coffee
interface ILoggable {
    default pub log: () -> Void = λ () => io.print("loggable");
}

interface IDebuggable {
    default pub log: () -> Void = λ () => io.print("debuggable");
}

interface ICombined inherits ILoggable, IDebuggable {
    override default pub log: () -> Void = λ () => {
        ILoggable.log();
        IDebuggable.log();
    }
}
```

#### Visibility Rules for Default Interface Methods when Parenting Interfaces

Visibility on an inherited default may only widen, never narrow:

`priv` -> `prot` -> `pub` is allowed.
Anything else is a compile error.
Of course this allows for `priv` -> `pub` too.

This must be done with the override keyword.

Consider:

```Coffee
interface IBase {
    default prot log: () -> Void = λ () => io.print("base");
}

interface IDerived inherits IBase {
    override default pub log: () -> Void = λ () => io.print("derived");
}
```

#### Interfaces with Proxy Classes

In Loid, interfaces cannot be made over proxy classes, and proxy classes cannot implement interfaces.

This is done for the sake of simplicity.
As you can CLEARLY tell, Loid is a very simple language with not many keywords.
(This is sarcasm, Loid is actually a very large language)

### Typestated Classes and Interfaces

Typestate-Oriented Programming guarantees that methods are called in the right order and work in the right methodology.

Typestating is defined at the `interface` level and implemented by a `typestated` class.

Consider:

```Rust
typestated interface IFileOps {
    pub file_name: String;
    pub construct: (String) -> TypeOf<this>;

    pub open: () -> Void;

    depends (open) {
        pub read_line: () -> String;
        pub close: () -> String;
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
    inst pub file_name: String;

    pub construct(name: String) -> TypeOf<this> {
        this.file_name = name;

        return this;
    }

    inst pub open() -> Void {
        io.print("opened");
    }

    inst pub read_line() -> String {
        return "data";
    }

    inst pub close() -> Void {
        io.print("closed");
    }
}

let file: SafeFile = SafeFile.construct("data.txt");
// file.read_line() // FAILURE (compile error)
file.open();
file.read_line(); // SUCCESS
```

#### Typestated Object Inheritance

TO-DO (how inheritance works on typestated classes and interfaces, also for interfaces with default values)

#### Typestated Abstract Classes

TO-DO (decide whether to allow typestating in abstract classes)
(decide on how to allow that in a clean manner)

#### Typestated Proxies

TO-DO

### Typestated Structs

Typestate on structs works via type parameters rather than depends blocks.
A struct carries its state as a type argument, and `impl` blocks are specialized per state.

It relies on generics.

Consider:

```Rust
struct File<S> {
    pub path: String,
}

impl File<Closed> {
    pub fn open(path: String) -> File<Open> {
        return File<Open> { path: path };
    }
}

impl File<Open> {
    pub fn read_line() -> String {
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

`AbstractClass<A>` - the type of an abstract class named `A`. Values of this type may be subclasses of `A` but not instantiated.
Say in `AbstractClass<Animal>` the `Animal` is an abstract class.
If a type of a variable is `AbstractClass<A>` it can be assigned a value of `A` (which is the abstract class itself), or value of `B` if `B` is a child of the abstract class.

If a type of a variable is `A` and `A` is an abstract class, it can be assigned any child of said abstract class.

`ConcreteClass<A>` - the type of a concrete class `A`.
If a type of a variable is `ConcreteClass<A>` it can be assigned a value of `A` (which is the concrete class itself), or a value of `B` if `B` is a child of the abstract class.

To clarify, since Loid is a language with first-class types, the value of the said variable (or parameter, or constant, or whatever), is actually just the type itself.

`ProxyClassOf<A>` - the type of a proxy class wrapping `A`. But not a type that takes in a type.
To be clearer:

-   A variable of type `ProxyClassOf<A>` can hold a value of an instance of a proxy class of `A`
-   A variable of type `ProxyClass<A>` can hold a value of any proxy class of `A`, the class itself, not an instantiated object of it.

Recall that proxy classes are technically concrete classes, as such, any proxy class can also be treated as a concrete class.

TO-DO (explain how types'll work with proxy class children)

`ChildClass<A>` - the type of a class that inherits from `A`.
This is to say:

-   A variable of type `ChildClass<A>` can hold a value of a child class of `A`, the class itself, not an instantiated object of it.

TO-DO (parent classes for the type system)

TO-DO (how interfaces, singletons, and typestated classes, and typestated interfaces, including their inheritance fits into this)

#### Type-Level Utility Functions for Reflection Types upon Classes

TO-DO

### Namespaces

Namespaces act as isolated environments to prevent global scope pollution.
Lots of words to say: a namespace like any other language.

Consider:

```Rust
namespace Networking {
    pub class Router { ... }
    pub fn ping(ip: String) -> Bool { ... }
    priv const DEFAULT_PORT: Int = 8080;
}
```

You use namespaces with `import` and `use` keywords in other files.

More on this at a later chapter.

#### Namespace Transformations

TO-DO (how to make things like structs and similar objects, into namespaces (especially important for resolutions with the `with` keyword))

## Functional Programming

Loid uses functional programming for many things.
Functions are first-class (just like types in Loid, more on this later).

Functions also support currying and partial application by default (more on this also later).

Function example:

```Rust
fn calculate_area(w: Float, h: Float) -> Float {
    return w * h;
}
```

### Lambda functions

Lambda functions are written with the `lambda` keyword.
The `λ` symbol is an alias for the `lambda` keyword.

Consider:

```Rust
let square: (Int) -> Int = λ (x: Int) -> Int => x * x;

let complex_lambda: (Int) -> Int = λ (x: Int) -> Int {
    let y: Int = 10;

    return x * y;
};
```

Functions naturally supported partial application with the `_` keyword.
And piping with the `|>` and `<|` operators.

### Function Types

When specifying a function as a type (for params or variables) Loid uses a familiar arrow notation.

Or a more generic `Function<T[], U>` type.

Consider:

```Rust
fn process_number(n: Int, formatter: (Int) -> String) -> String {
    return formatter(n);
}
```

In Loid `(Int, String) -> String` is the same as `Function<[Int, String], String>`
And `(Int) -> String` is the same as `Function<[Int], String>`.

### Defining the Function Type in Loid Type Language

TO-DO

### Partial Application of Functions

TO-Do

### Function Piping

To-DO

## The `TypeOf` Keyword

TO-DO

## Meta Programming

Loid offers some metaprogramming capabilities.
They are designed to be highly explicit and avoiding magic.

This section from the type system will only go into types that are useful for metaprogramming.
However, the chapter on metaprogramming will go into far more depth and detail on how the loid language actually works.

This is regarding the `Block` type, macros, and so on.

## Advanced Types

Leveraging Martin-Löf Type Theory, Loid's type system can restrict values through their types.

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

#### Runtime Literal Types

TO-DO (has to allow due to the nature of side-effects being unabstracted to their fullest)

### The `Never` type

The `Never` type is a bottom type.

I.E.:

```Rust
Never <: T <: Unknown
```

The `never` type can only be used as a return type for a function that terminates.

```Rust
fn panic(msg: String) -> Never {
    io.print("fatal: " + msg);
    sys.exit(1);
}
```

#### Never Behaviour Quirks

TO-DO

## Built-in and Monadic Types

Loid omits the infamous programming concept of `null`. For obvious reasons.

Instead of missing values with the `null` type, Loid relies on monadic types.

These types enforce that the developer handles both the "success" and "failure" tracks explicitly.

### Option, Maybe, Either, Result

#### Option

The `Option<T>` type represents a value that might be present.

It has two variants: `Some` and `None`.

Consider:

```Rust
struct User {
    username: String,
    email   : Option<String>,
}

let u: User = User {
    username: "LoidFan69",
    email   : Option.Some("fan@proton.me"),
}

match u.email {
    Option.Some { e } => io.print("sending to: " + e);
    Option.None       => io.print("who is you");
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
let l: Either<Int, String> = Either.Left(42);
let r: Either<Int, String> = Either.Right("hi");
```

TO-DO (still need to properly decide on syntax for payload initializing and access)

Combinators:

```Rust
impl Either<L, R> {
    pub fn is_left() ->  Bool;
    pub fn is_right() -> Bool;

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

TO-DO (write a basic explanation)
Ideas:
`Result` is a wrapper around `Either`.
More on the `?` operator, `Result` type (and the general monadic error handling system) in the Error Handling chapter.

### Sets

Loid has both a runtime collection type for sets and a type-level set representation.

#### Runtime Sets

TO-DO (add more useful functions to the API)

These are instances of the `Set` class, which stores an unique list of elements:

This is the Set API:

```Rust
class Set<T> {
    priv fn construct() -> Set<T>;

    pub fn new() -> Set<T>;
    pub fn of(items: List<T>) -> Set<T>;

    pub fn add(item: T)    ->    Bool;        // true if newly inserted
    pub fn remove(item: T) -> Bool;
    pub fn has(item: T)    -> Bool;

    pub fn union(other: Set<T>) -> Set<T>;
    pub fn intersect(other: Set<T>) -> Set<T>;
    pub fn difference(other: Set<T>) -> Set<T>;
    pub fn symmetric_difference(other: Set<T>) -> Set<T>;

    pub fn is_subset(other: Set<T>)   -> Bool;
    pub fn is_superset(other: Set<T>) -> Bool;

    pub fn size() -> Int;
    pub fn to_list() -> List<T>;

    pub fn for_each<F: (T) -> Void>(f: F) -> Void;
}
```

These sets are unordered, deduplicated, and support O(1) membership.

TO-DO (examples)

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

#### Infinite Type-Level Sets

TO-DO

### Promises

Loid's asynchronous model is Promise-based, mirroring JavaScript but with full type level tracking.

#### Async Functions

Consider:

```Coffee
async fn fetch_user(id: Int) -> Promise<User> {
    let res = await http.get("/users/" + (Int.to_string <| id>));
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

For the purposes of metaprogramming, one can define an `AsyncBlock` type block.

These blocks are a built-in type that extend the `Block` type.
These can work in metaprogramming terms as well.

```Rust
let some_block: AsyncBlock = async {
    ...
}
```

These can be worked with and ran same as other blocks, but these will simply be asynchronous.

#### After Keyword

TO-DO
Waits for multiple jobs and then runs that block of code

```Odin
async { ... } after async { ... }
```

#### When Keyword

TO-DO
Runs a background task (daemon) in the program.

#### Assign After Keyword

TO-DO (requires being in an async block)

```Rust
let [a, b, c] = assign after [
    fetch_user(1),
    fetch_user(2),
    fetch_user(3),
];
```

#### Assign Before Keyword

TO-Do (requires being in an async block)

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

## Constrained Types

TO-DO

### Natural Number Type

TO-DO

#### Proving Naturality

TO-DO

#### Handling Unnaturality

TO-DO

### Theorem Proving for Constraints

TO-DO

#### Induction

TO-DO

#### Rational Number Type

TO-DO

#### Irrational Number Type

TO-DO

#### Issue with the Irrational Number Type

TO-DO (floating point values are always finite, so it'd have to be not a computed number but rather a more general statement? Like say sqrt2 instead of whatever that's gonna compute to.)

## First-Class Types

TO-DO

### The `Type<T>` Wrapper

TO-DO

### Reflections

TO-DO (`TypeRep`? Is this even a good idea?)

### `Type.of`

TO-DO (?)

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

### Type Functions

TO-DO

## Immutably Dependent Types

TO-DO

### The `Ref` and `Snap` types

TO-DO (maybe something to do with the Call-By-Push-Value Paradigm?)

### Arrays

TO-DO

### Dependent Operations

TO-DO

### Matrices

TO-DO

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

## Type Inference

TO-DO (small explanation, will be explained more in a later chapter)

### Declaration Inference

TO-DO

### `?` Inference

TO-DO (very small explanation, elaborated upon in another chapter)

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

### Runtime Behaviour

TO-DO

## Conditional Types

TO-DO (conditions applied to any type, rather than constraints, which simply narrows down a certain type to a smaller subtype, conditional types are a generalization of that (works with say `string | int` types too, for an example, constraints are just an example of an use case of conditional types, uses the same syntax)).

### Distribution Over Unions

TO-DO

### Non-Distributive Conditionals

TO-DO

### The `infer` Keyword

To-DO

## Logical Types

TO-DO

### Boolean Logic on Types

TO-DO

### Loopy and Recursive Types

TO-DO

### Where to Use These

TO-DO

## Theorem Proving

TO-DO (more in-depth)

### Natural Numbers

TO-DO

### Irrational Number Type

TO-DO

#### Example: Proving Irrationality to the Compiler

TO-DO

### Types as Statements

TO-DO

### Induction

TO-DO

## Runtime/Compile-time Testing

TO-DO

### Runtime Assertions Through `assert`

TO-DO

### `comptime assert`

TO-DO

### `test` blocks

TO-DO

### Stubbing

TO-DO

### Mocking

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

The `Unknown` type is the top type.

Everything in Loid technically descends from the `Unknown` type.

Anything can be assigned to a variable of `Unknown` type.
However, no operations can be done on an `Unknown` variable.
Nor can it be passed into a function, or done anything with.

The `Unknown` type can either be a return value for a function, or an instantiating type for a variable.

The `Unknown` variable can be cast or pattern matched to another type, and then used (in-line).

This exists mostly for the reasons of formality and is generally advised against for usage unless you have a deep understanding of what you are doing.

Any `Unknown`s will get a compiler warning.

## Type Language

TO-DO (basic rundown, finish in another chapter)

### Loid Derivatives

TO-DO

### TLoid

TO-DO

## Foreign Types

TO-DO (basic rundown/intro, finish in another chapter)

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
