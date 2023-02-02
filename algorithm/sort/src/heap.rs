struct BigHeap<'scope, T: PartialOrd> {
	array: &'scope mut [T],
}

impl<'scope, T: PartialOrd> BigHeap<'scope, T> {
	fn popup(&mut self, index: usize) {
		if index == 0 {
			return;
		}
		let parent = (index - 1) >> 1;
		if self.array[index] > self.array[parent] {
			self.array.swap(index, parent);
			self.popup(parent);
		}
	}

	fn sink(&mut self, index: usize, length: usize) {
		let mut child = (index << 1) + 1;
		if child < length - 1 && self.array[child] < self.array[child + 1] {
			child += 1;
		}
		if child >= length {
			return;
		}
		if self.array[index] < self.array[child] {
			self.array.swap(index, child);
			self.sink(child, length);
		}
	}
}

pub fn sort<T: PartialOrd>(array: &mut [T]) {
	let length = array.len();
	let mut heap = BigHeap { array };
	for i in 1..length {
		heap.popup(i);
	}
	for i in (1..length).rev() {
		heap.array.swap(0, i);
		heap.sink(0, i);
	}
}
