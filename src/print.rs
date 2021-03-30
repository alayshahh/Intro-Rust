pub fn run() {
    //print to console
    println!("Hello from the print.rs file");

    //basic formatting
    println!("{} is from {}", "Brad", "NJ");

    //positional args
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    //names args
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "basketball"
    );

    //place holder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //place hodler for debug traits
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10+10);

}
