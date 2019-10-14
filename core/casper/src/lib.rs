extern crate futures;
extern crate futures_timer;

use std::time::{Instant, Duration};

use futures::prelude::*;
use futures_timer::Delay;
use substrate_casper_primitives::CasperApi;

fn smoke() {
	println!("in smoke");
	let dur = Duration::from_millis(10000);
	let start = Instant::now();
	let timeout = Delay::new(dur);
	timeout.wait().unwrap();
}


pub fn start_casper() {
	println!("start");
	std::thread::spawn(|| {
		loop {
			smoke();
		}
	});
	println!("over");
}


