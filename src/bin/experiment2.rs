use itertools::Itertools;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::{Rng, thread_rng};

use aisd_tree::{experiment, rb_tree};
use aisd_tree::chart::draw_chart;
use aisd_tree::experiment::{Data, divide_into};

fn main() {
    let range = (10000..=100000_usize).step_by(10000);
    let reps = 20_usize;
    let ref mut rng = thread_rng();

    let elements = range.clone().try_len().unwrap();

    let mut ins = vec![Vec::with_capacity(elements); 8];
    let mut del = vec![Vec::with_capacity(elements); 8];
    let names = vec!["avg comps", "avg ptr_read", "avg ptr_swap", "avg height", "max comps", "max ptr_read", "max ptr_swap", "max height"];

    for n in range.clone() {
        let range = Uniform::new(0, 2 * n - 1).unwrap();
        let mut insert_data = Data::new();
        let mut delete_data = Data::new();

        for r in 0..reps {
            println!("rep: {}", r);
            let mut tree = rb_tree::BinTree::new();
            let mut elements = Vec::new();

            for _i in 0..n {
                let mut stat = experiment::Stats::new();
                let x = range.sample(rng);
                tree.insert(x, &mut stat);
                elements.push(x);
                stat.height(tree.height());
                insert_data.add(stat)
            }

            for _i in 0..n {
                let mut stat = experiment::Stats::new();
                let index = rng.gen_range(0..elements.len());
                let x = elements.remove(index);
                // let x = range.sample(rng);
                tree.delete(x, &mut stat);
                stat.height(tree.height());
                delete_data.add(stat)
            }
        }
        divide_into(insert_data, &mut ins);
        divide_into(delete_data, &mut del);
        println!("Done {}", n)
    }

    draw_chart(ins, names.clone(), range.clone(), "rb_insert", |_, y| y);
    draw_chart(del, names, range, "rb_delete", |_, y| y);
}