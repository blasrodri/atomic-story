//! These are tests that are put here when I do not fully understand why they are behaving in the way
//! they are.
#[cfg(test)]
mod tests {
    use loom::{
        sync::{atomic::AtomicUsize, Arc},
        thread,
    };
    use std::sync::atomic::Ordering::{Relaxed, SeqCst};

    #[test]
    #[should_panic]
    fn atomics_failing() {
        // TODO: Investigate why it fails with both SeqCst and Relaxed
        let ordering = [SeqCst, Relaxed];
        for ord in ordering {
            let ord = ord.clone();
            loom::model(move || {
                let num_a = Arc::new(AtomicUsize::new(1));
                let num_b = Arc::new(AtomicUsize::new(0));

                let num_a2 = num_a.clone();
                let num_b2 = num_b.clone();
                let tb = thread::spawn(move || {
                    for idx in 1..10 {
                        num_a2.store(idx + 1, ord);
                        num_b2.store(idx, ord);
                    }
                });

                let _ = thread::spawn(move || {
                    for _ in 1..10 {
                        let na = num_a.load(ord);
                        let nb = num_b.load(ord);
                        assert!(na > nb);
                    }
                });
                tb.join().unwrap();
            });
        }
    }
}
