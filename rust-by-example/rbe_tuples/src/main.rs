use std::fmt; // Import `fmt`


 #[derive(Debug)]
//struct Matrix0(f32, f32, f32, f32);
struct Matrix(u32, u32, u32, u32);

fn main() {
//println!("Hello, world!");
    //let	t =	(12, "eggs"); 
    //let	b = Box::new(t);		//	allocate	a	tuple	in	the	heap
    /*
    The	type	of	t	is	(i32,	&str),	so	the	type	of	b	
    is	Box<(i32,	&str)>.	Box::new() allocates	enough	memory	
    to	contain	the	tuple	on	the	heap.	When	b	goes out	of	scope,
    the	memory	is	freed	immediately,	unless	b	has	been	moved 
    —by	returning	it,	for	example
    */
//println!("{:?}",t );
    //	Given	a	tuple	value	t,	you	can	access	its	elements
    //	as	t.0,	t.1,	and so	on.
//println!("{}", t.1); 

//let matrix0 = Matrix0(1.1, 1.2, 2.1, 2.2);
let matrix = Matrix(0,8,5,0);

pub fn reverse(pair: (u32,u32)) -> (u32,u32) {
    let (x,y,) = pair;
    (y,x,)
}

pub fn transpose(input: Matrix) -> (Matrix) {
    
    let matrix = input;
    let (x, y) = reverse((matrix.1, matrix.2));
    
    Matrix(matrix.0, x, y, matrix.3,)
    
       
}

println!("Debug: {:?}", matrix);  //printing in debug format

    //try printing in display/release format
    // Implement `Display` for `MinMax`.
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{} {}\n{} {}", self.0, self.1, self.2, self.3)
    }
}
println!("Matrix:\n{}", matrix);
println!("\n");
println!("Transpose:\n{}", transpose(matrix));
   
}
