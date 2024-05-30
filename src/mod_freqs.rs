use num::{complex::Complex32 as C32, Zero};

use crate::fft::{fft, ifft};

pub struct ModFreqs<I, F> {
    src: I,
    modified_chunk: Vec<C32>,
    sample_size: usize,
    modifier_function: F,
}

impl<I: Iterator<Item = C32>, F: FnMut(C32) -> C32> ModFreqs<I, F> {
    pub fn new(src: I, modifier_function: F, sample_size: usize) -> Self {
        Self {
            modified_chunk: Vec::with_capacity(sample_size),
            modifier_function,
            sample_size,
            src,
        }
    }
}

impl<I: Iterator<Item = C32>, F: FnMut(C32) -> C32> Iterator for ModFreqs<I, F> {
    type Item = C32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.modified_chunk.pop() {
            return Some(v);
        }

        let first = self.src.next()?;

        self.modified_chunk
            .push_within_capacity(first)
            .expect("WUT???");

        for v in self
            .src
            .by_ref()
            .chain(std::iter::repeat(C32::zero()))
            .take(self.sample_size - 1)
        {
            self.modified_chunk
                .push_within_capacity(v)
                .expect("No capacity??");
        }

        let mut freqs = fft(&self.modified_chunk);
        for f in &mut freqs {
            *f = (self.modifier_function)(*f);
        }

        let res = ifft(&freqs);
        self.modified_chunk = res;

        Some(self.modified_chunk.pop().expect("We just filled it..."))
    }
}
