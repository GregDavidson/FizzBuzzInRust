fn main() {
    let mut fizz = ( (0..3).rev().cycle(), "fizz" );
    let mut buzz = ( (0..5).rev().cycle(), "buzz" );
    let mut boom = ( (0..7).rev().cycle(), "boom" );
    let mut pairs = vec!(&mut fizz, &mut buzz, &mut boom);
    for n in 1..101 {
        if pairs.iter_mut().fold( // did we print anything?
            false, |b, pair|
            if let Some(x) = pair.0.next() {
                if x != 0 { b } else {
                    print!("{}", pair.1); true
                }
            } else { panic!("not a cycle") }
        ) { println!("") } else { println!("{}", n) }
    }
}
