use std::mem::swap;

/// 分桶基数排序，是最直接的实现方案。
pub fn bucket(array: &mut [u32], bits: u32) {
    let mask = (0..bits).fold(0u32, |acc, _| (acc << 1) + 1);
    for offset in (0..32).step_by(bits as usize) {
        group_sort(array, mask, offset);
    }
}

pub fn group_sort(array: &mut [u32], mask: u32, offset: u32) {
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
        for value in bucket {
            array[index] = value;
            index += 1;
        }
    }
}

/// 双数组基数排序，通过统计每组基数的数量来确定起始位置，从而排序每组。
/// 实际上等于在数组内划分范围作为桶。
///
/// 该算法使用的内存低于分桶，是更好的方案。
pub fn double_array(array: &mut [u32], bits: u32) {
    let mask = (0..bits).fold(0u32, |acc, _| (acc << 1) + 1);

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

    let mut aux= vec![0; array.len()];
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

pub fn sort_aux(array: &mut [u32], aux: &mut [u32], mask: u32, offset: u32) {
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
