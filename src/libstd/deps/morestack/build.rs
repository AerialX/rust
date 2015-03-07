use std::env;

extern crate gcc;

fn main() {
	let target = env::var("TARGET").unwrap();
	let arch = target.split("-").next().unwrap();

	let record_sp = format!("../../../rt/arch/{}/morestack.S", arch);
	let sources = &[
		&*record_sp,
	];

	gcc::compile_library("libmorestack.a", sources);
}
