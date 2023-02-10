/// 选择排序，逐渐缩小遍历范围，在每次遍历找出最大和最小值放到两端。
pub fn sort<T: Ord>(array: &mut [T]) {

	// 每次搞定两个元素，所以只用循环一半即可。
	for i in 0..array.len() >> 1 {
		let e = array.len() - i;
		select(&mut array[i..e]);
	}
}

fn select<T: Ord>(array: &mut [T]) {
	let mut min = 0;
	let mut max = array.len() - 1;

	for j in 0..array.len() {
		if array[j] < array[min] { min = j; }
		if array[j] > array[max] { max = j; }
	}

	/*
	 * 如果最大的数在 0，则它会被最小值交换影响，需调整下。
	 * 举例:
	 * 原始数组 [3,1,2] min=1 max=0
	 * 交换小值 [1,3,2] 此时最大值位置变为 1 而不是 0.
	 */
	if 0 == max { max = min; }

	array.swap(0, min);
	array.swap(array.len() - 1, max);
}
