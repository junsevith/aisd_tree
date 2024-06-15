use itertools::Itertools;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::thread_rng;
use rayon::prelude::*;

use aisd_tree::{bin_tree, experiment};
use aisd_tree::chart::draw_chart;
use aisd_tree::experiment::{Data, divide_into};

fn main() {
    let range = (10_000..=100_000_usize).step_by(10_000);
    let reps = 20_usize;


    let elements = range.clone().try_len().unwrap();

    let mut ins = vec![Vec::with_capacity(elements); 8];
    let mut del = vec![Vec::with_capacity(elements); 8];
    let names = vec!["avg comps", "avg ptr_read", "avg ptr_swap", "avg height", "max comps", "max ptr_read", "max ptr_swap", "max height"];

    range.clone().map(|n| {

        println!("n: {}", n);
        let mut range = Uniform::new(0, 2 * n - 1).unwrap();

        (0..reps).into_par_iter().map(|r| {
            let ref mut rng = thread_rng();
            let mut tree = bin_tree::BinTree::new();
            let mut insert_data = Data::new();
            let mut delete_data = Data::new();

            for _i in 0..n {
                let mut stat = experiment::Stats::new();
                let x = range.sample(rng);
                tree.insert(x, &mut stat);
                stat.height(tree.height());
                insert_data.add_stat(stat);
            }

            for _i in 0..n {
                let mut stat = experiment::Stats::new();
                // let index = rng.gen_range(0..n);
                let x = range.sample(rng);
                stat.height(tree.height());
                tree.delete(x, &mut stat);
                delete_data.add_stat(stat);
            }

            println!("Done {}", r);
            (insert_data, delete_data)
        }
        ).reduce(
            || (Data::new(), Data::new()),
            |(mut insert_data, mut delete_data), (insert_data2, delete_data2)| {
            (insert_data + insert_data2, delete_data + delete_data2)
        })
    }).for_each(|(insert_data, delete_data)| {
        divide_into(insert_data, &mut ins);
        divide_into(delete_data, &mut del);
    });


    draw_chart(ins, names.clone(), range.clone(), "insert", |_, y| y);
    draw_chart(del, names, range, "delete", |_, y| y);
}