use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
	name:  String,
	left:  usize,
	right: usize
}

impl Philosopher {

	fn new(name: &str) -> Philosopher {
		
		Philosopher {
			name: name.to_string()
		}
	}
	
	fn eat(&self) {
	
		println!("{} has started eating.", self.name);
		thread::sleep_ms(1000);
		println!("{} is done eating.", self.name);
	}
}

struct Table {
	forks: Vec<Mutex<()>>
}

fn main() {

	let table = Arc::new(Table {forks: vec![
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(())
	]});

	let philosophers = vec![
		Philosopher::new("Judith Butler"),
		Philosopher::new("Gilles Deleuze"),
		Philosopher::new("Karl Marx"),
		Philosopher::new("Emma Goldman"),
		Philosopher::new("Michel Foucault")
	];
	
	let handles: Vec<_> = philosophers.into_iter().map(|p| {
		
		let table = table.clone();
		
		thread::spawn(move || {
			
			p.eat();
		})
	}).collect();
	
	for h in handles {
	
		h.join().unwrap();
	}
}
