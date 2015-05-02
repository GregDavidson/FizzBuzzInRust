_--> [My Public Page Is Here](https://gregdavidson.github.io/) <--_


# Fizz Buzz Variations in Rust

Exploring the
[FizzBuzz Game](https://en.wikipedia.org/wiki/Fizz_buzz) as
an exercise in learning the
[Rust Language](http://www.rust-lang.org/) I came up with
four solutions exhibited here along with a development
version containing some initial exploration.

If you haven't tried solving the FizzBuzz Game before, you
might want to try solving it in your favorite language and
perhaps also explore Rust a bit before looking at these
_SPOILERS_.

In reverse chronological order these are:

| File | Technique
|-----:|:---------
| fizz_buzz-closure.rs | heap-allocated closures
| fizz_buzz-if-let.rs | cyclic iterators with if let
| fizz_buzz-match.rs | cyclic iterators with match
| fizz_buzz-looper.rs | mutable structures
| fizz_buzz-devel.rs | early exploration and false starts

My first idea was to use closures.  I didn't know enough to
make that work until I'd found the other solutions.  I was
delighted when I found the trick of creating cyclic
iterators using standard components.

I would love to find a flexible approach that didn't require
so much mutability without incurring the usual overhead of
using the mod operator or growing the heap.  Tail Call
Optimization would help but Rust doesn't offer that yet.

J. Greg Davidson, Thursday, 30 April 2015
