extern crate regex;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let pattern = Regex::new(r"\d+(\.\d+)*").unwrap();
    //assert!(pattern.is_match(r"\dd\z\dddd")); // <--  this is a match, there are many, many more

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";
    /*
    oodles is a &str referring to the last six	bytes of the text belonging	to noodles,
    so it	represents	the	text	“oodles”.
    */
    println!("Do {} eat {} of {} ?", poodles, oodles, noodles);

    assert_eq!("ಠ_ಠ".len(), 7);
    assert_eq!("ಠ_ಠ".chars().count(), 3);

    //let mut s = "hello";
    // s[0] = 'c';         //	error:	the	type `str` cannot be mutably indexed
    //s.push('\n');		//	error:	no	method	named `push` found for type	`&str`

    // there are several ways to create strings
    let error_msg = "whaaaaat?".to_string();
    println!("{}", error_msg);

    assert_eq!(
        format!("{}°{:02}′{:02}″N", 24, 5, 23),
        "24°05′23″N".to_string()
    );

    // arrays, slices and vectors have two methods concat() and join() that can
    // form new strings from old bits
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(",	"), "veni,	vidi,	vici");
}
