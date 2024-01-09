use napi_derive::napi;

#[napi]
pub fn distance(a: String, b: String) -> i64 {
	fast_levenshtein::distance(a, b) as i64
}
