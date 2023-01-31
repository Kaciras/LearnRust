use criterion::{Criterion, criterion_group, criterion_main};
use rand::distributions::Uniform;
use rand::Rng;

use sort::{bubble, selection};

const LENGTH: usize = 10_000;

fn criterion_benchmark(c: &mut Criterion) {
	let template: Vec<u32> = rand::thread_rng()
		.sample_iter(Uniform::from(0..1000))
		.take(LENGTH).collect();

	c.bench_function("Bubble", |b| {
		b.iter_with_setup(
			|| template.clone(),
			|mut array| bubble::sort(array.as_mut_slice())
		)
	});

	c.bench_function("Selection", |b| {
		b.iter_with_setup(
			|| template.clone(),
			|mut array| selection::sort(array.as_mut_slice())
		)
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
