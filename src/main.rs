#![no_std]
#![no_main]

extern crate alloc;

use embedded_alloc::LlffHeap as Heap;
use panic_probe as _;
use rhai::Engine;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    let _engine = Engine::new_raw();

    loop {}
}
