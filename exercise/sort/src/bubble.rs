/// 冒泡排序，
pub fn sort<T: PartialOrd>(array: &mut [T]) {

	// 正向遍历
	let end = array.len();
	for i in 1..end {
	    for j in 0..end - i {

	// let end = array.len() - 1;
	// for i in 0..array.len() {
	// 	for j in 0..i {
			if array[j] > array[j + 1] {
				array.swap(j, j + 1)
			}
		}
	}
}
