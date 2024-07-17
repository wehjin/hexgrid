use crate::coordinates::hex_coord::HexCoord;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum HexDirection {
	NorthEast,
	SouthEast,
	South,
	SouthWest,
	NorthWest,
	North,
}

impl HexDirection {
	pub const fn to_vector(&self) -> HexCoord {
		match self {
			HexDirection::South => HexCoord::new(0, 1),
			HexDirection::SouthWest => HexCoord::new(-1, 1),
			HexDirection::NorthWest => HexCoord::new(-1, 0),
			HexDirection::North => HexCoord::new(0, -1),
			HexDirection::NorthEast => HexCoord::new(1, -1),
			HexDirection::SouthEast => HexCoord::new(1, 0),
		}
	}
}