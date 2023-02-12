/// 归并排序，将两半分别排序，然后合并，分半操作一直递归到两个元素以下。
/// 该算法有稳定，简单，以及 Nlog2(N) 的性能，缺点是需要额外的内存。
pub fn sort<T: Ord + Copy>(array: &mut [T]) {

	// 辅助数组的在最坏的情况下必定要复制一次原始数组。因为每次归并都会交换俩数组,
	// 所以循环实现可以在最后检查下，偶数次无需复制,递归无论如何都要复制。
	// let mut aux: Vec<T> = vec![Default::default(); array.len()];

	let mut aux = array.to_vec();
	merge(aux.as_mut_slice(), array);
}

fn merge<T: Ord + Copy>(src: &mut [T], dest: &mut [T]) {
	let length = src.len();
	let middle = length >> 1;

	if length > 2 {
		let (left_s, right_s) = src.split_at_mut(middle);
		let (left_d, right_d) = dest.split_at_mut(middle);
		merge(left_d, left_s);
		merge(right_d, right_s);
	}

	// 长度为 0 和 1 时仍能通过合并，没必要做检查。

	let mut left = 0;
	let mut right = middle;
	let mut i = 0;

	// 一边到头就不用再比较了，直接复制另一边到 dest 即可。
	while left < middle && right < length {
		if src[left] < src[right] {
			dest[i] = src[left];
			left += 1;
		} else {
			dest[i] = src[right];
			right += 1;
		}
		i += 1;
	}

	if left == middle {
		dest[i..].copy_from_slice(&mut src[right..]);
	} else if right == length {
		dest[i..].copy_from_slice(&mut src[left..middle]);
	}
}
