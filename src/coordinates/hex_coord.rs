use std::ops::{Add, Mul};
use crate::coordinates::pixel_coord::PixelCoord;
use crate::HEX_SIZE;
use crate::direction::HexDirection;
use crate::spiral::HexSpiral;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct HexCoord {
	pub q: i32,
	pub r: i32,
}

impl HexCoord {
	pub const ORIGIN: Self = HexCoord::new(0, 0);
	pub const fn new(q: i32, r: i32) -> Self { Self { q, r } }
	pub fn s(&self) -> i32 { -self.q - self.r }
	pub fn to_pixel(&self) -> PixelCoord {
		let q = self.q as f32;
		let r = self.r as f32;
		let x = HEX_SIZE * (3. / 2. * q);
		let y = HEX_SIZE * (f32::sqrt(3.) / 2. * q + f32::sqrt(3.) * r);
		return PixelCoord { x, y };
	}
	pub fn to_ring(&self, radius: usize) -> Vec<Self> {
		const MOVEMENTS: [HexDirection; 6] = [
			HexDirection::South,
			HexDirection::SouthWest,
			HexDirection::NorthWest,
			HexDirection::North,
			HexDirection::NorthEast,
			HexDirection::SouthEast,
		];
		let mut results = vec![];
		let mut hex = *self + (MOVEMENTS[4].to_vector() * radius as i32);
		for travel in &MOVEMENTS[0..6] {
			let travel_vector = travel.to_vector();
			for _ in 0..radius {
				results.push(hex);
				hex = hex + travel_vector;
			}
		}
		results
	}
	pub fn to_spiral(&self, outer_radius: usize) -> Vec<Self> {
		let mut results = vec![*self];
		for radius in 1..=outer_radius {
			let ring = self.to_ring(radius);
			results.extend(ring);
		}
		results
	}

	pub fn iter_spiral(&self) -> impl Iterator<Item=HexCoord> {
		HexSpiral::new(*self)
	}
}

impl Add<HexCoord> for HexCoord {
	type Output = Self;

	fn add(self, rhs: HexCoord) -> Self::Output {
		let q = self.q + rhs.q;
		let r = self.r + rhs.r;
		Self { q, r }
	}
}

impl Mul<i32> for HexCoord {
	type Output = Self;

	fn mul(self, rhs: i32) -> Self::Output {
		Self { q: self.q * rhs, r: self.r * rhs }
	}
}