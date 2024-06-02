use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;

use aisd_tree::{experiment, rb_tree};

fn main() {
    loop {
        let n = 10000usize;
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 2 * n - 1).unwrap();
        let mut tree = rb_tree::BinTree::new();
        let mut elements = Vec::new();
        let mut stats = experiment::Stats::new();
        for _i in 0..n {
            let x = range.sample(&mut rng);
            // println!("inserting: {:?}", x);
            tree.insert(x, &mut stats);
            elements.push(x);
            // println!("height: {:?}", tree.height());
            // println!("{:?}", tree);
        }

        for _i in 0..n {
            let index = rng.gen_range(0..elements.len());
            // println!("index: {:?}", index);
            let x = elements.remove(index);
            // println!("deleting: {:?}", x);
            tree.delete(x, &mut stats);
            // println!("height: {:?}", tree.height());
            // println!("{:?}", tree);
        }
    }
}