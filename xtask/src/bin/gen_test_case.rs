use std::{fs, iter::repeat_with, path::Path};

fn main() {
	let tmp_dir = Path::new("tmp");
	if !tmp_dir.exists() {
		fs::create_dir(tmp_dir).unwrap();
	}
	let file = tmp_dir.join("test_case.txt");
	write(&file);
}

fn rand_str(len: usize) -> String {
	repeat_with(fastrand::alphanumeric).take(len).collect()
}

fn rand_str_arr(len: usize) -> Vec<String> {
	let mut arr = Vec::new();
	for _ in 0..100 {
		arr.push(rand_str(len));
	}
	arr
}

fn write(file: &Path) {
	let mut test_case = Vec::new();

	for len in [4, 8, 16, 32, 64, 128, 256, 512, 1024] {
		test_case.append(&mut rand_str_arr(len));
	}

	fs::write(file, test_case.join("\n")).unwrap();
}
