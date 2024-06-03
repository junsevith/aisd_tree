
pub mod bin_tree;
pub mod rb_tree;
pub mod experiment;
pub mod chart;
pub mod splay_tree;

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;
    use rand::prelude::{Distribution, SliceRandom};
    use rand::Rng;
    use super::*;


    #[test]
    fn test_bin_tree_rand() {
        let n = 50usize;
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 2 * n - 1).unwrap();
        let mut tree = bin_tree::BinTree::new();
        let mut elements = Vec::new();
        let mut stats = experiment::Stats::new();
        for _i in 0..n {
            let x = range.sample(&mut rng);
            println!("inserting: {:?}", x);
            tree.insert(x, &mut stats);
            elements.push(x);
            println!("height: {:?}", tree.height2());
            println!("{:?}", tree);
        }

        for _i in 0..n {
            let index = rng.gen_range(0..elements.len());
            // println!("index: {:?}", index);
            let x = elements.remove(index);
            println!("deleting: {:?}", x);
            tree.delete(x, &mut stats);
            println!("height: {:?}", tree.height2());
            println!("{:?}", tree);
        }
    }

    #[test]
    fn test_bin_tree_inc() {
        let n = 50usize;
        let mut tree = bin_tree::BinTree::new();
        let ref mut stats = experiment::Stats::new();
        for i in 0..n {
            println!("inserting: {:?}", i);
            tree.insert(i, stats);
            println!("{:?}", tree.height());
            println!("{:?}", tree);
        }
        println!("{:?}", tree);
        let mut elements = (0..n).collect::<Vec<_>>();
        elements.shuffle(&mut rand::thread_rng());

        for i in elements {
            println!("deleting: {:?}", i);
            tree.delete(i, stats);
            println!("height: {:?}", tree.height());
            println!("{:?}", tree);
        }
    }

    #[test]
    fn test_rb_tree_rand() {
        let n = 50usize;
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 2 * n - 1).unwrap();
        let mut tree = rb_tree::BinTree::new();
        let mut elements = Vec::new();
        let mut stats = experiment::Stats::new();
        for _i in 0..n {
            let x = range.sample(&mut rng);
            println!("inserting: {:?}", x);
            tree.insert(x, &mut stats);
            elements.push(x);
            println!("height: {:?}", tree.height());
            println!("{:?}", tree);
        }

        for _i in 0..n {
            let index = rng.gen_range(0..elements.len());
            // println!("index: {:?}", index);
            let x = elements.remove(index);
            println!("deleting: {:?}", x);
            tree.delete(x, &mut stats);
            println!("height: {:?}", tree.height());
            println!("{:?}", tree);
        }
    }

    #[test]
    fn test_rb_tree_inc() {
        let n = 50usize;
        let mut tree = rb_tree::BinTree::new();
        let ref mut stats = experiment::Stats::new();
        for i in 0..n {
            println!("inserting: {:?}", i);
            tree.insert(i, stats);
            println!("{:?}", tree.height());
            println!("{:?}", tree);
        }
        println!("{:?}", tree);
        let mut elements = (0..n).collect::<Vec<_>>();
        elements.shuffle(&mut rand::thread_rng());

        for i in elements {
            println!("deleting: {:?}", i);
            tree.delete(i, stats);
            println!("height: {:?}", tree.height());
            println!("{:?}", tree);
        }
    }

    #[test]
    fn test_splay_tree_rand() {
        let n = 50usize;
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 2 * n - 1).unwrap();
        let mut tree = splay_tree::SplayTree::new();
        let mut elements = Vec::new();
        let mut stats = experiment::Stats::new();
        for _i in 0..n {
            let x = range.sample(&mut rng);
            println!("inserting: {:?}", x);
            tree.insert(x, &mut stats);
            elements.push(x);
            println!("height: {:?}", tree.height());
            println!("{:?}", tree);
        }

        for _i in 0..n {
            let index = rng.gen_range(0..elements.len());
            // println!("index: {:?}", index);
            let x = elements.remove(index);
            println!("deleting: {:?}", x);
            tree.delete(x, &mut stats);
            println!("height: {:?}", tree.height());
            println!("{:?}", tree);
        }
    }

    #[test]
    fn test_splay_tree_inc() {
        let n = 50usize;
        let mut tree = splay_tree::SplayTree::new();
        let ref mut stats = experiment::Stats::new();
        for i in 0..n {
            println!("inserting: {:?}", i);
            tree.insert(i, stats);
            println!("{:?}", tree.height());
            println!("{:?}", tree);
        }
        println!("{:?}", tree);
        let mut elements = (0..n).collect::<Vec<_>>();
        elements.shuffle(&mut rand::thread_rng());

        for i in elements {
            println!("deleting: {:?}", i);
            tree.delete(i, stats);
            println!("height: {:?}", tree.height());
            println!("{:?}", tree);
        }
    }
}

