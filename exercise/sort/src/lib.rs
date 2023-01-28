use rstest::{fixture, rstest};
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

	#[fixture]
	fn rand_nums(#[default(10)] length: usize) -> Vec<u32> {
		return rand::thread_rng()
			.sample_iter(Uniform::from(0..1000))
			.take(length).collect();
	}

	#[template]
	#[rstest]
	#[case::selection(selection::sort)]
	#[case::bubble(bubble::sort)]
	fn algorithms(#[case] algorithm: SortFn) {}

	#[apply(algorithms)]
	fn empty(#[case] algorithm: SortFn) {
		let mut array: [u32; 0] = [];
		algorithm(&mut array);
		assert!(array.is_empty());
	}

	#[apply(algorithms)]
	fn single(#[case] algorithm: SortFn) {
		let mut array= [8964];
		algorithm(&mut array);
		assert_eq!(array, [8964]);
	}

	#[apply(algorithms)]
	fn odd(#[case] algorithm: SortFn, #[with(10)] mut rand_nums: Vec<u32>) {
		algorithm(rand_nums.as_mut_slice());
		// println!("{:?}", array);
		assert!(is_sorted(&rand_nums));
	}

	#[apply(algorithms)]
	fn even(#[case] algorithm: SortFn, #[with(11)] mut rand_nums: Vec<u32>) {
		algorithm(rand_nums.as_mut_slice());
		// println!("{:?}", array);
		assert!(is_sorted(&rand_nums));
	}
}
