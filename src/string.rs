// Primitive str  = Immutable fixed-length string somewhere in memory.
// String  =  Growable, heap-allocated data structure-- use when you need to modify or own string data

pub fn run(){

    let mut hello = String::from("Hello ");

    // get length 
    println!("Length: {}", hello.len());

    // push char
    hello.push('W');

    // push string
    hello.push_str("orld!");

    // Capacity in bytes

    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("is empty: {}", hello.is_empty());

    // Contains 
    println!("contains 'World': {}", hello.contains("World"));

    // replace
    println!("replace: {}", hello.replace("World", "There"));

    //Loop through string by white space
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity
    let mut s =  String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", hello);

}