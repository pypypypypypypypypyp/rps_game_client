use crate::prelude::*;

const TEXTURE_RES: u32 = 512;
const TEXTURE_SIZE: Vec2<usize> = Vec2{ x: 1, y: 1 };

pub const STRIDE: i32 = std::mem::size_of::<Vertex>() as i32;

#[derive(Debug,Copy,Clone)]
pub enum Tex {
	Color([f32; 4]),
	Texture(usize),
}

pub use self::Tex::*;

#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct Vertex {
	pos: Vec3<f32>,
	col: [f32; 4],
	uv: Vec2<f32>,
	blend: f32,
}

pub fn quad_uvs(n: usize) -> [Vec2<f32>; 6] {
	let (w, h) = (1.0 / TEXTURE_SIZE.x as f32, 1.0 / TEXTURE_SIZE.y as f32);
	let zz = vec2((n % TEXTURE_SIZE.x) as f32 / TEXTURE_SIZE.x as f32, 1.0 - h - (n / TEXTURE_SIZE.x) as f32 / TEXTURE_SIZE.y as f32);
	let zz = vec2(zz.x + 2.5 / TEXTURE_RES as f32, zz.y + 2.5 / TEXTURE_RES as f32); //pixel correction
	let (w, h) = (w - 5.0 / TEXTURE_RES as f32, h - 5.0 / TEXTURE_RES as f32); //more pixel correction
	//why does pixel correct require 5 times the normal amount?? are mipmaps not properly disabled?
	[
		zz,
		zz + vec2(w, 0.0),
		zz + vec2(w, h),
		zz,
		zz + vec2(0.0, h),
		zz + vec2(w, h),
	]
}

pub fn make_quad(pos: Vec3<f32>, mut size: Vec2<f32>, tex: Tex, trans: Mat2<f32>) -> [Vertex; 6] {
	let z = pos.z;
	let mut pos = vec2(pos.x, pos.y);
	if size.x < 0.0 {
		pos.x += size.x;
		size.x *= -1.0;
	}
	if size.y < 0.0 {
		pos.y += size.y;
		size.y *= -1.0;
	}
	let mut col = [0.0; 4];
	let mut blend = 0.0;
	let mut uvs = [Vec2::zero(); 6];
	match tex {
		Color(rgb) => col = rgb,
		Texture(n) => {
			uvs = quad_uvs(n);
			blend = 1.0;
		}
	}
	let size = size / 2.0;
	let pos = pos + size;
	[
		Vertex { pos: (pos - trans * size).extend(z), col, uv: uvs[0], blend,  },
		Vertex { pos: (pos + trans * vec2(size.x, -size.y)).extend(z), col, uv: uvs[1], blend, },
		Vertex { pos: (pos + trans * size).extend(z), col, uv: uvs[2], blend, },
		Vertex { pos: (pos - trans * size).extend(z), col, uv: uvs[3], blend, },
		Vertex { pos: (pos + trans * vec2(-size.x, size.y)).extend(z), col, uv: uvs[4], blend, },
		Vertex { pos: (pos + trans * size).extend(z), col, uv: uvs[5], blend, },
	]
}

pub fn quad(v: &mut Vec<Vertex>, pos: Vec3<f32>, size: Vec2<f32>, tex: Tex) {
	v.extend_from_slice(&make_quad(pos, size, tex, Mat2::ident()));
}

/*pub fn transformed_quad(v: &mut Vec<Vertex>, pos: Vec3<f32>, size: Vec2<f32>, tex: Tex, trans: Mat2<f32>) {
	v.extend_from_slice(&make_quad(pos, size, tex, trans));
}*/

pub fn draw_string(v: &mut Vec<Vertex>, mut pos: Vec3<f32>, size: Vec2<f32>, s: String) {
	for c in s.chars() {
		v.extend_from_slice(&char_uvs(c, make_quad(pos, size, Texture(0), Mat2::ident())));
		pos.x += size.x;
	}
}

const CHAR_SHEET_LENGTH: Vec2<usize> = Vec2{ x: 8, y: 5, };

fn char_uvs(c: char, mut v: [Vertex; 6]) -> [Vertex; 6] {
	let offset = match c {
		'0' => 0,
		'1' => 1,
		'2' => 2,
		'3' => 3,
		'4' => 4,
		'5' => 5,
		'6' => 6,
		'7' => 7,
		'8' => 8,
		'9' => 9,
		'a' => 10,
		'b' => 11,
		'c' => 12,
		'd' => 13,
		'e' => 14,
		'f' => 15,
		'g' => 16,
		'h' => 17,
		'i' => 18,
		'j' => 19,
		'k' => 20,
		'l' => 21,
		'm' => 22,
		'n' => 23,
		'o' => 24,
		'p' => 25,
		'q' => 26,
		'r' => 27,
		's' => 28,
		't' => 29,
		'u' => 30,
		'v' => 31,
		'w' => 32,
		'x' => 33,
		'y' => 34,
		'z' => 35,
		'.' => 36,
		':' => 37,
		' ' => 38,
		'/' => 39,
		_ => 0,
	};
	let s = CHAR_SHEET_LENGTH;
	let offset = vec2(offset % s.x, offset / s.x).f32() / TEXTURE_SIZE.f32();
	v.iter_mut().map(|v| v.uv = (v.uv + offset) / s.f32()).last();
	v
}
