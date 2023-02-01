use rstest::{fixture, rstest};
use rstest_reuse::{self, *};

pub mod insertion;
pub mod bubble;
pub mod merge;
pub mod selection;

pub mod counting;

type SortFn = fn(&mut [u32]) -> ();

const VALUE_RANGE: u32 = 1000;

fn bound_counting(array: &mut [u32]) {
	counting::sort(array, VALUE_RANGE)
}

#[cfg(test)]
mod tests {
	use rand::{distributions::Uniform, Rng};

	use super::*;

	#[fixture]
	fn rand_nums(#[default(10)] length: usize) -> Vec<u32> {
		return rand::thread_rng()
			.sample_iter(Uniform::from(0..VALUE_RANGE))
			.take(length).collect();
	}

	#[template]
	#[rstest]
	#[case::counting(bound_counting)]

	// #[case::insertion(insertion::sort)]
	// #[case::selection(selection::sort)]
	// #[case::merge(merge::sort)]
	// #[case::bubble(bubble::sort)]
	fn algorithms(#[case] algorithm: SortFn) {}

	#[apply(algorithms)]
	fn empty(#[case] algorithm: SortFn) {
		let mut array: [u32; 0] = [];
		algorithm(&mut array);
		assert!(array.is_empty());
	}

	#[apply(algorithms)]
	fn single(#[case] algorithm: SortFn) {
		let mut array = [896];
		algorithm(&mut array);
		assert_eq!(array, [896]);
	}

	#[apply(algorithms)]
	fn odd(#[case] algorithm: SortFn, #[with(10)] mut nums: Vec<u32>) {
		let mut expected = nums.clone();

		algorithm(nums.as_mut_slice());
		// println!("{:?}", nums);

		expected.sort();
		assert_eq!(expected, nums);
	}

	#[apply(algorithms)]
	fn even(#[case] algorithm: SortFn, #[with(11)] mut nums: Vec<u32>) {
		let mut expected = nums.clone();

		algorithm(nums.as_mut_slice());
		// println!("{:?}", array);

		expected.sort();
		assert_eq!(expected, nums);
	}
}
