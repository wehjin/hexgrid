use crate::coordinates::AxialCoord;
use crate::spiral::HexSpiral;

#[derive(Debug, Copy, Clone)]
pub struct Cell(AxialCoord);
#[derive(Debug, Clone)]
pub struct Grid {
	pub cells: Vec<Cell>,
	pub xy_min_max: MinMax,
}

impl Grid {
	pub fn len(&self) -> usize { self.cells.len() }
	pub fn board_size(&self, spacing: usize) -> usize {
		let size = self.xy_min_max.size_to_cover();
		(size * spacing as f32) as usize
	}
	pub fn to_board_coords(&self, spacing: usize) -> (usize, Vec<(f32, f32)>) {
		let board_size = self.board_size(spacing);
		let board_center = board_size as f32 / 2.;
		let spacing = spacing as f32;
		let coords = self.cells.iter()
			.map(|cell| {
				let (x, y) = cell.0.to_pixel().same_y();
				let board_x = board_center + (x * spacing);
				let board_y = board_center + (y * spacing);
				(board_x, board_y)
			})
			.collect()
			;
		(board_size, coords)
	}
}

impl Grid {
	pub fn new(size: usize) -> Self {
		let mut min_max = MinMax::new();
		let origin = AxialCoord::new(0, 0);
		let mut cells = vec![];
		let mut spiral = HexSpiral::new(origin);
		for _i in 0..size {
			let next = spiral.next().unwrap();
			cells.push(Cell(next));
			let pixel = next.to_pixel();
			min_max.update(pixel.x, pixel.y);
		}
		Grid { cells, xy_min_max: min_max }
	}
}

#[derive(Debug, Copy, Clone)]
pub struct MinMax {
	pub x: (f32, f32),
	pub y: (f32, f32),
}
impl MinMax {
	pub fn size_to_cover(&self) -> f32 {
		let half_width = {
			let (min, max) = self.x;
			let half_x = min.abs().max(max.abs()).ceil() + 1.;
			half_x
		};
		let half_height = {
			let (min, max) = self.y;
			let half_y = min.abs().max(max.abs()).ceil() + 1.;
			half_y
		};
		2. * half_width.max(half_height) - 1.
	}
}

impl MinMax {
	pub fn new() -> Self {
		Self { x: (0., 0.), y: (0., 0.) }
	}
	pub fn update(&mut self, x: f32, y: f32) {
		let (min, max) = self.x;
		self.x = (min.min(x), max.max(x));
		let (min, max) = self.y;
		self.y = (min.min(y), max.max(y));
	}
}

#[cfg(test)]
mod tests {
	use crate::Grid;

	#[test]
	fn board_size() {
		let counts = [1, 2, 3, 7, 8];
		let mut sizes = Vec::new();
		for count in counts {
			let grid = Grid::new(count);
			assert_eq!(grid.len(), count);
			sizes.push(grid.board_size(1));
		}
		assert_eq!(sizes, vec![1, 5, 5, 5, 7])
	}

	#[test]
	fn board_coords1() {
		let grid = Grid::new(1);
		assert_eq!(
			grid.to_board_coords(10),
			(10, vec![(5.0, 5.0)])
		)
	}
	#[test]
	fn board_coords2() {
		let grid = Grid::new(2);
		assert_eq!(
			grid.to_board_coords(10),
			(50, vec![(25.0, 25.0), (40.0, 16.339746)])
		);
	}
	#[test]
	fn board_coords4() {
		let grid = Grid::new(4);
		assert_eq!(
			grid.to_board_coords(10),
			(50, vec![(25.0, 25.0), (40.0, 16.339746), (40.0, 33.660255), (25.0, 42.320507)])
		);
	}
}

