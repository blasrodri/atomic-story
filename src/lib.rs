//! # Atomic Story
//!
//! Understanding atomics and memory ordering takes time, and it might also be a painful
//! journey. This is mine.
//!
//! I felt that having a repo with simple unit tests would help me enhance my understanding on the topic.
//! Besides, it's a very compact way of knowledge transfer - that's why I am making this public.
//!
//! ## Anatomy
//!
//! ### Counter one writer one reader
//! This is the simples example. Using `Relaxed` is more than enough, since we're only checking the
//! final value after all threads have incremented the value.
//!
//! ### Two numbers with a dependency between them.
//! This example is more interesting. There are two threads: a writer and a reader. And two values,
//! `num_a` and `num_b`. This exmample explores different `Ordering` combinations, and how they
//! can yield different results.

pub mod counter_one_writer_one_reader;
pub mod two_numbers_with_dependency;
