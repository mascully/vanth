# Vanth

Currently this a not-very-organized collection of ideas I have for a new programming language. My goal is to make a data-oriented language for configuration, build scripts, and reproducible development environments.

Inspired by ideas from Rust, Typescript, Zig, D, Unison, Nickel, and Nix.


## Example

```va
use vanth::v1;

interface MyInterface {
	foo: String;
	optional_foo = "default";
}

Module {
	Module.description = "An example expression";
}
```


## Expressions

Everything in Vanth is an expression.

`use` and `interface` declarations have the same effect whether they are inside an object expression or preceding it.


## Types

Types are values, like in Zig. All values are fully immutable.


### Primitive types

* `String`
* `Char`
* `Byte`
* `Path`
* `Bool`
* `Int`
* `Nat`
* `Float`
* `Type`

Objects are `Object` and are not primitives.

`Int` is always 32-bit 2's complement encoding. `Nat` is unsigned. `Float` is always IEEE 754 32-bit single precision. `Nat` is a subtype of `Int`, i.e. anywhere that accepts `Int` can also take a `Nat`. It is always an error when either of these types reaches a magnitude above or equal to 2<sup>31</sup>. Note that this means that `Nat` effectively only has 31 bits, not 32.

`String` is any valid UTF-8 string.

`Char` is a 32-bit Unicode codepoint.

`Byte` is an octet.

Arrays are `[T]` or `[T; len]` like Rust. There are also Typescript-like tuples, `[Int, Int, String]` etc.

Use `&` for type intersections, e.g. `Module & Module.Shell`. Use `|` for type unions, e.g. `Os.Windows | Os.Linux`.


### Literals

Rich numeric notation is supported: `42`, `0x2a`, `0b101010`, `1_000_000`, `4.2`, `1.0e42`.

`String`s use double quotes: `"hello, world"`.

`Char`s use single quotes: `'a'`.

`Byte`s can be defined either with `b` prefixed to a character literal, e.g. `b'a'` is 0x61, or by using a `Nat` literal, which will be implictly casted if possible.

### Casting

There is a `Cast` interface which is implemented by primitive types.

Something like the following, but not quite. I haven't decided on the exact design yet.

```va
let x_int: Int = 12;
let x_float: Float = x_int.Cast.Float;
```


## Syntax

Whitespace is not significant.

Formatting options for autoformatters can be configured using the `@format` annotation. This applies to the formatting of the entire annotated expression.

```va
@format({ indentation = format.Indentation.Spaces { 4 } })
Module {
	/* ... */
}
```

Not all formatting options are inheritable. For example, modifying the indentation style of a single element of an array will have no effect on the indentation of the line overall.

```va
@format({ indentation = format.Indentation.Tabs })
{
	// The line is still indented with a tab.
	.foo = [1, @format({ indentation = format.Indentation.Spaces { 8 } }) 2, 3];
}
```

Single-line comments are `//`. Multi-line comments are `/* */`.





## Objects

Objects are like structs, classes, dictionaries, or maps in other languages. They are collections of properties defined by key-value pairs. Both keys and values may be of any type. Properties can be set using either `.` syntax or `.[expression]` with an arbitrary expression inside.

```va
let my_object: Object = {
	.foo = 5;
	.["bar"] = 42;
	.[9 + 7] = "sixteen";
	.[{ .key = "hello" }] = "world";
}
```

A key defined with `.` is identical to using `[]` with a string that is the same as the key, like in JavaScript. Properties can be indexed using either `.` or `[]`.

```va
let a = my_object["foo"];
let b = my_object.bar;
let c = my_object[16];
let d = my_object[{ .key = "hello" }];
```

The type of an object may be accessed from within the object itself using `Self`.

```va
let 

```

An empty object, `{}`, acts as the unit type for Vanth. When a function returns no value and is only used for its side-effects, its return type is `{}`. This is equivalent to `()` in Rust.


### Overrides

An object can be dereferenced with `.{ /* ... */ }` to create a new object with fields inherited from another object, with some of them possible changed. This is like an overlay in Nix.

```va
let x = { .inner = { .a = 6; .b = 8; .sum = $.a + $.b }; };
let y = x.inner.{ .b = 10; }.sum; // y is now `16`
```


### Variables

Variables are declared using the `let` keyword.

```va
let x = 1;
```

Although values themselves are immutable, variables can be reassigned to new values.

```va
let x = 1;
x = 2;
```

Variable shadowing is permitted, allowing the type of a variable to be changed.

```va
// `x` is now an `Int`.
let x = 1;
// `x` is now a `String`.
let x = "foo";
```

Note that it is often possible to reassign a variable to a different type, but without shadowing, this may force the type to be overly broad.

```va
let x = 1;
x = "foo";
// `x` now has the type `Int | String`, even though we know it can only be `String`.
```


## Hashing

Vanth hashes are 192-bit values represented as Base52, i.e. uppercase and lowercase ASCII letters only. They are always 34 characters long. They can optionally be truncated to any desired length for better readability but reduced security. If the `Vanth.hashTruncation` configuration property is set to `Vanth.HashTruncation.Deny`, then any truncated hashes will result in a build failure.

