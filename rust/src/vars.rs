pub fn run() {
    let name = "Mike";
    let mut age = 50;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Mike", 50);
    println!("{} is {}", my_name, my_age);
}
