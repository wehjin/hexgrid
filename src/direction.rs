use crate::coordinates::AxialCoord;

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
	pub const fn to_vector(&self) -> AxialCoord {
		match self {
			HexDirection::South => AxialCoord::new(0, 1),
			HexDirection::SouthWest => AxialCoord::new(-1, 1),
			HexDirection::NorthWest => AxialCoord::new(-1, 0),
			HexDirection::North => AxialCoord::new(0, -1),
			HexDirection::NorthEast => AxialCoord::new(1, -1),
			HexDirection::SouthEast => AxialCoord::new(1, 0),
		}
	}
}