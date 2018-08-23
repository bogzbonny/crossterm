
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
mod terminal;
mod color;
mod cursor;
mod some_types;
mod input;

use std::io::Write;
use std::{thread,time};
fn main()
{
    input::keyboard::async_input::read_async_demo();
   terminal::raw_mode::print_wait_screen_on_alternate_window();
    thread::sleep(time::Duration::from_millis(2000));
}
