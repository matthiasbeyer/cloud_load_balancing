extern crate csv;
extern crate libcommon;

extern crate rayon;

use std::collections::HashMap;
use std::env;
use libcommon::*;

use rayon::par_iter::IntoParallelIterator;
use rayon::par_iter::ParallelIterator;

static N : usize = 10;

fn main() {
    let (nodes, tasks)   = load(env::args().collect());
    let chunks = tasks.chunks(tasks.len() / N).map(Vec::from).collect::<Vec<Vec<_>>>();
    let ass = chunks.into_par_iter()
        .map(|tasks| {
            let mut ass : Vec<_> = nodes.clone().into_iter().map(Assignment::new).collect();

            for task in tasks.into_iter() {
                ass.sort_by(|a, b| a.len().cmp(&b.len()));
                ass[0].push(task);
            }

            ass
        })
        .collect::<Vec<Vec<_>>>()
        .into_iter()
        .fold(HashMap::new(), |mut acc, assignments| {
            for assignment in assignments.into_iter() {
                let name = assignment.node.name().clone();
                if acc.contains_key(&name) {
                    let contained : &mut Assignment = acc.get_mut(&name).unwrap();
                    for task in assignment.tasks.into_iter() {
                        contained.push(task);
                    }
                } else {
                    acc.insert(name, assignment);
                }
            }

            acc
        });

    let mut output = vec![];
    for (_, v) in ass.into_iter() {
        output.push(v);
    }

    write_out(output);
}
