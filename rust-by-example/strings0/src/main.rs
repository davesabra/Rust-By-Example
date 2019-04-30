extern crate regex;
use regex::Regex;
fn main() {
    println!("Hello, world!");
    let	speech	=	"\"Ouch!\"	said	the	well.\n";
    println!("{}", speech);
    println!("In the room the women	come and go,				
        Singing	of Mount Abora");
    println!("It was a bright, cold	day	in April, and \
    	there were four of us—\
        more or less.");

    let	default_win_install_path =	r"C:\Program Files\Gorillas";
    //let	pattern	= Regex::new(r"\d+(\.\d+)*");
    println!("{}", default_win_install_path);
    //println!("{}", pattern);// not allowed

    let	pattern	= Regex::new(r"\d+(\.\d+)*").unwrap();
    //assert!(pattern.is_match(r"\d+(\.\d+)*"));// why doesnt this match?
    assert!(pattern.is_match(r"\"));

    println!(r###"
        This raw string started with 'r###"'.				
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by	three pound signs ('###'): 
        "###);





}
