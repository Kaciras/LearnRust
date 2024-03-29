use std::cmp::Ordering;
use std::mem::swap;

/*
 * 基数排序，
 */

fn mask_low(bits: u32) -> u32 {
	return (0..bits).fold(0u32, |mask, _| (mask << 1) | 1);
}

fn mask_high(bits: u32) -> u32 {
	return (0..bits).fold(0u32, |mask, i| mask | 1 << (u32::BITS - i - 1));
}

/// 分桶基数排序，是最直接的实现方案。
pub fn bucket(array: &mut [u32], bits: u32) {
	let mask = mask_low(bits);
	for offset in (0..32).step_by(bits as usize) {
		group_sort(array, mask, offset);
	}
}

fn group_sort(array: &mut [u32], mask: u32, offset: u32) {
	let mut buckets = Vec::with_capacity(mask as usize + 1);
	for _ in 0..mask + 1 {
		buckets.push(Vec::<u32>::new())
	}

	for value in array.as_ref() {
		let radix = (value >> offset) & mask;
		buckets[radix as usize].push(*value);
	}

	let mut index = 0;
	for bucket in buckets {
		let b = index;
		index += bucket.len();

		// 复制要求两个切片大小相等，就不能宽松点嘛……
		array[b..index].clone_from_slice(bucket.as_slice());
	}
}

/// 双数组基数排序，通过统计每组基数的数量来确定起始位置，从而排序每组。
/// 实际上等于在数组内划分范围作为桶。
///
/// 该算法使用的内存低于分桶，是更好的方案。
pub fn counting(array: &mut [u32], bits: u32) {
	let mask = mask_low(bits);

	/*
	 * array 从外面传来的，其生命周期在调用方那，而 aux 仅到该函数结束。
	 * 它们的生命周期不一样，导致无法交换。
	 *
	 * 但实际上 array 是个指针，其在调用时就复制了，生存期是等于本地变量的。
	 *
	 * 解决办法就是重新创个本地变量，傻逼 Rust!
	 * https://stackoverflow.com/q/53835730/7065321
	 */
	let mut array = &mut *array;

	let mut aux = vec![0; array.len()];
	let mut aux = aux.as_mut_slice();

	for offset in (0..32).step_by(bits as usize) {
		sort_aux(array, aux, mask, offset);
		swap(&mut array, &mut aux);
	}

	// 若交换了奇数次，则最终结果在 aux 中，需要倒回去。
	if (32 - 1) / bits & 1 == 1 {
		array.clone_from_slice(aux);
	}
}

fn sort_aux(array: &mut [u32], aux: &mut [u32], mask: u32, offset: u32) {
	// 做累加需要第一个为 0 的虚拟元素，所以长度得多一个。
	let mut counts: Vec<usize> = vec![0; (mask + 2) as usize];

	// 第一步，统计每组基数出现的次数。
	for value in array.as_ref() {
		let radix = (value >> offset) & mask;
		counts[radix as usize + 1] += 1;
	}

	// 下一个基数组的起始位置就是前面数量的和。
	for i in 1..counts.len() {
		counts[i] += counts[i - 1];
	}

	// 然后把元素放到对应的分组内即可。
	for value in array.as_ref() {
		let radix = (value >> offset) & mask;
		let i = counts[radix as usize];
		aux[i] = *value;
		counts[radix as usize] = i + 1;
	}
}

/// 三路基数快排，是一种按基数从大到小的排序方法。对每次基数，都把数组划分为大于、等于和小于三部分，
/// 不相等的继续划分，而相等的那段进展到下一个基数。
///
/// 该算法的优势是无需额外空间，性能在某些数据上比基于比较的快排更好。
/// “某些数据”是指“整体比较”比“基数比较”慢的对象，比如字符串比较需要遍历，而比较单个字符则很快。
/// 用于数字的话说该算法由于比较次数多反而更慢。
///
/// 三路划分在基于比较的快排中不是必要的，但基数排序必须这么做。
pub fn quick(array: &mut [u32], bits: u32) {
	return partition(array, mask_high(bits), bits);
}

fn partition(array: &mut [u32], mask: u32, bits: u32) {
	if array.len() < 2 {
		return;
	}
	middle_3_with_mask(array, mask);
	let pivot = array[0] & mask;

	let mut mid = 0usize;            // 相等段的起始
	let mut left = 1;                // 相等段的结束 + 1
	let mut right = array.len() - 1; // 大于段的起始

	// 因为待排段在相等段和大于段之间，所以两者交叉时划分结束。
	while left <= right {
		let v = array[left] & mask;
		match v.cmp(&pivot) {
			// 相当于把 v 插入到小于段的末尾。
			Ordering::Less => {
				array.swap(mid, left);
				mid += 1;
				left += 1;
			}
			// 把 v 添加到大于段的开头。
			Ordering::Greater => {
				array.swap(right, left);
				right -= 1;
			}
			// v 正处于相等段的末尾，无需交换。
			Ordering::Equal => left += 1
		}
	}

	/*
	 * 【指针位置】
	 *
	 */

	partition(&mut array[..mid], mask, bits);
	partition(&mut array[left..], mask, bits);

	let mask = mask >> bits;
	if mask != 0 {
		partition(&mut array[mid..left], mask, bits);
	}
}

#[inline]
pub fn middle_3_with_mask(array: &mut [u32], mask: u32) {
	let m = array.len() >> 1;
	let e = array.len() - 1;
	let b = array[m] & mask;

	match (array[0] & mask < b, b < array[e] & mask) {
		(false, true) => {},
		(true, false) => array.swap(0, e),
		_ => array.swap(0, m), // 正序和反序时中间的都是 m。
	};
}
