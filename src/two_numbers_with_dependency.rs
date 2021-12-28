//! Modelling a scenario with two values, a and b. Both a and b are incremented only on a writer thread
//! and and read on a reader thread.
//! Invariants
//! At t0: num_a > num_b
//! On each iteration, increment (store) a, with a value larger than the one used in b.
//! On each iteration of the loop, read b before reading a
//!
//! Expected logic:
//! num_a should be >= than num_b
//!
//! Notes:
//! whenever the reader uses `Relaxed` on both operations, then the test fails.
#[cfg(test)]
mod tests {
    use loom::{
        sync::{atomic::AtomicUsize, Arc},
        thread,
    };
    use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};

    #[test]
    #[should_panic]
    fn atomics_relaxed_failing() {
        let ordering = Relaxed;
        loom::model(move || {
            let num_a = Arc::new(AtomicUsize::new(1));
            let num_b = Arc::new(AtomicUsize::new(0));

            let num_a2 = num_a.clone();
            let num_b2 = num_b.clone();
            let tb = thread::spawn(move || {
                for idx in 1..4 {
                    num_a2.store(idx + 1, ordering);
                    num_b2.store(idx, ordering);
                }
            });

            let _ = thread::spawn(move || {
                for _ in 1..4 {
                    let nb = num_b.load(ordering);
                    let na = num_a.load(ordering);
                    assert!(na >= nb);
                }
            });
            tb.join().unwrap();
        });
    }

    #[test]
    #[should_panic]
    fn atomics_writer_release_reader_relaxed_failing() {
        loom::model(move || {
            let num_a = Arc::new(AtomicUsize::new(1));
            let num_b = Arc::new(AtomicUsize::new(0));

            let num_a2 = num_a.clone();
            let num_b2 = num_b.clone();
            let tb = thread::spawn(move || {
                for idx in 1..4 {
                    num_a2.store(idx + 1, Release);
                    num_b2.store(idx, Release);
                }
            });

            let _ = thread::spawn(move || {
                for _ in 1..4 {
                    let nb = num_b.load(Relaxed);
                    let na = num_a.load(Relaxed);
                    assert!(na >= nb);
                }
            });
            tb.join().unwrap();
        });
    }

    #[test]
    fn atomics_writer_release_reader_seqcst_and_relaxed() {
        loom::model(move || {
            let num_a = Arc::new(AtomicUsize::new(1));
            let num_b = Arc::new(AtomicUsize::new(0));

            let num_a2 = num_a.clone();
            let num_b2 = num_b.clone();
            let tb = thread::spawn(move || {
                for idx in 1..4 {
                    num_a2.store(idx + 1, Release);
                    num_b2.store(idx, Release);
                }
            });

            let _ = thread::spawn(move || {
                for _ in 1..4 {
                    // Note that if we put SeqCst on num_b, we ensure that it's read _before_ num_a, which
                    // ensures that the test passes.
                    let nb = num_b.load(SeqCst);
                    let na = num_a.load(Relaxed);
                    assert!(na >= nb);
                }
            });
            tb.join().unwrap();
        });
    }

    #[test]
    fn atomics_seq_cst_does_not_fail() {
        let ordering = SeqCst;
        loom::model(move || {
            let num_a = Arc::new(AtomicUsize::new(1));
            let num_b = Arc::new(AtomicUsize::new(0));

            let num_a2 = num_a.clone();
            let num_b2 = num_b.clone();
            let tb = thread::spawn(move || {
                for idx in 1..4 {
                    num_a2.store(idx + 1, ordering);
                    num_b2.store(idx, ordering);
                }
            });

            let _ = thread::spawn(move || {
                for _ in 1..4 {
                    let nb = num_b.load(ordering);
                    let na = num_a.load(ordering);
                    assert!(na >= nb);
                }
            });
            tb.join().unwrap();
        });
    }

    #[test]
    fn atomics_acquire_release_does_not_fail() {
        loom::model(move || {
            let num_a = Arc::new(AtomicUsize::new(1));
            let num_b = Arc::new(AtomicUsize::new(0));

            let num_a2 = num_a.clone();
            let num_b2 = num_b.clone();
            let tb = thread::spawn(move || {
                for idx in 1..4 {
                    // on every iteration of the loop, num_a > num_b
                    num_a2.store(idx + 1, Release);
                    num_b2.store(idx, Release);
                }
            });

            let _ = thread::spawn(move || {
                for _ in 1..4 {
                    let nb = num_b.load(Acquire);
                    let na = num_a.load(Acquire);
                    assert!(na >= nb);
                }
            });
            tb.join().unwrap();
        });
    }
}
