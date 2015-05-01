fn main() {
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    let mut boom = ( (0..7).rev().cycle(), "boom" );
    let mut pairs = [&mut fizz, &mut buzz, &mut boom];
    for n in 1..101 {
        if pairs.iter_mut().fold( // did we print anything?
            false, |b, pair|
                match pair.0.next() {
                    Some(x) =>
                        if x != 0 { b } else {
                            print!("{}", pair.1); true
                        },
                    None => panic!("fell off cycle")
                }
        ) { println!("") } else { println!("{}", n) }
    }
}
