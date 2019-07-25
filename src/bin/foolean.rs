// A sample program using The Shambles
extern crate awfultypes;
use awfultypes::foolean::Foolean;

fn main() {
	println!("Behold the Foolean!");
	println!("Initializing 'x' as 'true'");
	let x = Foolean::new(true);
	println!("Debug output: {:?}", x);
	println!("Regular output: {}", x);
	println!("Let's try some .gets! {}\t{}\t{}\t{}", x.get(), x.get(), x.get(), x.get());
	println!("And some Nots: {}\t{}", !x, !x);
	println!("We can clone 'x' but there's no guarantee it'll be equal...");
	let y = x.clone();
	println!("What does debug say about the clone? {:?}", y);
	println!("Equality checks: {}\t{}\t{}", x == y, x == y, x == y);
	println!("And finally, test string parsing:");
	let z: Foolean = "true".parse().unwrap();
	println!("True: {:?}", z);
	let zz: Foolean = "false".parse().unwrap();
	println!("False: {:?}", zz);
	match "what".parse::<Foolean>() {
		Ok(_o) => panic!("Oh god it parsed 'what' as a bool help"),
		Err(e) => { println!("Oh look, 'what' isnt a bool: {}", e); }
	}
	println!("Sic transit gloria Foolean!");
}
