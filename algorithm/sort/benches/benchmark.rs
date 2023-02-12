use std::cell::RefCell;
use std::ops::DerefMut;
use std::ptr::copy_nonoverlapping;

use criterion::{Criterion, criterion_group, criterion_main};
use rand::distributions::Uniform;
use rand::Rng;

use sort::{*};

const DATA_SIZE_LARGE: usize = 8_000_000;
const DATA_SIZE_SMALL: usize = 40_000;
const VALUE_RANGE: u32 = 0x1000000;

const FAST_CASES: [(&str, SortFn); 11] = [
	// Contrast
	("Rust Stable", <[u32]>::sort),
	("Rust Unstable", <[u32]>::sort_unstable),

	// n ** 2
	("Shell", insertion::shell),

	// n * log(2, n)
	("Merge", merge::sort),
	("Heap", heap::sort),
	("Quick", quick::sort),
	("QuickInsert", hybrid::quick_insert),

	// n * log(radix, n)
	("RadixCounting", bound_radix_array),
	("RadixGrouping", bound_radix_grouping),
	("3-Way Radix", bound_radix_quick),

	// O(n)
	("Counting", bound_counting),
];

const SLOW_CASES :[(&str, SortFn); 5] = [
	// n ** 2
	("Bubble", bubble::sort),
	("Comb", bubble::comb),
	("Selection", selection::sort),
	("Insertion", insertion::sort),
	("Binary", insertion::binary),
];

/// 复制数组的几种方式，clone_into 跟指针操作性能一样，to_vec 则要慢一些。
fn array_clone(c: &mut Criterion) {
	let template: Vec<u32> = rand::thread_rng()
		.sample_iter(Uniform::from(0..VALUE_RANGE))
		.take(DATA_SIZE_LARGE).collect();

	let keep_array = RefCell::new(vec![0u32; DATA_SIZE_LARGE]);
	let mut raw_slice = vec![0u32; DATA_SIZE_LARGE];

	c.bench_function("to_vec", |b| {
		b.iter(|| template.to_vec());
	});
	c.bench_function("clone_into", |b| {
		b.iter(|| {
			template.clone_into(keep_array.borrow_mut().deref_mut());
			return keep_array.borrow_mut();
		});
	});
	c.bench_function("unsafe_ptr", |b| {
		b.iter(|| unsafe {
			let p = template.as_slice().as_ptr();
			let q = raw_slice.as_mut_slice().as_mut_ptr();
			copy_nonoverlapping(p, q, DATA_SIZE_LARGE);
		});
	});
}

fn sort(c: &mut Criterion) {
	let template: Vec<u32> = rand::thread_rng()
		.sample_iter(Uniform::from(0..VALUE_RANGE))
		.take(DATA_SIZE_LARGE).collect();

	let inst = RefCell::new(vec![0u32; DATA_SIZE_LARGE]);

	for (id, algorithm) in FAST_CASES {
		c.bench_function(id, |b| {
			b.iter_with_setup(
				|| {
					template.clone_into(inst.borrow_mut().deref_mut());
				},
				|_| algorithm(inst.borrow_mut().deref_mut()),
			)
		});
	}

	for (id, algorithm) in SLOW_CASES {
		c.bench_function(id, |b| {
			b.iter_with_setup(
				|| {
					template.clone_into(inst.borrow_mut().deref_mut());
				},
				|_| {
					let mut b = inst.borrow_mut();
					let x = b.deref_mut();
					algorithm(&mut x[..DATA_SIZE_SMALL])
				},
			)
		});
	}
}

criterion_group!(benches, sort);
criterion_main!(benches);
