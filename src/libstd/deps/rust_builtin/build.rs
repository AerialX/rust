extern crate gcc;

fn main() {
	let sources = &[
		"../../../rt/rust_builtin.c",
		"../../../rt/rust_android_dummy.c"
	];

	gcc::compile_library("librust_builtin.a", sources);
}
