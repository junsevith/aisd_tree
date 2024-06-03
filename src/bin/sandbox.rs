use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;

use aisd_tree::{experiment, splay_tree};

fn main() {
        let n = 10u32;
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 2 * n - 1).unwrap();
        let mut tree = splay_tree::SplayTree::new();
        let mut elements = Vec::new();
        let ref mut stats = experiment::Stats::new();
        for _i in 0..n {
            let x = range.sample(&mut rng);
            println!("inserting: {:?}", x);
            tree.insert(x, stats);
            elements.push(x);
            // println!("height: {:?}", tree.height());
            // tree.print_tree();
            println!("{:?}", tree);
        }

        for _i in 0..n {
            let index = rng.gen_range(0..elements.len());
            // println!("index: {:?}", index);
            let x = elements.remove(index);
            println!("deleting: {:?}", x);
            tree.delete(x, stats);
            // println!("height: {:?}", tree.height());
            // tree.print_tree();
            println!("{:?}", tree);
        }
}