Any type can be suffixed with `#` followed by a hash of its expected value.


```va
	use vanth.pkgs#TYFTYfrtfghdRYerOhjvbfghcjklNXRTdFYUVBNHJvhjk;

	let x: [Int]#asqihoasq = [1, 2, 3];
```

If the actual hash of the expression `[1, 2, 3]` does not match the provided expected hash, then an error will occur.

The type itself can be left to be inferred using _.

```
let x: _#asqihoasq = [1, 2, 3];
```

Equality of values is entirely determined by their hash. If two values have the same hash, then they can be substituted for one another in any context to produce the same result, assuming no impure functions are used.

Note that fixed-length arrays have the same hash as dynamic-length arrays which happen to have the same length.

```va
let x: [Int] = [1, 2, 3];
let y: [Int; 3] = [1, 2, 3];
assert_eq(x, y);
```

The hash of the value returned by a function call is determined by the hash of the function itself and the hashes of the arguments passed to it. E.g. the hash of `foo(x, y)` is essentially the hashes of `foo`, `x`, and `y` combined together.

### Function hashes

The hash of a function is determined by its contents at the abstract syntax tree (AST) level. Formatting and whitespace do not affect the hash. Local variable names also do not affect the hash, but their position and usage in the function do.


### Locking

To calculate the hash of a particular value and add it to the source code of a Vanth file, run the command `vanth lock <expression path>`.

```
> cat test.va
.x: Int = some_calculation(15);
> vanth lock test.x
> cat test.va
.x: Int#oauyiqQIuASsaqwuiSAQDuiadwuidasiwdqiow = some_calculation(15);
```

If the expression for `x` was changed, e.g. to `some_calculation(16)`, then the file would fail to build because of a mismatched hash. This can be fixed by re-locking the expression with the same command.

To lock multiple expressions at once, expression path filters can be used, e.g. `vanth lock test._` will lock all top-level values in the file `test.va`.


## Memoization

Vanth uses an SQLite file to cache the results of evaluating expressions. Whenever it needs to evaluate the result of a function call, it calculates its hash and then checks to see if it has been calculated already. If so, the previously calculated value is reused.

No memoization is performed for impure functions.


## Imports

External dependencies can be imported somehow.


## Interfaces

```va
interface Foo {
	x: String,
	// Lazy evaluation means recursive fields are okay.
	y: Foo,
}

open interface OpenFoo {}

```

Open interfaces allow for any property to be defined for them. This is useful for e.g. `shell.Env`. A module can define its environment variables like so:

```va
Module {
	Module.shell.Env.SHELL = "zsh";
}
```

Or

```va
Module {
	Module.shell.Env += { .SHELL = "zsh"; }
}
```

You can restrict the type of additional properties that can be defined. I'm not sure about the syntax of this yet.

```va
open<String> interface Env {}
```


## Enums

Enums are named collections of types called variants. Any value can be an enum variant. Unlike Rust, enum variants are types themselves.

```va
enum Foo {
	Bar,
	Baz,
	interface BarBaz {
		x: String,
		y: Int,
	}
	"hello"
	5
	[10 + 4]
}

open enum OpenFoo {
	Bar,
}
```

In `Foo`, there are 6 variants. For 5 of the variants, only a single value will satisfy the type of the variant. E.g. the value `14` is the only value which is valid for the type `Foo.[14]`. The value `Baz` is the only value valid for the type `Foo.Baz`. For the `Foo.BarBaz` variant, there exist multiple values that satisfy its type, like `Foo.BarBaz { .x = "hi"; .y = 2 }`.

An enum is like a named version of a type intersection.

## Blocks

The contents within the curly braces `{}` of an expression are called a "block". Variables defined within a block are scoped locally to it.

```va
let x = 1;
let y = {
	let x = 2;
};
trace(x);
```

Here, `1` will be printed because the `x` inside the `y` block is a different variable.

Objects can also be used as collections of operations used to calculate a single, final value. The `return` keyword at the end of a block causes the entire expression to evaluate to its current state merged with the value provided to `return`.

```va
let x = {
	.foo = "hello";
	return { .bar = "world" };
}
```

Here, `x` evaluates to `{ .foo = "hello"; .bar = "world" }`.

In general, the value passed to `return` can be of any type, but if other properties are defined in the block, like `.foo` in the example above, then the `return` value must be an `Object`. If used, `return` must be the final expression within a block.

If only local variables are used, no properties, then this syntax can be used to create expressions that resembles the bodies of functions in other programming languages.

```va
let x = {
	let sum = 0;
	for i in 0..10 {
		sum += i;
	}
	return sum;
}
```

Here, `x` evaluates to `45`.


### Self-references

The keyword `$` can be used like `this` or `self` in other languages. I'm not sure if I'll use `$` or `self` as the syntax yet.

```va
let obj = {
	.a = 5;
	.b = $.a + 3; // .b is now 8.
};
```

