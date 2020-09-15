use std::cell::RefCell;

// Declare a new thread local storage key of type [`std::thread::LocalKey`].
// RefCell: A mutable memory location with dynamically checked borrow rules.
thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

struct Context<'a> {
    waker: &'a Waker,
}

impl<'a> Context<'a> {
    // provides the context
    fn from_waker(waker: &'a Waker) -> Self {
        Context { waker }
    }

    // provide the waker
    fn waker(&self) -> &'a Waker {
        &self.waker
    }
}

struct Waker;

impl Waker {
    fn wake(&self) {
        NOTIFY.with(|is_ready| *is_ready.borrow_mut() = true)
    }
}

enum Poll<T> {
    Ready(T),
    Pending,
}

trait Future {
    type Output;

    fn poll(&mut self, context: &Context) -> Poll<Self::Output>;
}
// Initialize with default value
#[derive(Default)]
struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        println!("Checking whether ready to poll or not");
        match self.count {
            3 => Poll::Ready(3),
            _ => {
                self.count += 1;
                ctx.waker().wake();
                Poll::Pending
            }
        }
    }
}

fn run<F>(mut future: F) -> F::Output
    where
        F: Future,
{

    // It loops until it gets notified that the future is ready to be polled again
    NOTIFY.with(|is_ready| loop {

        // Immutably borrows the wrapped value and check whether true or not.
        if *is_ready.borrow() {
            // Mutably borrows the wrapped value and updates the value.
            *is_ready.borrow_mut() = false;
            // getting the context for method
            let ctx: Context = Context::from_waker(&Waker);
            // validating the future is ready to poll or not.
            if let Poll::Ready(value) = future.poll(&ctx) {
                println!("Finally!!! our future is completed");
                return value;
            }
        }
    })
}

// This program pertains to internal execution of futures
fn main() {
    let my_future = MyFuture::default();
    // executing future
    println!("Output: {}", run(my_future));
}
