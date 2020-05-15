//! This crate is a minimalist crate to generate signal
//! with arbitrary functions to a limited time-resolution.
//! 
//! # Example
//! ```
//! use periodicsynth::{sin, synth};
//! 
//! fn main()
//! { let samp = synth(sin, 440f, 8000); }
//! ```

mod common;
pub use common::*;

/// Synthesize a signal in a defined time-resolution
/// which is controlled by the number of samples by
/// by using those time-positions or a custom data
/// structure to accomplish that.
/// 
/// # Arguments
/// - `func` — function to use for callback to
/// generate amplitude values.
/// 
/// - `data` — function specific mutable data/
/// data-structure to pass-in.
///
/// - `n` — number of samples to generate (controls
/// the time-resolution).
pub fn synth<'a, U: Copy, F: Fn(f64, &mut U) -> f64>
(func: F, data: &mut U, n: usize) -> Vec<f64> {
    let mut samples = Vec::<f64>::with_capacity(n);
    
    let step_factor = 1.0/n as f64;
    let mut time_pos = 0f64;

    for _ in 0..n {
        samples.push(func(time_pos, data));
        time_pos+= step_factor;
    } samples
}