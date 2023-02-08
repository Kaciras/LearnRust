pub fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
	if array.len() < 2 {
		return;
	}
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