`$` evaluates to the object block that is currently in scope. This works because of lazy evaluation. If a self-referential loop is detected, it results in a build error.


## Functions

Functions may be defined locally using the `fn` keyword, or the same way as properties, but with parentheses `()` suffixed onto their name. They may contain any number of parameters of any type.

```va
// Locally defined function
fn add(a: Int, b: Int) = a + b;

// Function that is a property of the object
.add(a: Int, b: Int) = a + b;

// Using an expression with `return`
fn fib(n: Int) = {
	let a = 0;
	let b = 1;
	for _ in 0..n {
		let new_value = a + b;
		a = b;
		b = new_value;
	}
	return b;
}
```

The return type of a function can be inferred, or it can be explicitly declared with `:`.

```va
let add(a: Int, b: Int): Int = a + b;
```

If the first parameter of a function is the keyword `self`, then it can be used as a method of its object.

```va
let x = {
	.data: [u8] = /* ... */;
	.to_json(self): String = { /* ... */ }
}
let json = x.to_json();
```

Functions are values themselves and can be used as such. The type of a function is annotated with parentheses, the same as the definition of a function itself but without the function and paramter identifiers.

```va
let add_one(x: Int): Int = x + 1;
let mapping_function: (Int): Int = add_one;
```


### Purity

All functions in Vanth are pure by default. This means that for a given set of input arguments, the returned value of a function called with those arguments is guaranteed to always be the same.

A function can be explicitly marked as impure with the annotation `@impure`. Functions which are impure can only be called by functions which are also impure themselves.

```va
use vanth.xml.[parse_xml, Xml];

// Okay
@impure
let get_xml(path: Path): Xml = read_file(path).parse_xml();

// Error, cannot call impure function `read_file` from pure function.
let get_xml_without_annotation(path: Path): Xml = read_file(path).parse_xml
```

Values which use the results from impure functions are themselves impure, unless locked to a hash, in which case they become pure again.

```va
// Must be annotated as `@impure`.
@impure
let x = get_xml(my_path);

// Pure, does not need `@impure`.
let y: _#IAyuiAuiaiAyusbhafaJVy = get_xml(my_path);
```

If the value returned by an impure function, in this case `get_xml`, does not match the provided hash then it will result in an error.


### Anonymous functions

Omit the name to make a function anonymous.

```va
let squared = range(0, 20).map(fn(n: Int) = n * n);
```


### Implicit parameters

Use `$` followed by a number to define expressions that behave as anonymous functions. This is like the `it` keyword in Kotlin.

```va
let squared = range(0, 20).map($1 * $1);
```

This value can be used like any other, including accessing its fields.

```va
let person_a = { .name: "Bob"; .age = 2 };
let person_b = { .name: "Maria"; .age = 87 };
let ages = [person_a, person_b].map($1.age);
```

`$0` within an anonymous function refers to the function itself. This can be used for recursive calls of anonymous functions.

```va
let prime_fibs = [2, 3, 5, 7, 11].map(match $1 { 0 | 1 => $1; _ => $0($1 - 1) + $0($1 - 2) });
```


## Pattern matching

Pattern matching can be used to destructure objects.

```va
let x = [1, 2, 3];
let y = match x {
	[1, _, _] => "starts with 1";
	[_, _, 3] => "ends with 3";
	_ => "other";
};
```

Matching is exhaustive, like in Rust.

Since types are values, it is also possible to match on types:

```va
fn type_as_string(ty: Type) = match ty {

}
```


## Filters

The `Filter` type is like a regex for Vanth expression paths. E.g. the filter `gcc._.include` will match `gcc.v13_3_0.include`.

`[]` syntax can be used in filters for more fine-grained matching, e.g. `gcc.["v13_3_0" | "v13_2_0"].include`.


## Advanced types

### Generics

Types can take generic parameters with the syntax `<>`.

```va
interface List<T> {
	.get(index: Nat): T;
	.len: Nat;
}
```


### Negative types

A type can be negated by prefixing it with `!`. Values then only satisfy this type if they don't implement it.

```
let x: !Int = "hello"; // Okay
x = false; // Okay
x = 5; // Error
```


## Annotations

Any expression may be prefixed with an annotation of the form `@annotation_name(argument_value)`. If an annotation does not take an argument then the parentheses are unnecessary, `@annotation_name`. The type of argument value is specific to the annotation, but in general this may be any type.

Annotations may be placed on the same line as an expression, separated by a space, or they may be on separate lines.

```va

```


## Documentation

The `@doc` annotation is used to provide documentation for values and types.

Strings inside it, e.g. `@doc("this function fixes things")` can use interpolation, `@doc("example usage: {foo(5, 6)}")`. Specifically for strings in `@doc`, these expressions are not expanded but rather printed verbatim. The interpreter still checks that they evaluate successfully, otherwise the docs fail to compile.


## Licenses

Modules have a `Module.license: String` property, as well as a `Module.allowLicenses` and `Module.denyLicenses` property. I don't know what the types of these properties will be yet. Modules will only build successfully if the license of every one of their dependencies recursively satisfies the specified license requirements.
