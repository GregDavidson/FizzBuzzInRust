fn test1() {
    println!("*** test1 ***");
    let mut fizz = (0..3).rev().cycle();
    let mut buzz = (0..5).rev().cycle();
    for n in 1..21 {
        println!("{}: fizz {} buzz {}",
                 n,
                 match fizz.next() {
                     Some(x) => x,
                     None => panic!("fell off cycle"),
                 },
                 match buzz.next() {
                     Some(x) => x,
                     None => panic!("fell off cycle"),
                 }
        )
    }
    println!("");
}

fn test2() {
    println!("*** test2 ***");
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    for n in 1..21 {
        let f = match fizz.0.next() {
            Some(x) => x,
            None => panic!("fell off cycle"),
        };
        let b = match buzz.0.next() {
            Some(x) => x,
            None => panic!("fell off cycle"),
        };
        if f != 0 && b != 0 { println!("{}", n) } else {
            if f == 0 { print!("{}", fizz.1 ) }
            if b == 0 { print!("{}", buzz.1 ) }
            println!("");
        }
    }
    println!("");
}

fn test2b() {
    println!("*** test2b ***");
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    for n in 1..21 {
        let mut printed_something = false;
        {   let pair = &mut fizz;
            if match pair.0.next() {
                Some(x) => x,
                None => panic!("fell off cycle"),
            } == 0 {
                print!("{}", pair.1);
                printed_something = true;
            }
        }
        {   let pair = &mut buzz;
            if match pair.0.next() {
                Some(x) => x,
                None => panic!("fell off cycle"),
            } == 0 {
                print!("{}", pair.1);
                printed_something = true;
            }
        }
        if printed_something {
            println!("")
        } else {
            println!("{}", n)
        }
    }
    println!("");
}

fn test3() {
    println!("*** test3 ***");
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    for n in 1..21 {
        let pairs = vec!(
            ( match fizz.0.next() {
                Some(x) => x,
                None => panic!("fell off cycle"),
            }, fizz.1 ),
            ( match buzz.0.next() {
                Some(x) => x,
                None => panic!("fell off cycle"),
            }, buzz.1 ) );
        if pairs.iter().fold(
            false, |b, pair| if pair.0 != 0 { b } else {
                print!("{}", pair.1); true
            }
        ) { println!("") } else { println!("{}", n) }
    }
    println!("");
}

#[cfg(foo)]
fn next(
    ref mut it:
    core::iter::Cycle<core::iter::Rev<core::ops::Range<_>>>
) -> int32 {
    match(it.next()) {
        Some(x) => x,
        None => panic!("fell off cycle")
    }
}

#[cfg(foo)]
fn fold( ref mut iters:
collections::vec::Vec<&core::iter::Cycle<core::iter::Rev<core::ops::Range<_>>>>
) -> bool {
    true
}

#[cfg(foo)]
fn test4() {
    println!("*** test4 ***");
    let mut fizz = (0..3).rev().cycle();
    let mut buzz = (0..5).rev().cycle();
    let mut iters = vec!(&fizz, &buzz);
    // let mut ii: () = iters;
    // collections::vec::Vec<&core::iter::Cycle<core::iter::Rev<core::ops::Range<_>>>>
    // let mut ff: () = iters[0];
    // core::iter::Cycle<core::iter::Rev<core::ops::Range<_>>>
    // for n in 1..21 { println!("{}: {} {}", n,
    //     next(iters[0]), next(iters[1])); }
    for n in 1..21 {
        println!(
            "{}: {}",
            n,
            iters.iter_mut().fold(0, |b, ref mut x| {
                let mut next = x.next();
                match next {
                    Some(xx) => xx,
                    None => panic!("fell off cycle")
                }
            })
        )
    }
    println!("");
}

#[cfg(foo)]
fn fizz_buzz() {
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    let mut pairs = vec!(&fizz, &buzz);
    for n in 1..21 {
        if pairs.iter_mut().fold(
            false, |b, pair| match pair.0.next() {
// error:                          ^~~~~~
// cannot borrow immutable anonymous field `pair.0` as mutable
                Some(x) =>
                    if (x != 0) { b } else {
                        print!("{}", pair.1); true
                    },
                None => panic!("fell off cycle")
            }
        ) { println!("") } else { println!("{}", n) }
    }
}

#[cfg(foo)]
fn fizz_buzz() {
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    let mut pairs = vec!(&fizz, &buzz);
    for n in 1..21 {
        if pairs.iter_mut().fold(
            false, |b, pair| {
                let it = &mut pair.0;
                match it.next() {
                    Some(x) =>
                        if (x != 0) { b } else {
                            print!("{}", pair.1); true
                        },
                    None => panic!("fell off cycle")
                }
            }
        ) { println!("") } else { println!("{}", n) }
    }
}

fn fizz_buzz() {
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    let mut pairs = vec!(&mut fizz, &mut buzz);
    for n in 1..21 {
        if pairs.iter_mut().fold(
            false, |b, pair|
                match pair.0.next() {
                    Some(x) =>
                        if (x != 0) { b } else {
                            print!("{}", pair.1); true
                        },
                    None => panic!("fell off cycle")
                }
        ) { println!("") } else { println!("{}", n) }
    }
}

fn main() {
    // test1();
    // test2();
    // test2b();
    // test3();
    fizz_buzz();
}
