// Loops are used to interate until a condition is met

pub fn run() {
    // let mut count = 0;

    //Infite Loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }
    // FizzBuzz while loop
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if count % 5 == 0 {
    //         println!("Fizz");
    //     } else if count % 3 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     count += 1
    // }

    //  for range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 5 == 0 {
            println!("Fizz");
        } else if x % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
