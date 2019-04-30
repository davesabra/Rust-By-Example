// pop and slice
use std::mem;
// This function borrows a slice
fn analyze_slice(slice: &[f64]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

// slice references are a good choice for function that 
 // operates on a homogeneous data set
fn	print(n: &[f64]) {          // shared slice of f64s
    for elem in n {								
        println!("{}",elem);				
    } 
}

fn main() {
    //println!("Hello, world!");
    // You	can	use	the	pop	method	to	remove	the	last	element	of a vector
    // and	return	it. More precisely,	popping	a	value	from	
    // a	Vec<T>	returns	an	Option<T>:
    // None	if	the	vector	was	already	empty, or Some(v) if its last element had been	v:

    //let	mut	v = vec!["carmen","miranda"]; 
    //assert_eq!(v.pop(),	Some("miranda"));
    //assert_eq!(v.pop(),	Some("carmen")); 
    //assert_eq!(v.pop(),	None);

    //
    let	v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707]; 
    let	a:	[f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let	sv:	&[f64] = &v; 
    let sa: &[f64] = &a;

    //println!("{:?}, {:?}", v,a );
    //println!("{:?}, {:?}", sv, sa );
    //print(&v);
    //print(&a);		
    //	print fn works	on	vectors		
    //	works	on	arrays

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&sa));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&sa);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&sa[0 .. 1]);

    /*
    You	can	get	a reference	to a slice of an array or vector,
    or a slice of an existing slice, by	indexing it	with a range:
    */
    println!("{:?}, {:?}", v,a );
    //print(&v[0..2]);             // <-- print the first two elements of v
    //print(&a[2..]);              // <-- print elements of a starting with a[2]
    print(&sv[1..3]);            //<--print v[1] and v[2]








}
