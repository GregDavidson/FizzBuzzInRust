struct Looper {
    var: i32,                   // mut
    to: i32,
    val: &'static str,
}

impl Looper {
    fn bump(&mut self) -> bool {
        if self.var == self.to {
            self.var = 1; print!("{}", self.val); true
        } else {
            self.var += 1; false
        }
    }
}

fn main() {
    let mut fizz = Looper { var: 1, to: 3, val: "fizz" };
    let mut buzz = Looper { var: 1, to: 5, val: "buzz" };
    let mut loopers = [&mut fizz, &mut buzz];
    for n in 1..101 {
        if ! loopers.iter_mut().fold(
            false, |b, ref mut x| x.bump() || b
        ) { print!("{}", n) }
        // let mut printed_something = false;
        // for it in loopers.iter_mut() {
        //     if it.bump() { printed_something = true; }
        // }
        // if ! printed_something { print!("{}", n) }
        println!("")
    }
}
