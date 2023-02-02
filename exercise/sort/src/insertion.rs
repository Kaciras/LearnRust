/// 插入排序，遍历数组的每个元素，
pub fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
	for i in 1..array.len() {
		let v = array[i];
		for j in (0..i).rev() {
			if v >= array[j] {
				break;
			}
			(array[j + 1], array[j]) = (array[j], v);
		}
	}
}
