# The Papy Programming Language #

## What is Papy ##

Papy is a stack based programming language. you put items on the stack, and then operate on them with functions.

the program is equivalent to the infix notation expression ((5 + 4) + 3):

```
3
4
5
+
+
```

Papy is capable of user defined functions of the following syntax:

```
def function_name arity: body end
```

an example, making a `double` function.

```
def double 1: %0 %0 + end
```

the arity is a u32, and the %NUMBER is 0-based indices of local-stack coordinates.

note that user-defined-functions dont actually work right now...!

## running ##

clone this repo and `cargo run` for instant gratification.

Right now the only way to run programs is by modifying the `lines` vector in src/bin/papy.rs

## USAGE ##

Papy


TODO:

* User-defined-functions
* Clean up interpreter.rs
* Unboxed closures as opposed to function pointers.
* Conditional and looping constructs

License: MIT
