use tokio::time::{sleep, Duration};
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::task::{Context, Poll};
use std::future::Future;

// Define a custom future that completes after a delay
struct MyFuture {
    delay: tokio::time::Sleep,
    // Use PhantomPinned to ensure that MyFuture is not movable after pinning
    _pin: PhantomPinned,
}

impl MyFuture {
    fn new() -> Pin<Box<Self>> {
        Box::pin(MyFuture {
            delay: sleep(Duration::from_secs(2)),
            _pin: PhantomPinned, // Ensure it is pinned
        })
    }
}

// Implement the Future trait for MyFuture
impl Future for MyFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Access the pinned structure correctly
        let this = self.project();

        // Poll the internal delay (the sleep future)
        match this.delay.poll(cx) {
            Poll::Ready(_) => Poll::Ready("Task is complete!"), // Task finished
            Poll::Pending => Poll::Pending, // Task is still in progress
        }
    }
}

// Pin-projection helper for accessing inner fields safely
impl MyFuture {
    fn project(self: Pin<&mut Self>) -> Pin<&mut tokio::time::Sleep> {
        // Pin-projection: this safely accesses the `delay` field
        unsafe { self.map_unchecked_mut(|s| &mut s.delay) }
    }
}

#[tokio::main]
async fn main() {
    let my_future = MyFuture::new();

    println!("Waiting for task to complete...");

    let result = my_future.await; // Await the custom future
    println!("{}", result);
}
