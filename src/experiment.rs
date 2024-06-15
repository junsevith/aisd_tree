use std::ops::Add;

pub struct Stats {
    comps: usize,
    ptr_read: usize,
    ptr_swap: usize,
    height: usize,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            comps: 0,
            ptr_read: 0,
            ptr_swap: 0,
            height: 0,
        }
    }

    pub fn comp(&mut self) {
        self.comps += 1;
    }

    pub fn read(&mut self) {
        self.ptr_read += 1;
    }

    pub fn swap(&mut self) {
        self.ptr_swap += 1;
    }

    pub fn height(&mut self, height: usize) {
        self.height = height;
    }
}

pub struct Data {
    count: usize,
    sum: Stats,
    max: Stats,
}

impl Data {
    pub fn new() -> Self {
        Data {
            count: 0,
            sum: Stats::new(),
            max: Stats::new(),
        }
    }
    pub fn add_stat(&mut self, stats: Stats) {
        self.count += 1;

        self.sum.comps += stats.comps;
        self.sum.ptr_read += stats.ptr_read;
        self.sum.ptr_swap += stats.ptr_swap;
        self.sum.height += stats.height;

        if stats.comps > self.max.comps {
            self.max.comps = stats.comps;
        }
        if stats.ptr_read > self.max.ptr_read {
            self.max.ptr_read = stats.ptr_read;
        }
        if stats.ptr_swap > self.max.ptr_swap {
            self.max.ptr_swap = stats.ptr_swap;
        }
        if stats.height > self.max.height {
            self.max.height = stats.height;
        }
    }

    pub fn avg(&self) -> (f64, f64, f64, f64) {
        let count = self.count as f64;
        let sum = self.sum.comps as f64;
        let ptr_read = self.sum.ptr_read as f64;
        let ptr_swap = self.sum.ptr_swap as f64;
        let height = self.sum.height as f64;

        (sum / count, ptr_read / count, ptr_swap / count, height / count)
    }

    pub fn max(&self) -> &Stats {
        &self.max
    }
}

pub fn divide_into(data: Data, dataset: &mut Vec<Vec<f64>>) {
    let (sum, ptr_read, ptr_swap, height) = data.avg();
    dataset[0].push(sum);
    dataset[1].push(ptr_read);
    dataset[2].push(ptr_swap);
    dataset[3].push(height);

    let max = data.max();
    dataset[4].push(max.comps as f64);
    dataset[5].push(max.ptr_read as f64);
    dataset[6].push(max.ptr_swap as f64);
    dataset[7].push(max.height as f64);
}

impl Add for Data {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.count += other.count;

        self.sum.comps += other.sum.comps;
        self.sum.ptr_read += other.sum.ptr_read;
        self.sum.ptr_swap += other.sum.ptr_swap;
        self.sum.height += other.sum.height;

        self.max.comps = self.max.comps.max(other.max.comps);
        self.max.ptr_read = self.max.ptr_read.max(other.max.ptr_read);
        self.max.ptr_swap = self.max.ptr_swap.max(other.max.ptr_swap);
        self.max.height = self.max.height.max(other.max.height);

        self
    }
}