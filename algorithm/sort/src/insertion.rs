/// 插入排序，遍历数组的每个元素，把前面更大的元素挨个后挪。
/// 其结果就是让该元素插入到前面合适的位置，使前面的部分有序。
pub fn sort<T: Ord + Copy>(array: &mut [T]) {
	insert(array, 1);
}

/// 希尔排序是插入排序的一种改进版本，针对插入排序的两个特性：
/// * 数组接近有序时，插入排序速度很快。
/// * 数组较乱时，插入排序由于需要挨个移动，速度又很慢。
///
/// 故使用大步长将数组排到接近有序，再用小步长进一步整理，最后步长为 1 等于插排。
/// 这样大步长下元素能移动很远，小步长下又能享受插排再接近有序时的高性能。
pub fn shell<T: Ord + Copy>(array: &mut [T]) {
	let mut n = 1;
	// 2**k-1 步长下复杂度为 O(n**(3 / 2))，虽不是最佳，但胜在简单。
	while n < array.len() {
		n = n * 3 + 1;
	}
	while n >= 3 {
		n /= 3;
		insert(array, n);
	}
}

fn insert<T: Ord + Copy>(array: &mut [T], span: usize) {
	// 在 span 跨度下，第一个元素已经算是有序的，所以从 1 * span 开始。
	for i in span..array.len() {
		let v = array[i];

		// 注意 rev() 和 step_by() 的先后顺序不能搞错。
		for k in (0..=i - span).rev().step_by(span) {
			if v >= array[k] {
				break;
			}
			// 当前元素跟前面的交换，相当于向前移动了。
			(array[k + span], array[k]) = (array[k], v);
		}
	}
}

/// 插入排序中当前元素之前的部分已经有序，所以可以通过二分快速找出目标位置。
pub fn binary<T: Ord + Copy>(array: &mut [T]) {
	for i in 1..array.len() {
		let v = array[i];

		let mut lo = 0;
		let mut hi = i;
		while lo < hi {
			let mid = (hi + lo) >> 1;
			if v < array[mid] {
				hi = mid;
			} else {
				lo = mid + 1;
			}
		}

		array.copy_within(lo..i, lo + 1);
		array[lo] = v;
	}
}
