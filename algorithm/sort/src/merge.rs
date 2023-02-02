/// 归并排序，将两半分别排序，然后合并，分半操作一直递归到两个元素以下。
/// 该算法有稳定，简单，以及 Nlog2(N) 的性能，缺点是需要额外的内存。
pub fn sort<T: PartialOrd + Copy + Default>(array: &mut [T]) {

	// 辅助数组的在最坏的情况下必定要复制一次原始数组。因为每次归并都会交换俩数组,
	// 所以循环实现可以在最后检查下，偶数次无需复制,递归无论如何都要复制。
	// let mut aux: Vec<T> = vec![Default::default(); array.len()];

	let mut aux = array.to_vec();
	merge(aux.as_mut_slice(), array);
}

fn merge<T: PartialOrd + Copy + Default>(src: &mut [T], dest: &mut [T]) {
	let middle = src.len() >> 1;

	if src.len() > 2 {
		merge(&mut dest[..middle], &mut src[..middle]);
		merge(&mut dest[middle..], &mut src[middle..]);
	}

	// 长度为 0 和 1 时仍能通过合并，没必要做检查。

	let mut left = 0;
	let mut lm = 0;
	let mut rm = middle;

	while left < src.len() {
		if lm == middle {
			dest[left] = src[rm];
			rm += 1;
		} else if rm == src.len() {
			dest[left] = src[lm];
			lm += 1;
		} else if src[lm] < src[rm] {
			dest[left] = src[lm];
			lm += 1;
		} else {
			dest[left] = src[rm];
			rm += 1;
		}
		left += 1;
	}
}
