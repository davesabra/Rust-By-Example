fn main() {
    println!("Hello, world!");
    let a: u8 = 105;
    let b: u8 = 91;
    println!("a       = {:0>8b}", a);
    println!("b       = {:0>8b}", b);
    println!("a | b   = {:0>8b}", a | b);
    println!("a & b   = {:0>8b}", a & b);
    println!("a ^ b   = {:0>8b}", a ^ b);
    println!("!a      = {:0>8b}", !a);
    println!("a << 3  = {:0>8b}", a << 3);
    println!("a >> 3  = {:0>8b}", a >> 3);

    let _x: u8 = 3;
    println!("_x      = {:0>8b}", _x);
    println!(" _x << 1  = {:0>8b}", _x << 1); // bitshift 1 column to left == binary doubling


}
