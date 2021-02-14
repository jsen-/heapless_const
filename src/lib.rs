//! `static` friendly data structures that don't require dynamic memory
//! allocation
//!
//! # Examples
//!
//! ```
//! use heapless_const::Queue;
//!
//! let mut rb: Queue<u8, 4> = Queue::new();
//!
//! assert!(rb.enqueue(0).is_ok());
//! assert!(rb.enqueue(1).is_ok());
//! assert!(rb.enqueue(2).is_ok());
//! assert!(rb.enqueue(3).is_ok());
//! assert!(rb.enqueue(4).is_err()); // full
//!
//! assert_eq!(rb.dequeue(), Some(0));
//! ```
//!
//! ### Single producer single consumer mode
//!
//! ```
//! use heapless_const::Queue;
//!
//! static mut RB: Queue<Event, 4> = Queue::new();
//!
//! #[derive(Debug)]
//! enum Event { A, B }
//!
//! fn main() {
//!     // NOTE(unsafe) beware of aliasing the `consumer` end point
//!     let mut consumer = unsafe { RB.split().1 };
//!
//!     loop {
//!         // `dequeue` is a lockless operation
//!         match consumer.dequeue() {
//!             Some(Event::A) => { /* .. */ },
//!             Some(Event::B) => { /* .. */ },
//!             None => { /* sleep */},
//!         }
//! #       break
//!     }
//! }
//!
//! // this is a different execution context that can preempt `main`
//! fn interrupt_handler() {
//!     // NOTE(unsafe) beware of aliasing the `producer` end point
//!     let mut producer = unsafe { RB.split().0 };
//! #   let condition = true;
//!
//!     // ..
//!
//!     if condition {
//!         producer.enqueue(Event::A).unwrap();
//!     } else {
//!         producer.enqueue(Event::B).unwrap();
//!     }
//!
//!     // ..
//! }
//! ```

#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]
mod cfail;
mod sealed;
mod spsc;
pub use spsc::{Consumer, Producer, Queue};
