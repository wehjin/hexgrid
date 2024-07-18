use std::fs::File;
use std::io::Write;

use clap::Parser;
use handlebars::Handlebars;
use serde_json::json;

use hexgrid::Grid;

/// An example program to produce an SVG with text arranged in
/// a hex grid spiral.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Text to arrange in a spiral hex pattern.
	#[arg(default_value = "1234五六七八九十")]
	text: String,
	/// Number of times to greet.
	#[arg(short, long, default_value = "5000")]
	limit: usize,
	/// Distance between cells.
	#[arg(short, long, default_value = "100")]
	spacing: usize,
}
fn main() -> anyhow::Result<()> {
	let args = Args::parse();
	let text = args.text;
	let limit = args.limit;
	let spacing = args.spacing;

	let mut chars = text.chars().collect::<Vec<_>>();
	if limit < chars.len() {
		chars.truncate(limit);
	}
	let glyphs = chars.into_iter().collect::<String>();
	let hb = Handlebars::new();
	let content = content(&glyphs, spacing, &hb)?;
	let output_path = write_file(content.as_str())?;
	println!("Wrote to {}", &output_path);
	Ok(())
}

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
	<text dominant-baseline="central" text-anchor="middle" fill="SeaGreen" text-rendering="optimizeLegibility"
		x="{{x-list}}"
		y="{{y-list}}"
		font-size="{{font-size}}"
	>
	{{glyphs}}
	</text>
</svg>
"#;

fn content(glyphs: impl AsRef<str>, spacing: usize, hb: &Handlebars) -> anyhow::Result<String> {
	let glyphs = glyphs.as_ref();
	let grid = Grid::new(glyphs.chars().count());
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
	Ok(out)
}

fn write_file(svg: &str) -> anyhow::Result<String> {
	let path = "examples/web/assets/spiral.svg";
	let mut file = File::create(path)?;
	file.write_all(svg.as_bytes())?;
	Ok(path.to_string())
}
