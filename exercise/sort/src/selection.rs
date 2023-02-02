/// 选择排序，逐渐缩小遍历范围，在每次遍历找出最大和最小值放到两端。
pub fn sort<T: PartialOrd>(array: &mut [T]) {

	// 每次搞定两个元素，所以只用循环一半即可。
	for i in 0..array.len() >> 1 {
		let mut min = i;
		let mut max = array.len() - i - 1;

		for j in i..array.len() - i {
			if array[j] < array[min] { min = j; }
			if array[j] > array[max] { max = j; }
		}

		/*
		 * 如果最大的数在起始位置（i），则第一个交换后它在 min，需要把 max 也更新一下。
		 * 举例:
		 * 原始数组 [3,1,2] min=1 max=0
		 * 交换小值 [1,3,2] 此时最大值位置变为 1 而不是 0.
		 */
		if i == max {
			max = min;
		}

		array.swap(i, min);

		// 傻逼 Rust 到现在都搞不定在一行写同一变量的多个访问。
		let e = array.len() - i;
		array.swap(e - 1, max);
	}
}
