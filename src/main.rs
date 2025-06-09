#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]

// Необходимо для ночной версии Rust
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(panic_info_message)]

mod allocator;
mod vga_buffer;

extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker}
};

use core::panic::PanicInfo;

// Точка входа (исправляем unsafe)
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    vga_buffer::init();
    println!("Hello World from Rust OS!");
    
    // Асинхронная демо-задача
    async_demo().run();
    
    loop {}
}

// Базовый асинхронный исполнитель
struct Executor {
    tasks: Vec<async_task::Task>,
}

impl Executor {
    fn new() -> Self {
        Executor { tasks: Vec::new() }
    }
    
    fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.tasks.push(async_task::Task::new(future));
    }
    
    fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop() {
            task.poll();
        }
    }
}

mod async_task {
    use super::*;

    pub struct Task {
        future: Pin<Box<dyn Future<Output = ()>>>,
    }

    impl Task {
        pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
            Task {
                future: Box::pin(future),
            }
        }
        
        pub fn poll(&mut self) {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match self.future.as_mut().poll(&mut context) {
                Poll::Ready(()) => println!("Task completed!"),
                Poll::Pending => {
                    println!("Task pending...");
                    // Перепланируем задачу
                    self.future.as_mut().poll(&mut context);
                }
            }
        }
    }

    fn dummy_waker() -> Waker {
        unsafe { Waker::from_raw(dummy_raw_waker()) }
    }

    fn dummy_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            dummy_raw_waker()
        }
        
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(0 as *const (), &VTABLE)
    }
}

// Асинхронная демо-функция
async fn async_counter() {
    for i in 0..5 {
        println!("Async count: {}", i);
        async_delay().await;
    }
}

// Задержка (имитация)
struct Delay {
    cycles: usize,
    current: usize,
}

impl Future for Delay {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.current >= self.cycles {
            Poll::Ready(())
        } else {
            self.current += 1;
            Poll::Pending
        }
    }
}

fn async_delay() -> Delay {
    Delay {
        cycles: 10_000_000,
        current: 0,
    }
}

fn async_demo() -> Executor {
    let mut executor = Executor::new();
    executor.spawn(async_counter());
    executor
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}