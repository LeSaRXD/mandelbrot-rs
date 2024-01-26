use image::{ImageBuffer, Rgb};

const WIDTH: u32 = 5000;
const HEIGHT: u32 = 5000;
const WIDTH_F64: f64 = WIDTH as f64;
const HEIGHT_F64: f64 = HEIGHT as f64;

#[derive(Debug, Clone)]
struct Complex(f64, f64);
impl From<(f64, f64)> for Complex {
	fn from(value: (f64, f64)) -> Self {
		Self(value.0, value.1)
	}
}
impl Complex {
	fn add(&mut self, other: &Complex) {
		self.0 += other.0;
		self.1 += other.1;
	}
	fn mul(&mut self, other: &Complex) {
		let new_real = self.0 * other.0 - self.1 * other.1;
		let new_comp = self.0 * other.1 + self.1 * other.0;
		self.0 = new_real;
		self.1 = new_comp;
	}
	fn dist_sqr(&self) -> f64 {
		self.0 * self.0 + self.1 * self.1
	}
}

fn main() {
	let mut buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			let c: Complex = pixels_to_coords((x, y), (-0.5, 0.0), (0.75, 0.75)).into();
			let iters = converges(&c, 100);
			buf.get_pixel_mut(x, y).0 = if iters == 0 {[0, 0, 0]} else {
				let mapped_iters = (1.0 / (iters as f32) * 255.0) as u8;
				[mapped_iters, 255 - mapped_iters, 255]
			};
		}
	}
	buf.save("mandelbrot.png").unwrap()
}

fn converges(c: &Complex, iterations: u16) -> u16 {
	let mut z = Complex(0.0, 0.0);
	for i in 1..=iterations {
		let z_n = z.clone();
		z.mul(&z_n); // calculating z_n+1
		z.add(c);
		if z.dist_sqr() > 4.0 {
			return i
		}
	}
	0
}

fn pixels_to_coords((x, y): (u32, u32), (off_x, off_y): (f64, f64), (scale_x, scale_y): (f64, f64)) -> (f64, f64) {
	((2.0 * (x as f64) / WIDTH_F64 - 1.0) / scale_x + off_x,
	(2.0 * (y as f64) / HEIGHT_F64 - 1.0) / scale_y + off_y)
}
