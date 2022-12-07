use std::time::Instant;

pub struct Measure {
    beg: Instant,
}

impl Measure {
    pub fn new() -> Self {
        Self {
            beg: Instant::now(),
        }
    }
}

impl Drop for Measure {
    fn drop(&mut self) {
        println!("took: {:?}", self.beg.elapsed());
    }
}

impl Default for Measure {
    fn default() -> Self {
        Self::new()
    }
}
