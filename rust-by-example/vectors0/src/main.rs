use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
    
   // using fold() is functional, using for.. is imperative
    let numbers = [1,2,3,4,5];
    let mut result1 = 0;

    // for loop
    for i in &numbers{
        result1 = result1 + i;
    }

    // fold
    let result2 = numbers.iter().fold(0,|acc, &x| acc+x);

    //look
    assert_eq!(result1,result2);
    println!("{} = {}", result1, result2);

    //varios ways to build and initialize vectors
    let	mut v = vec![2,	3,	5,	7]; 
    assert_eq!(v.iter().fold(1,	|a,	b|	a*b), 210);

    v.push(11); 
    v.push(13); 
    assert_eq!(v.iter().fold(1,	|a,	b|	a*b),	30030);

    let	mut	v =	Vec::new(); 
    v.push("step"); 
    v.push("on"); 
    v.push("no"); 
    v.push("pets"); 
    assert_eq!(v, vec!["step",	"on",	"no",	"pets"]);
    println!("{:?}", v);

    // or build a vector from the values produced by an iterator
    let	v: Vec<i32>	= (0..5).collect();
    assert_eq!(v, [0,1,2,3,4]);

    /*
    You	can	insert	and	remove	elements	wherever	you	like
    in	a	vector, although	these	operations	shift	all	the	elements
    after	the	insertion	point forward	or	backward,	so	they	may	be	slow
    if	the	vector	is	long:
    */
    let	mut	v	=	vec![10,	20,	30,	40,	50];
    //	Make	the	element	at	index	3	be	35. 
    v.insert(3,	35); 
    assert_eq!(v,	[10,	20,	30,	35,	40,	50]);
    //	Remove	the	element	at	index	1. 
    v.remove(1); 
    assert_eq!(v,	[10,	30,	35,	40,	50]);



    













    












}
