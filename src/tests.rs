use crate::coordinates::AxialCoord;

#[test]
fn travel() {
	let tests = [
		AxialCoord::default(),
		AxialCoord::new(1, -1),
	];
	let mut results = Vec::new();
	for test in tests {
		let pixel = test.to_pixel();
		results.push((pixel.x, pixel.y));
	}
	assert_eq!(vec![(0., 0.), (1.5, -0.8660254037844386)], results);
}
#[test]
fn ring() {
	let center = AxialCoord::default();
	let ring = center.to_ring(1);
	assert_eq!(
		&[
			AxialCoord::new(1, -1),
			AxialCoord::new(1, 0),
			AxialCoord::new(0, 1),
			AxialCoord::new(-1, 1),
			AxialCoord::new(-1, 0),
			AxialCoord::new(0, -1),
		],
		ring.as_slice()
	)
}
#[test]
fn spiral() {
	let center = AxialCoord::default();
	let spiral = center.to_spiral(1);
	assert_eq!(
		&[
			center,
			AxialCoord::new(1, -1),
			AxialCoord::new(1, 0),
			AxialCoord::new(0, 1),
			AxialCoord::new(-1, 1),
			AxialCoord::new(-1, 0),
			AxialCoord::new(0, -1),
		],
		spiral.as_slice()
	)
}
#[test]
fn endless_spiral() {
	let center = AxialCoord::default();
	let spiral_iter = center.iter_spiral();
	let results = spiral_iter.take(9).collect::<Vec<_>>();
	assert_eq!(
		&[
			center,
			AxialCoord::new(1, -1),
			AxialCoord::new(1, 0),
			AxialCoord::new(0, 1),
			AxialCoord::new(-1, 1),
			AxialCoord::new(-1, 0),
			AxialCoord::new(0, -1),
			AxialCoord::new(1, -2),
			AxialCoord::new(2, -2),
		],
		results.as_slice()
	)
}
