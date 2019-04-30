use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `vec` in `v` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}",count, v)?;                  // prints the index - count
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![3, 4, 5]);
    println!("{}", v);
    

}
// idiom, Print each index i with its value x from an array-like collection -- items
/*
for (i, x) in items.iter().enumerate() {
    println!("Item {} = {}", i, x);
}
*/
