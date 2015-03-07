use std::env;

extern crate gcc;

fn main() {
	let target = env::var("TARGET").unwrap();
	let arch = target.split("-").next().unwrap();

	let record_sp = format!("../../../rt/arch/{}/record_sp.S", arch);
	let sources = &[
		&*record_sp,
		"../../../rt/rust_try.ll",
	];

	gcc::compile_library("librustrt_native.a", sources);
}
