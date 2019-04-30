
extern	crate	iron; 
//#[macro_use]extern	crate	mime
//pub use self::Type::{Application, Audio, Image, Message, Model, Multipart, Text, Video};
#[macro_use]extern crate media_types;


use	iron::prelude::*; 
use	iron::status;


fn	main()	{

println!("Serving	on	http://localhost:3000...");
Iron::new(get_form).http("localhost:3000").unwrap();      //iron::new creates server
}

fn	get_form(_request: &mut Request) ->	IronResult<Response> {	
    let	mut	response =	Response::new();
	response.set_mut(status::Ok);				
    response.set_mut(mime!(Text/Html;	Charset=Utf8));		//mime! in now media type		
    response.set_mut(r#"
    	<title>GCD	Calculator</title>	
    	<form	action="/gcd"	method="post">
          <input	type="text"	name="n"/>
          <input	type="text"	name="n"/>
          <button	type="submit">ComputeGCD</button>
        </form>
                    "#);
	Ok(response) // control flows off here,  Rusts 'last expression' of this block{}          
}

