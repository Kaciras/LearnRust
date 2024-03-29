/// 冒泡排序，一次比较两个元素，如果它们的顺序错误就把它们交换过来。
/// 由于大量的交换，导致该算法速度通常很慢。
pub fn sort<T: Ord>(array: &mut [T]) {
	let length = array.len();

	// 每次冒泡能够将一个最值移到一端，故 N-1 次必能排完。
	for i in 1..length {
		for j in 0..length - i {

	// let end = array.len() - 1;
	// for i in 0..array.len() {
	// 	for j in 0..i {
			if array[j] > array[j + 1] {
				array.swap(j, j + 1)
			}
		}
	}
}

/// 梳排序，跟希尔排序类似，使用不同的步长来做冒泡，避免某些情况下出现大量交换。
pub fn comb<T: Ord>(array: &mut [T]) {
	let mut step = array.len();

	// 另一种停止方法，一次遍历中未发现需要冒泡的即认为已经有序。
	let mut swapped = true;

	// 仅当步长为 1 时才能检查到每个元素，判定数组有序。
	while step > 1 || swapped {
		swapped = false;

		// 0.8 是一个常用的因子。
		if step > 1 {
			step = (step as f32 * 0.8) as usize;
		}

		let mut j = 0;
		while j + step < array.len() {
			if array[j] > array[j + step] {
				swapped = true;
				array.swap(j, j + step)
			}
			j += step;
		}

		// step_by 参数不能为 0，空数组需要额外的检查，所以它并不能替换 for-index 循环，
		// 但 Rust 的开发者却不这么认为。
		// for j in (0..array.len() - step).step_by(step)
	}
}
