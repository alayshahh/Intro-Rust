// Reference pointers - Point to a resource in memory

pub fn run(){
    
    // Primitive array
    let mut arr1 = [1,2,3];
    let arr2 = arr1;
    arr1[0] = 5;

    println!("Values: {:?}", (arr1, arr2));

    // With non primitives, if you assign another varaible to a piece of data, 
    // the first variable will no longer hold that value.
    // you will need to usea reference (&) to point to the resource.

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Values Vector: {:?}",(&vec1, vec2)); //the reason that we need to use &vec1 is because vec2 borrows vec1 and we cannot use vec1 until vec2 returns vec1.
}