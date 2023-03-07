use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::ptr;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, SystemTime};

use futures::FutureExt;
use futures::task::{ArcWake, waker_ref};

/// 跟 JS 公共的 Promise + 闭包存状态不同，Rust 以不同的 Future 保存状态。
struct SimpleSleepFuture {
    complete_at: SystemTime,
}

// 简单的轮询检查式等待，完全没有异步，也用不着 Context 来唤醒什么。
impl Future for SimpleSleepFuture {

    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let is_after = SystemTime::now() >= self.complete_at;
        return if is_after { Poll::Ready(()) } else { Poll::Pending }
    }
}

fn sleep_poll(time: Duration) -> SimpleSleepFuture {
    return SimpleSleepFuture { complete_at: SystemTime::now() + time }
}

/// 本例子中使用同步轮询，没有异步通知所以也不需要实现 wake。
struct NOPWake {}

impl ArcWake for NOPWake {
    fn wake_by_ref(_: &Arc<Self>) {}
}

async fn wait() {
    sleep_poll(Duration::from_secs(1)).await;
    println!("First step.");

    sleep_poll(Duration::from_secs(1)).await;
    println!("Second step, finished.");
}

fn main() {
    let task = Arc::new(NOPWake {});
    let waker = waker_ref(&task);
    let ctx = &mut Context::from_waker(&*waker);

    let mut future = wait().boxed();
    let mut prev = future.deref() as *const _;

    loop {
        let pinned = future.as_mut();

        let pointer = pinned.deref() as *const _;
        if prev != pointer {
            prev = pointer;
            println!("Future changed.");
        }

        if pinned.poll(ctx) != Poll::Pending { break }
    }
}
