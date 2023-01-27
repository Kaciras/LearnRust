mod bubble;
mod selection;

/// 因为 Slice::is_sorted 还是实验功能，需要 nightly 版本，所以就自己写了。
fn is_sorted<T>(iterable: &Vec<T>) -> bool where T: PartialOrd {
	return iterable.windows(2).all(|w| w[0] <= w[1]);
}

#[cfg(test)]
mod tests {
	use rand::{distributions::Uniform, Rng};

	use super::*;

	#[test]
	fn sa() {
		let range = Uniform::from(0..1000);
		let mut array: Vec<u32> = rand::thread_rng().sample_iter(range).take(10).collect();
		let mut array2: Vec<u32> = rand::thread_rng().sample_iter(range).take(11).collect();

		bubble::sort(array.as_mut_slice());
		bubble::sort(array2.as_mut_slice());
		println!("{:?}", array);
		println!("{:?}", array2);

		assert!(is_sorted(&array));
		assert!(is_sorted(&array2));
	}
}
