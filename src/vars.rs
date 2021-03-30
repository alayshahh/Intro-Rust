// variables hold primitive data or references to data
// varaibels are immutable by default
// rust is a block scoped language

pub fn run (){
    let name = "Brad";
    let mut age  = 37; //need mut keyword to change the  value of the variables
   
    println!("my name is {} and I am {}", name, age);
    
    age = 38; 

    println!("my name is {} and I am {}", name, age);

    //define const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars at a time
    let (my_name, my_age) =  ("Alay", 20);
    
    println!("Name: {}, Age: {}", my_name, my_age);



}