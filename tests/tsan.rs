#![deny(warnings)]
use heapless_const::Queue;
use scoped_threadpool::Pool;
use std::thread;

#[test]
fn once() {
    static mut RB: Queue<i32, 4> = Queue::new();

    let rb = unsafe { &mut RB };

    rb.enqueue(0).unwrap();

    let (mut p, mut c) = rb.split();

    p.enqueue(1).unwrap();

    thread::spawn(move || {
        p.enqueue(1).unwrap();
    });

    thread::spawn(move || {
        c.dequeue().unwrap();
    });
}

#[test]
fn twice() {
    static mut RB: Queue<i32, 8> = Queue::new();

    let rb = unsafe { &mut RB };

    rb.enqueue(0).unwrap();
    rb.enqueue(1).unwrap();

    let (mut p, mut c) = rb.split();

    thread::spawn(move || {
        p.enqueue(2).unwrap();
        p.enqueue(3).unwrap();
    });

    thread::spawn(move || {
        c.dequeue().unwrap();
        c.dequeue().unwrap();
    });
}

#[test]
fn scoped() {
    let mut rb: Queue<i32, 4> = Queue::new();

    rb.enqueue(0).unwrap();

    {
        let (mut p, mut c) = rb.split();

        Pool::new(2).scoped(move |scope| {
            scope.execute(move || {
                p.enqueue(1).unwrap();
            });

            scope.execute(move || {
                c.dequeue().unwrap();
            });
        });
    }

    rb.dequeue().unwrap();
}

#[test]
fn contention() {
    const N: usize = 1024;

    let mut rb: Queue<u8, N> = Queue::new();

    {
        let (mut p, mut c) = rb.split();

        Pool::new(2).scoped(move |scope| {
            scope.execute(move || {
                let mut sum: u32 = 0;

                for i in 0..(2 * N) {
                    let i = i as u8;
                    sum = sum.wrapping_add(i as u32);
                    while let Err(_) = p.enqueue(i) {}
                }

                println!("producer: {}", sum);
            });

            scope.execute(move || {
                let mut sum: u32 = 0;

                for _ in 0..(2 * N) {
                    loop {
                        match c.dequeue() {
                            Some(v) => {
                                sum = sum.wrapping_add(v as u32);
                                break;
                            }
                            _ => {}
                        }
                    }
                }

                println!("consumer: {}", sum);
            });
        });
    }

    assert!(rb.is_empty());
}
