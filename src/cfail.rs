//! Compile fail tests
//!
//! # `Send`-ness
//!
//! Collections of `Send`-able things are `Send`
//!
//! ```
//! use heapless_const::{Queue, Consumer, Producer};
//!
//! struct IsSend;
//!
//! unsafe impl Send for IsSend {}
//!
//! fn is_send<T>() where T: Send {}
//!
//! is_send::<Consumer<IsSend, 4>>();
//! is_send::<Producer<IsSend, 4>>();
//! is_send::<Queue<IsSend, 4>>();
//! ```
//!
//! Collections of non-`Send`-able things are *not* `Send`
//!
//! ``` compile_fail
//! use std::marker::PhantomData;
//! use heapless_const::{Consumer, MultiCore};
//!
//! type NotSend = PhantomData<*const ()>;
//!
//! fn is_send<T>() where T: Send {}
//!
//! is_send::<Consumer<NotSend, 4>>();
//! ```
//!
//! ``` compile_fail
//! use std::marker::PhantomData;
//! use heapless_const::{Producer, MultiCore};
//!
//! type NotSend = PhantomData<*const ()>;
//!
//! fn is_send<T>() where T: Send {}
//!
//! is_send::<Producer<NotSend, 4>>();
//! ```
//!
//! ``` compile_fail
//! use std::marker::PhantomData;
//! use heapless_const::Queue;
//!
//! type NotSend = PhantomData<*const ()>;
//!
//! fn is_send<T>() where T: Send {}
//!
//! is_send::<Queue<NotSend, 4>>();
//! ```
//!
//! # Freeze
//!
//! Splitting a `RingBuffer` should invalidate the original reference.
//!
//! ```
//! use heapless_const::Queue;
//!
//! let mut rb: Queue<u8, 4> = Queue::new();
//!
//! let (p, c) = rb.split();
//! rb.enqueue(0).unwrap();
//! ```
