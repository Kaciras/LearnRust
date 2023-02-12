/*
 * 混合排序算法，指组合使用多种基本算法，因为不同算法再不同的条件下有不同的性能，所以组合使用更好。
 */

use std::cmp::Ordering;

use crate::{insertion, quick};

/// 快排 + 插排的组合体，根据插排比快排在小块下对性能更好。
/// 另外比起 quick::sort，本函数还采用了三路划分。
pub fn quick_insert<T: Ord + Copy>(array: &mut [T]) {
	let length = array.len();

	// Rust 的插排也是一个个移动的，没用希尔或二分。
	if length < 23 {
		return insertion::sort(array);
	}

	quick::middle_3(array);
	let pivot = array[0];

	// 这里跟 radix::quick 差不多，详细的注释见那边。
	let mut mid = 0usize;
	let mut left = 1;
	let mut right = length - 1;

	while left <= right {
		match array[left].cmp(&pivot) {
			Ordering::Less => {
				array.swap(mid, left);
				mid += 1;
				left += 1;
			}
			Ordering::Greater => {
				array.swap(right, left);
				right -= 1;
			}
			Ordering::Equal => left += 1
		}
	}

	quick_insert(&mut array[..mid]);
	quick_insert(&mut array[left..]);
}
