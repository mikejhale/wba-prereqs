pub fn run() {
    let mut count = 0;

    for x in 0..100 {
        if x % 15 == 0 {
            println!("foobar");
        } else if x % 3 == 0 {
            println!("foo");
        } else if x % 5 == 0 {
            println!("bar")
        } else {
            println!("{}", x);
        }
    }
}
