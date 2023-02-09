use criterion::{Criterion, criterion_group, criterion_main};
use rand::distributions::Uniform;
use rand::Rng;

use sort::{*};

const DATA_SIZE: usize = 20_000;
const VALUE_RANGE: u32 = 1000;

const CASES: [(&str, SortFn); 11] = [
	// O(n ** 2)
	("Bubble", bubble::sort),
	("Selection", selection::sort),
	("Insertion", insertion::sort),
	("Shell", insertion::shell),
	("Binary", insertion::binary),

	// O(n * log(2, n))
	("Merge",  merge::sort),
	("Heap",  heap::sort),
	("Quick",  quick::sort),

	// O(n * log(radix, n))
	("RadixCounting",  bound_radix_array),
	("RadixGrouping",  bound_radix_grouping),

	// O(n)
	("Counting",  bound_counting),
];

fn criterion_benchmark(c: &mut Criterion) {
	let template: Vec<u32> = rand::thread_rng()
		.sample_iter(Uniform::from(0..VALUE_RANGE))
		.take(DATA_SIZE).collect();

	for (id, algorithm) in CASES {
		c.bench_function(id, |b| {
			b.iter_with_setup(
				|| template.clone(),
				|mut array| algorithm(array.as_mut_slice())
			)
		});
	}
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
