/// 双指针快排，
pub fn sort<T: Ord + Copy>(array: &mut [T]) {
	if array.len() < 2 {
		return;
	}

	// 当中轴为最大或最小值时，每次只能分割出一个元素，相当于退化成了选择排序。
	// 理想的情况是分割成两部分，所以要尽量选择中等大小的元素作为轴。
	middle_3(array);

	let pivot = array[0];
	let mut left = 1;
	let mut right = array.len() - 1;

	while left <= right {
		// 必须是小于等于，因为需要跳过相等的。
		if array[left] <= pivot {
			left += 1;
		} else if array[right] >= pivot {
			right -= 1;
		} else {
			array.swap(left, right);
			left += 1;
			right -= 1;
		}
	}

	array.swap(right, 0);

	if left > 2 {
		sort(&mut array[..left - 1]);
	}
	if right < array.len() - 2 {
		sort(&mut array[right + 1 ..]);
	}
}

/// 三数取中，该函数还会把中间大小的数交换到数组头部。
pub fn middle_3<T: Ord + Copy>(array: &mut [T]) {
	let m = array.len() >> 1;
	let e = array.len() - 1;
	let b = array[m];

	match (array[0] < b, b < array[e]) {
		(false, true) => {},
		(true, false) => array.swap(0, e),
		_ => array.swap(0, m), // 正序和反序时中间的都是 m。
	};
}
