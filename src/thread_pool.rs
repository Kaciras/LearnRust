use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
	threads: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

impl ThreadPool {

	/// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let mut threads = Vec::with_capacity(size);
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		for _ in 0..size {
			threads.push(Worker::new(Arc::clone(&receiver)))
		}
		return ThreadPool { threads, sender };
	}

	pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static{
		let job = Box::new(f);
		self.sender.send(job).unwrap();
	}
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>) { (*self)() }
}

struct Worker {
	task: thread::JoinHandle<()>,
}

impl Worker {
	fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let task = thread::spawn(move || {
			loop {
				let job = receiver.lock().unwrap().recv().unwrap();
				job.call_box();
			}
		});
		return Worker{ task };
	}
}