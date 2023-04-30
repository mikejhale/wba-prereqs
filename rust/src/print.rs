pub fn run() {
    println!("Hello from the print.rs file");

    println!("{} is from {}", "Mike", "Illinois");

    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Mike", "IL", "smoke cigars"
    );

    println!(
        "{name} likes to {activity}",
        name = "Kari",
        activity = "run marathons"
    );

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "hello"));
    println!("10 + 10 = {}", 10 + 10);
}
