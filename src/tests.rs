use crate::HexCoord;

#[test]
fn travel() {
	let tests = [
		HexCoord::default(),
		HexCoord::new(1, -1),
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
	let center = HexCoord::default();
	let ring = center.to_ring(1);
	assert_eq!(
		&[
			HexCoord::new(1, -1),
			HexCoord::new(1, 0),
			HexCoord::new(0, 1),
			HexCoord::new(-1, 1),
			HexCoord::new(-1, 0),
			HexCoord::new(0, -1),
		],
		ring.as_slice()
	)
}
#[test]
fn spiral() {
	let center = HexCoord::default();
	let spiral = center.to_spiral(1);
	assert_eq!(
		&[
			center,
			HexCoord::new(1, -1),
			HexCoord::new(1, 0),
			HexCoord::new(0, 1),
			HexCoord::new(-1, 1),
			HexCoord::new(-1, 0),
			HexCoord::new(0, -1),
		],
		spiral.as_slice()
	)
}
#[test]
fn endless_spiral() {
	let center = HexCoord::default();
	let spiral_iter = center.iter_spiral();
	let results = spiral_iter.take(9).collect::<Vec<_>>();
	assert_eq!(
		&[
			center,
			HexCoord::new(1, -1),
			HexCoord::new(1, 0),
			HexCoord::new(0, 1),
			HexCoord::new(-1, 1),
			HexCoord::new(-1, 0),
			HexCoord::new(0, -1),
			HexCoord::new(1, -2),
			HexCoord::new(2, -2),
		],
		results.as_slice()
	)
}
