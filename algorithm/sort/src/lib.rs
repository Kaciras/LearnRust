use rstest::{fixture, rstest};
use rstest_reuse::{self, *};

pub mod insertion;
pub mod bubble;
pub mod heap;
pub mod merge;
pub mod quick;
pub mod selection;

pub mod counting;
pub mod radix;

pub type SortFn = fn(&mut [u32]) -> ();

const VALUE_RANGE: u32 = 1000;

pub fn bound_counting(array: &mut [u32]) {
	counting::sort(array, VALUE_RANGE)
}

pub fn bound_radix_grouping(array: &mut [u32]) {
	radix::bucket(array, 4);
}

pub fn bound_radix_array(array: &mut [u32]) {
	radix::counting(array, 4);
}

#[cfg(test)]
mod tests {
	use rand::{distributions::Uniform, Rng};

	use super::*;

	#[fixture]
	fn nums(#[default(10)] length: usize) -> Vec<u32> {
		return rand::thread_rng()
			.sample_iter(Uniform::from(0..VALUE_RANGE))
			.take(length).collect();
	}

	#[template]
	#[rstest]
	// #[case::counting(bound_counting)]
	// #[case::radix_counting(bound_radix_array)]
	// #[case::radix_group(bound_radix_grouping)]
	// #[case::heap(heap::sort)]
	// #[case::insertion(insertion::sort)]
	// #[case::shell(insertion::shell)]
	// #[case::binary(insertion::binary)]
	// #[case::selection(selection::sort)]
	// #[case::merge(merge::sort)]
	// #[case::quick(quick::sort)]
	// #[case::bubble(bubble::sort)]
	#[case::comb(bubble::comb)]
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

	#[apply(algorithms)]
	fn big_dataset(#[case] algorithm: SortFn, #[with(2048)] mut nums: Vec<u32>) {
		let mut expected = nums.clone();
		expected.sort_unstable();

		algorithm(nums.as_mut_slice());
		assert_eq!(expected, nums);

		nums.reverse();
		algorithm(nums.as_mut_slice());
		assert_eq!(expected, nums);
	}
}
