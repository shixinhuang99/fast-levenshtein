mod js_int;

use std::collections::HashMap;

use js_int::JsInt;

pub fn distance<T, P>(a: T, b: P) -> usize
where
	T: AsRef<str>,
	P: AsRef<str>,
{
	let mut a = a.as_ref();
	let mut b = b.as_ref();

	if a == b {
		return 0;
	}

	let mut a_len = a.chars().count();
	let mut b_len = b.chars().count();

	if a_len < b_len {
		(a, b) = (b, a);
		(a_len, b_len) = (b_len, a_len);
	}

	if b_len == 0 {
		return a_len;
	}

	if a_len <= 32 {
		myers_32(a, b)
	} else {
		myers_x(a, b)
	}
}

fn myers_32(a: &str, b: &str) -> usize {
	let a_len = a.chars().count();
	let lst = JsInt::new(1) << (a_len - 1);
	let mut peq: HashMap<char, JsInt> = HashMap::new();
	let mut pv: JsInt = JsInt::new(-1);
	let mut mv: JsInt = JsInt::new(0);
	let mut sc = a_len;

	let mut i = a_len;
	for char_a in a.chars().rev() {
		i -= 1;
		peq.entry(char_a)
			.and_modify(|v| {
				*v |= JsInt::new(1) << i;
				v.u32_assign();
			})
			.or_insert_with(|| {
				let mut v = JsInt::new(1) << i;
				v.u32_assign();
				v
			});
	}

	for char_b in b.chars() {
		let mut eq = *peq.get(&char_b).unwrap_or(&JsInt::new(0));
		let xv = eq | mv;
		eq |= ((eq & pv) + pv) ^ pv;
		mv |= !(eq | pv);
		pv &= eq;
		if mv & lst != 0 {
			sc += 1;
		}
		if pv & lst != 0 {
			sc -= 1;
		}
		mv = (mv << 1) | 1;
		pv = (pv << 1) | !(xv | mv);
		mv &= xv;
	}

	sc
}

fn myers_x(b: &str, a: &str) -> usize {
	let a_len = a.chars().count();
	let b_len = b.chars().count();
	let mut peq: HashMap<char, JsInt> = HashMap::new();
	let mut mhc: Vec<JsInt> = Vec::new();
	let mut phc: Vec<JsInt> = Vec::new();
	let mut sc = b_len;

	for _ in 0..ceil_divide(a_len, 32) {
		mhc.push(JsInt::new(0));
		phc.push(JsInt::new(-1));
	}

	let vsize = ceil_divide(b_len, 32);
	for j in 0..vsize {
		let mut pv: JsInt = JsInt::new(-1);
		let mut mv: JsInt = JsInt::new(0);
		let start = j * 32;
		let vlen = if j == vsize - 1 {
			b_len.min(32) + start
		} else {
			(b_len - start).min(32) + start
		};

		for (k, char_b) in b.chars().skip(start).take(vlen - start).enumerate()
		{
			let k = k + start;

			peq.entry(char_b)
				.and_modify(|v| {
					*v |= JsInt::new(1) << k;
					v.u32_assign();
				})
				.or_insert_with(|| {
					let mut v = JsInt::new(1) << k;
					v.u32_assign();
					v
				});
		}

		for (i, char_a) in a.chars().enumerate() {
			let eq = *peq.get(&char_a).unwrap_or(&JsInt::new(0));
			let pb = (phc[i / 32] >> i) & 1;
			let mb = (mhc[i / 32] >> i) & 1;
			let xv = eq | mv;
			let xh = ((((eq | mb) & pv) + pv) ^ pv) | eq | mb;
			let mut ph = mv | !(xh | pv);
			let mut mh = pv & xh;
			if j == vsize - 1 {
				sc += ph >> (b_len - 1) & 1;
				sc -= mh >> (b_len - 1) & 1;
			}
			if ((ph >> 31) ^ pb) != 0 {
				phc[i / 32] ^= JsInt::new(1) << i;
			}
			if ((mh >> 31) ^ mb) != 0 {
				mhc[i / 32] ^= JsInt::new(1) << i;
			}
			ph = (ph << 1) | pb;
			mh = (mh << 1) | mb;
			pv = mh | !(xv | ph);
			mv = ph & xv;
		}

		peq.clear();
	}

	sc
}

fn ceil_divide(a: usize, b: usize) -> usize {
	(a + b - 1) / b
}
