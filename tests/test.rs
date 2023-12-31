fn get_test_case() -> Vec<String> {
	use std::{fs, path::PathBuf};

	let file = PathBuf::from_iter(["tmp", "test_case.txt"]);
	let data = fs::read_to_string(file).unwrap();

	data.lines().map(|s| s.to_string()).collect()
}

#[test]
fn cross_validation() {
	let test_case = get_test_case();
	let mut iter = test_case.iter();

	while let (Some(a), Some(b)) = (iter.next(), iter.next_back()) {
		let actual = fast_levenshtein::distance(a, b);
		let expected = strsim::levenshtein(a, b);

		assert_eq!(actual, expected, "\n{}\n{}", a, b);
	}
}
