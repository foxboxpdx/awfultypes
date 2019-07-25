// The Shambles - A Vec that has fallen over and can't get up
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec::Drain;
use std::vec::Splice;
use std::ops::RangeBounds;
use std::fmt::Debug;


pub struct Shambles<T> where T: Debug + Clone {
	x: Vec<T>
}

impl <T> Shambles<T> where T: Debug + Clone {
	pub fn new() -> Shambles<T> {
		Shambles { x: Vec::new() }
	}
	
	pub fn from_vec(v: Vec<T>) -> Shambles<T> {
		let mut x = Shambles { x: v };
		x.randomize();
		x
	}
	
	pub fn capacity(&mut self) -> usize {
		self.randomize();
		self.x.capacity()
	}
	
	pub fn get(&mut self, index: usize) -> &T {
		self.randomize();
		&self.x[index]
	}
	
	pub fn as_slice(&mut self) -> &[T] {
		self.randomize();
		self.x.as_slice()
	}
	
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		self.randomize();
		self.x.as_mut_slice()
	}
	
	pub fn insert(&mut self, index: usize, element: T) {
		self.x.insert(index, element);
		self.randomize();
	}
	
	pub fn remove(&mut self, index: usize) -> T {
		let retval = self.x.remove(index);
		self.randomize();
		retval
	}
	
	pub fn push(&mut self, value: T) {
		self.x.push(value);
		self.randomize();
	}
	
	pub fn pop(&mut self) -> Option<T> {
		let retval = self.x.pop();
		self.randomize();
		retval
	}
	
	pub fn append(&mut self, other: &mut Self) where T: std::clone::Clone {
		self.x.append(&mut other.gimmie());
		self.randomize();
	}
	
	pub fn drain<R>(&mut self, range: R) -> Drain<T> where R: std::ops::RangeBounds<usize> {
		self.randomize();
		self.x.drain(range)
	}
	
	pub fn clear(&mut self) {
		self.x.clear();
	}
	
	pub fn len(&mut self) -> usize {
		self.randomize();
		self.x.len()
	}
	
	pub fn is_empty(&mut self) -> bool {
		self.randomize();
		self.x.is_empty()
	}
	
	pub fn split_off(&mut self, at: usize) -> Vec<T> {
		self.randomize();
		self.x.split_off(at)
	}
	
	pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<<I as IntoIterator>::IntoIter> where I: IntoIterator<Item = T>, R: RangeBounds<usize> {
		self.randomize();
		self.x.splice(range, replace_with)
	}
	
	pub fn gimmie(&mut self) -> Vec<T> where T: Clone {
		self.x.clone()
	}
	
	pub fn randomize(&mut self) {
		let mut rng = thread_rng();
		self.x.shuffle(&mut rng);
	}
}

impl <T>std::fmt::Display for Shambles<T> where T: Debug + Clone {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result where T: Debug {
		write!(f, "{:?}", self.clone().as_slice())
	}
}

impl <T>Debug for Shambles<T> where T: Debug + Clone {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result where T: Debug {
		write!(f, "{:?}", self.x)
	}
}

impl <T>Clone for Shambles<T> where T: Debug + Clone {
	fn clone(&self) -> Shambles<T> where T: Debug {
		let v = self.x.clone();
		Shambles { x: v }
	}
	
	fn clone_from(&mut self, source: &Self) where T: Debug + Clone {
		self.x.clone_from(&source.x)
	}
}

impl <T>std::ops::Deref for Shambles<T> where T: Debug + Clone {
	type Target = [T];
	
	fn deref(&self) -> &[T] {
		&self.x
	}
}

impl <T>std::ops::DerefMut for Shambles<T> where T: Debug + Clone {
	fn deref_mut(&mut self) -> &mut [T] {
		self.randomize();
		&mut self.x
	}
}


