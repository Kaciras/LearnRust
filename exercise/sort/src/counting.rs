/// 计数排序，O(n) 级别的算法，对数据有要求，必须能够映射到有序的数字。
/// 原理就是开一个与值域同样大小的数组，统计每个元素出现的次数，完了根据次数再添加元素即可。
/// 
/// 计数排序等价于基数为最大值的基数排序。
/// 该算法拥有极快的速度，但需要消耗数据值域大小的数组空间。
pub fn sort(array: &mut [u32], max: u32) {
	let mut counts = vec![0; max as usize];
	for v in array.as_ref() {
		counts[*v as usize] += 1;
	}
	let mut i = 0;
	for k in 0..counts.len() {
		let s = i;
		i += counts[k];
		array[s..i].fill(k as u32);
	}
}
