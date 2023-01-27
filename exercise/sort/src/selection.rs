/// 选择排序逐渐缩小遍历范围，在每次遍历找出最大和最小值放到两端。
pub fn sort<T: PartialOrd>(array: &mut [T]) {

	// 每次搞定两个元素，所以只用循环一半即可。
	for i in 0..array.len() >> 1 {
		let mut min = i;
		let mut max = array.len() - i - 1;

		for j in i..array.len() - i {
			if array[j] < array[min] { min = j; }
			if array[j] > array[max] { max = j; }
		}

		// 刚开始有可能在最小的数的位置上放的是最大的数，在交换后最大的数将在 min 位置。
		if i == max {
			max = min;
		}

		array.swap(i, min);

		// 傻逼 Rust 到现在都搞不定在一行写同一变量的多个访问。
		let e = array.len() - i;
		array.swap(e - 1, max);
	}
}
