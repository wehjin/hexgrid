use std::fs::File;
use std::io::Write;

use handlebars::Handlebars;
use serde_json::json;

const CELL_SIZE: usize = 100usize;
const FONT_SIZE: usize = CELL_SIZE * 7 / 10;


const SANDBOX: &'static str = r#"
<svg width="300" height="100" fill="lavender" xmlns="http://www.w3.org/2000/svg">
	<rect width="100" height="100" x="0" y="0" rx="5" ry="5" fill="Thistle" />
	<rect width="98" height="100" x="101" y="0" rx="5" ry="5" fill="Lavender" />
	<rect width="100" height="100" x="200" y="0" rx="5" ry="5" fill="PeachPuff" />
	<text x="50,150,250" y="52"  dominant-baseline="central" text-anchor="middle" font-size="70" fill="red">
		新思早
	</text>
</svg>
"#;

fn main() -> anyhow::Result<()> {
	let reg = Handlebars::new();
	let svg = SANDBOX.to_string();
	println!("{}", &svg);
	write_file(svg.as_str())?;
	Ok(())
}

const CELL: &'static str = r#"
<svg width="200" height="100" fill="lavender" xmlns="http://www.w3.org/2000/svg">
	<rect width="200" height="100" x="0" y="0" rx="5" ry="5" fill="thistle" />
	<text x="50" y="52" dominant-baseline="central" text-anchor="middle" font-size="{{font-size}}" fill="red">
		{{glyph}}
	</text>
</svg>
"#;
fn cell_svg(glyph: impl AsRef<str>, reg: &Handlebars) -> anyhow::Result<String> {
	let glyph = glyph.as_ref().chars().next().unwrap();
	let data = json!({
		"font-size": FONT_SIZE,
		"glyph": glyph,
	});
	let svg = reg.render_template(CELL, &data)?;
	Ok(svg)
}

fn write_file(svg: &str) -> anyhow::Result<()> {
	let mut file = File::create("examples/web/assets/spiral.svg")?;
	file.write_all(svg.as_bytes())?;
	Ok(())
}
