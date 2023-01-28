use rstest::rstest;
use rstest_reuse::{self, *};

mod insertion;
mod bubble;
mod selection;

type SortFn = fn(&mut [u32]) -> ();

/// 因为 Slice::is_sorted 还是实验功能，需要 nightly 版本，所以就自己写了。
fn is_sorted<T>(iterable: &Vec<T>) -> bool where T: PartialOrd {
	return iterable.windows(2).all(|w| w[0] <= w[1]);
}

#[cfg(test)]
mod tests {
	use rand::{distributions::Uniform, Rng};

	use super::*;

	#[template]
	#[rstest]
	#[case(selection::sort)]
	#[case(bubble::sort)]
	fn algorithms(#[case] algorithm: SortFn) {}

	#[apply(algorithms)]
	fn empty(#[case] algorithm: SortFn) {
		let mut array: [u32; 0] = [];
		algorithm(&mut array);
	}

	#[apply(algorithms)]
	fn single(#[case] algorithm: SortFn) {
		let mut array= [8964];
		algorithm(&mut array);
		assert_eq!(array, [8964]);
	}

	#[apply(algorithms)]
	fn odd(#[case] algorithm: SortFn) {
		let range = Uniform::from(0..1000);
		let mut array: Vec<u32> = rand::thread_rng().sample_iter(range).take(10).collect();

		algorithm(array.as_mut_slice());
		// println!("{:?}", array);
		assert!(is_sorted(&array));
	}

	#[apply(algorithms)]
	fn even(#[case] algorithm: SortFn) {
		let range = Uniform::from(0..1000);
		let mut array: Vec<u32> = rand::thread_rng().sample_iter(range).take(11).collect();
		algorithm(array.as_mut_slice());
		// println!("{:?}", array);
		assert!(is_sorted(&array));
	}
}
