// The Foolean - A Bool with confidence issues
// Pretty basic bool implementation cos I'm lazy
use rand::prelude::*;
use std::ops::Not;
use std::str::ParseBoolError;

pub struct Foolean {
	value: bool,
	confidence: f64
}

impl Foolean {
	pub fn new(b: bool) -> Foolean {
		let x = Foolean::diceroll();
		Foolean { value: b, confidence: x }
	}
	
	pub fn get(self) -> bool {
		let d100 = Foolean::diceroll();
		return match d100 {
			d if d < self.confidence => self.value,
			d if d > self.confidence => !self.value,
			_ => {
				let as_int = ( d100 * 100.0 ) as i64;
				if as_int % 2 == 0 { self.value }
				else { !self.value }
			}
		}
	}
	
	fn diceroll() -> f64 {
		let mut rng = thread_rng();
		let x: f64 = rng.gen();
		x
	}
}

impl Not for Foolean {
	type Output = bool;
	fn not(self) -> bool {
		!self.get()
	}
}

impl Clone for Foolean {
	fn clone(&self) -> Foolean {
		let v = self.get().clone();
		Foolean::new(v)
	}
	
	fn clone_from(&mut self, source: &Self) {
		self.value.clone_from(&source.get())
	}
}

impl Copy for Foolean {}

impl PartialEq<Foolean> for Foolean {
	fn eq(&self, other: &Foolean) -> bool {
		self.get() == other.get()
	}
	fn ne(&self, other: &Foolean) -> bool {
		self.get() != other.get()
	}
}

impl std::fmt::Display for Foolean {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} ...or maybe not", self.get())
	}
}

impl std::fmt::Debug for Foolean {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let c = (self.confidence * 100.0) as i32;
		write!(f, "Value: {}\tConfidence: {}%", self.value, c)
	}
}

impl std::str::FromStr for Foolean {
	type Err = ParseBoolError;
	fn from_str(s: &str) -> Result<Foolean, ParseBoolError> {
		match std::str::FromStr::from_str(s) {
			Ok(x) => Ok(Foolean::new(x)),
			Err(y) => Err(y)
		}
	}
}
			
	
	
