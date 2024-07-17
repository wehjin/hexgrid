use crate::coordinates::hex_coord::HexCoord;

pub struct HexSpiral {
	center: HexCoord,
	ring_radius: usize,
	ring_iter: Option<Box<dyn Iterator<Item=HexCoord>>>,
}

impl Iterator for HexSpiral {
	type Item = HexCoord;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ring_radius == 0 {
			self.ring_radius = 1;
			self.ring_iter = Some(Box::new(self.center.to_ring(self.ring_radius).into_iter()));
			Some(self.center)
		} else {
			let mut ring_iter = self.ring_iter.take().expect("ring_iter");
			if let Some(item) = ring_iter.next() {
				self.ring_iter = Some(ring_iter);
				Some(item)
			} else {
				self.ring_radius += 1;
				let mut ring = self.center.to_ring(self.ring_radius);
				ring.rotate_right(self.ring_radius - 1);
				let mut ring_iter = Box::new(ring.into_iter());
				let item = ring_iter.next().expect("item in ring");
				self.ring_iter = Some(ring_iter);
				Some(item)
			}
		}
	}
}

impl HexSpiral {
	pub fn new(center: HexCoord) -> Self {
		HexSpiral { center, ring_radius: 0, ring_iter: None }
	}
}