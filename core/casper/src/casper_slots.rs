extern crate futures;
extern crate futures_timer;

use futures::prelude::*;
use std::{pin::Pin, time::{Duration, Instant}};
use futures_timer::Delay;


/// A stream that returns every time there is a new slot.
pub struct CasperSlots {
	last_slot: u64,
	slot_duration: u64,
	inner_delay: Delay,
}

impl CasperSlots {
	/// Create a new `Slots` stream.
	pub fn new(
		slot_duration: u64,
	) -> Self {
		CasperSlots {
			last_slot: 0,
			slot_duration,
			inner_delay: Delay::new_at(Instant::now()),
		}
	}
}

impl Stream for CasperSlots {
	type Item = u64;
	type Error = ();
	fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
		let slot_duration = self.slot_duration;

		(&mut self.inner_delay).wait();
        // &mut self.inner_delay.wait(); doesn't work.
		let ends_at = Instant::now() + Duration::from_millis(slot_duration);
		//	+ time_until_next(Duration::from_secs(timestamp), slot_duration);
		self.inner_delay = Delay::new_at(ends_at);
		self.last_slot = self.last_slot + 1;
		Ok(Async::Ready(Some(self.last_slot)))
	}
}
