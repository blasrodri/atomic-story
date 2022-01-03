//! # Verify that Relaxed works fine when updating a counter
//!
//!
//! Atomic operations tagged memory_order_relaxed are not synchronization operations; they do not impose an
//! order among concurrent memory accesses.
//!
//! They only guarantee atomicity and modification order consistency.
//!
//! # Relaxed
//! Our first example describes the following scenario: There are three writer threads
//! and the main thread acts as the only reader. There is only one counter being updated.
//! The ordering chosen is `Ordering::Relaxed`. We show that, after joining the writer threads
//! the value is properly update.
//!
//! I am still trying to come up with a meaningful failing example.
//!
//!
//!
//! ```rust
//! use loom::sync::atomic::AtomicUsize;
//! use loom::sync::atomic::Ordering::Relaxed;
//! use loom::sync::Arc;
//! use loom::thread;
//!
//! #[cfg(test)]
//! fn relaxed_guarantees_full_completion_upon_joining_all_threads() {
//! // Typical use for relaxed memory ordering is (in/de)crementing counters,
//! // since this only requires atomicity, but not ordering or synchronization
//!     loom::model(|| {
//!         let num = Arc::new(AtomicUsize::new(0));
//!         let num_reader = num.clone();
//!
//!         let mut v = vec![];
//!         for _ in 0..3 {
//!             let num = num.clone();
//!             v.push(thread::spawn(move || num.fetch_add(1, Relaxed)));
//!         }
//!
//!         for t in v {
//!             t.join().unwrap();
//!         }
//!
//!         let num_value = num_reader.load(Relaxed);
//!         assert_eq!(3, num_value)
//!     });
//! }
//! ```

#[cfg(test)]
mod tests {

    use loom::sync::atomic::AtomicUsize;
    use loom::sync::atomic::Ordering::Relaxed;
    use loom::sync::Arc;
    use loom::thread;

    #[test]
    fn relaxed_guarantees_full_completion_upon_joining_all_threads() {
        // Typical use for relaxed memory ordering is (in/de)crementing counters,
        // since this only requires atomicity, but not ordering or synchronization
        loom::model(|| {
            let num = Arc::new(AtomicUsize::new(0));
            let num_reader = num.clone();

            let mut v = vec![];
            for _ in 0..3 {
                let num = num.clone();
                v.push(thread::spawn(move || num.fetch_add(1, Relaxed)));
            }

            for t in v {
                t.join().unwrap();
            }

            let num_value = num_reader.load(Relaxed);
            assert_eq!(3, num_value)
        });
    }
}
