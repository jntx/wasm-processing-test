use wasm_bindgen::prelude::*;
use std::os::raw::c_void;
use std::slice;
use std::mem;

#[wasm_bindgen]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[wasm_bindgen]
pub fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[wasm_bindgen]
pub fn fill(pointer: *mut u8, max_width: usize, max_height: usize) {
	let byte_size = max_width * max_height * 4;
    let sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

	let w: f64 = max_width as f64;
	let h: f64 = max_height as f64;

	for x in 0..max_width {
		for y in 0..max_height {
			let pbuf = (x + y * max_width) * 4;

			let xf : f64 = x as f64;
			let yf : f64 = y as f64;
			let r: f64 = 128.0 + 128.0 * (((h - yf) + 100.0) / 15.0 * (xf + 100.0) / 16.0).cos();
			let g: f64 = 128.0 + 128.0 * ((yf + 70.0) / 20.0 * (300.0 - xf) / 20.0).sin();
			let b: f64 = 128.0 + 128.0 * ((-yf + 70.0) / 30.0 * ( -xf + 300.0) / 40.0).sin();

			sl[pbuf] = r as u8;
			sl[pbuf + 1] = g as u8;
			sl[pbuf + 2] = b as u8;
			sl[pbuf + 3] = 255;
		}
	}
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn wsin(alpha: f64) -> f64 {
	return alpha.sin();
}