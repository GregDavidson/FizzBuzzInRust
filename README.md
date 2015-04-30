# Fizz Buzz Variations in Rust

Exploring the
[FizzBuzz Game](https://en.wikipedia.org/wiki/Fizz_buzz) as
an exercise in learning the
[Rust Language](http://www.rust-lang.org/) I came up with
four solutions exhibited here along with a development
version containing some initial exploration.

In reverse chronological order these are:

| File | Technique
|-----:|:---------
| fizz_buzz-closure.rs | heap-allocated closures
| fizz_buzz-if-let.rs | cyclic iterators with if let
| fizz_buzz-match.rs | cyclic iterators with match
| fizz_buzz-looper.rs | mutable structures
| fizz_buzz-devel.rs | early exploration and false starts

My first idea was to use closures and I didn't know enough
to make that work until I'd found the other solutions.  I
was delighted to discover the trick of implemented cyclic
iterators with standard components.

I would love to find a flexible approach that didn't require
so much mutability, without incurring the usual overhead of
using the mod operator or growing the heap.  Tail Call
Optimization would help but Rust doesn't offer this yet.

J. Greg Davidson, Thursday, 30 April 2015
