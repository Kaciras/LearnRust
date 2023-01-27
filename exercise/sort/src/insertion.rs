pub fn insertion_sort(array: &mut [u32]) {
	for i in 0..array.len() {
		let mut min = i;
		let mut max = array.len() - i - 1;

		for j in i..array.len() - i {
			if array[j] < array[min] {
				min = j;
			}
			if array[j] > array[max] {
				max = j;
			}
		}
		if i == max {
			max = min; // 刚开始有可能在最小的数的位置上放的是最大的数，在交换后最大的数将在 min 位置。
		}
		array.swap(i, min);
		let e = array.len() - i;
		array.swap(e - 1, max);
	}
}

fn is_sorted<T>(iterable: &Vec<T>) -> bool where T: PartialOrd {
	return iterable.windows(2).all(|w| w[0] <= w[1]);
}

#[cfg(test)]
mod tests {
	use rand::{distributions::Uniform, Rng};

	use super::*;

	#[test]
	fn case_sensitive() {
		let range = Uniform::from(0..1000);
		let mut array: Vec<u32> = rand::thread_rng().sample_iter(range).take(10).collect();
		let mut array2: Vec<u32> = rand::thread_rng().sample_iter(range).take(11).collect();

		insertion_sort(array.as_mut_slice());
		insertion_sort(array2.as_mut_slice());
		println!("{:?}", array);
		println!("{:?}", array2);

		assert!(is_sorted(&array));
		assert!(is_sorted(&array2));
	}
}
