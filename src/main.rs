fn new_looper(count_to: i32, string: &'static str)
-> Box<FnMut() -> Option<&'static str>> {
    let mut count = Box::new( 1 );
    Box::new( move | | {
        *count += 1;
        if *count <= count_to { None } else {
            *count = 1; Some(string)
        }
    } )
}

fn main() {
    let mut loopers = vec!(
        new_looper(3, "fizz"),
        new_looper(5, "buzz"),
        new_looper(7, "boom")
    );
    for n in 1..101 {
        if loopers.iter_mut().fold( // did we print anything?
            false, |b, looper|
                if let Some(string) = looper() {
                    print!("{}", string); true
                } else { b }
        ) { println!("") } else { println!("{}", n) }
    }
}
