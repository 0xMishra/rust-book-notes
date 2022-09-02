// However, two concurrency concepts are embedded in the language: the std::marker traits Sync and Send.
pub fn run_sync_and_send() {
    // The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads. Almost every Rust type is Send,
    // but there are some exceptions, including Rc<T>: this cannot be Send

    // The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. In other words, any type T is Sync if &T (an immutable reference to T) is Send,

    // The smart pointer Rc<T> is also not Sync
}
