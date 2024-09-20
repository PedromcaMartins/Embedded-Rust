use core::{future::Future, pin::Pin, ptr, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}};

fn noop_waker() -> Waker {
    // Create a raw waker with dummy functions
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        |ptr| RawWaker::new(ptr, &VTABLE), // clone
        |_: *const ()| {},
        |_: *const ()| {},
        |_: *const ()| {},        
    );

    let raw_waker = RawWaker::new(ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

pub fn poll_future<F: Future>(future: &mut F) -> F::Output {
    // Create a dummy waker and context
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);
    let mut pinned_future = unsafe { Pin::new_unchecked(future) };

    // Poll the future until it's ready
    loop {
        match pinned_future.as_mut().poll(&mut context) {
            Poll::Ready(val) => return val,
            Poll::Pending => {
                // In a real no_std environment, you might yield or
                // do something else, like waiting for an interrupt
            }
        }
    }
}