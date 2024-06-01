pub struct Stats {
    comps: usize,
    ptrs: usize,
    height: usize,
}

impl Stats {
    fn new() -> Self {
        Stats { comps: 0, ptrs: 0, height: 0 }
    }
}