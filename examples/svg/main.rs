use std::fs::File;
use std::io::Write;

use handlebars::Handlebars;
use serde_json::json;

use hexgrid::Grid;

pub const SANDBOX: &'static str = r#"
<svg width="300" height="200" fill="lavender" xmlns="http://www.w3.org/2000/svg">
	<rect width="100" height="100" x="0" y="0" rx="5" ry="5" fill="Thistle" />
	<rect width="98" height="100" x="101" y="0" rx="5" ry="5" fill="Lavender" />
	<rect width="100" height="100" x="200" y="0" rx="5" ry="5" fill="PeachPuff" />
	<text x="50,150,250" y="52,152,52"  dominant-baseline="central" text-anchor="middle" font-size="70" fill="red">
		新思早
	</text>
</svg>
"#;

const CONTENT: &'static str = r#"
<svg width="{{board-size}}" height="{{board-size}}" xmlns="http://www.w3.org/2000/svg">
	<rect width="{{board-size}}" height="{{board-size}}" x="0" y="0" rx="5" ry="5" fill="Thistle" />
	<circle cx="{{board-center}}" cy="{{board-center}}" r="{{half-spacing}}" fill="MintCream"/>
	<rect width="98" height="{{spacing}}" x="101" y="0" rx="5" ry="5" fill="Lavender" />
	<rect width="{{spacing}}" height="{{spacing}}" x="200" y="0" rx="5" ry="5" fill="PeachPuff" />
	<text dominant-baseline="central" text-anchor="middle" fill="SeaGreen"
		x="{{x-list}}"
		y="{{y-list}}"
		font-size="{{font-size}}"
	>
	{{glyphs}}
	</text>
</svg>
"#;
fn content(glyphs: impl AsRef<str>, hb: &Handlebars) -> anyhow::Result<String> {
	let glyphs = glyphs.as_ref();
	let grid = Grid::new(glyphs.chars().count());
	let spacing = 100;
	let half_spacing = spacing / 2;
	let font_size = spacing * 7 / 10;
	let (board_size, board_coords) = grid.to_board_coords(spacing);
	let board_center = board_size / 2;
	let x_list = board_coords.iter().map(|it| it.0.to_string()).collect::<Vec<_>>().join(",");
	let y_list = board_coords.iter().map(|it| it.1.to_string()).collect::<Vec<_>>().join(",");
	let data = json!({
		"board-size": board_size,
		"board-center": board_center,
		"spacing": spacing,
		"half-spacing": half_spacing,
		"font-size": font_size,
		"glyphs" : glyphs,
		"x-list": x_list,
		"y-list": y_list,
	});
	let out = hb.render_template(CONTENT, &data)?;
	println!("CONTENT: {}", out);
	Ok(out)
}

fn main() -> anyhow::Result<()> {
	let hb = Handlebars::new();
	let glyphs = "1234五六七八九十";
	let content = content(glyphs, &hb)?;
	write_file(content.as_str())?;
	Ok(())
}

fn write_file(svg: &str) -> anyhow::Result<()> {
	let mut file = File::create("examples/web/assets/spiral.svg")?;
	file.write_all(svg.as_bytes())?;
	Ok(())
}
