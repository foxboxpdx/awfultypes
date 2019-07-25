// A sample program using The Shambles
extern crate awfultypes;
use awfultypes::shambles::Shambles;

fn main() {
	println!("Behold the Shambles!");
	println!("Initiaizing 'x' as a Shambles of i32...");
	let mut x = Shambles::from_vec(vec![1, 2, 3, 4, 5, 6]);
	println!("x: [1, 2, 3, 4, 5, 6]");
	println!("Initializing 'y' as a Shambles of String...");
	let mut y = Shambles::from_vec(vec!["One".to_string(), "Two".to_string(), "Three".to_string(), "Four".to_string(), "Five".to_string()]);
	println!("y: ['One', 'Two', 'Three', 'Four', 'Five']");
	println!("Now lets try to grab something from index 2 on both of these...");
	println!("x[2] = {}", x[2]);
	println!("y[2] = {}", y[2]);
	println!("Uhoh.  How about we call 'get'?");
	println!("x.get(2) = {}", x.get(2));
	println!("y.get(2) = {}", y.get(2));
	println!("Are they really going to randomize with EVERY operation?");
	println!("x = {:?}", x);
	println!("x.len = {}", x.len());
	println!("x = {:?}", x);
	println!("This is the worst thing ever.");
}
