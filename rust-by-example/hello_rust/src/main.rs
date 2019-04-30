fn main() {
    println!("Hello, world!");
    println!("I'm a rustacean. Learning Rust By Example");
    println!("with Exercism and VSCode.");

    let elem1 = 3u8;
    let mut vec = Vec::new();  //<-creates an empty vector, a 'growable' array
    
    // rustc doesnt know the exact type of Vec
    // now insert elem in the vector
    vec.push(elem1);
    // now rustc knows that vec is a vector of u8's,  (type-inference)
    println!("{:?}", vec);

    //single requirement is that the array is homogeneous w.r.t. type (here all u8's)

    let elem2 = 4u8;
    let elem3 = 5u8;

    vec.push(elem2);
    vec.push(elem3);

    println!("{:?}", vec);   // <- output [3,4,5]


}
