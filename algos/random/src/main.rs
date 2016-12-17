extern crate csv;
extern crate random;
extern crate libcommon;

use std::env;
use libcommon::*;
use random::Source;

fn main() {
    let (nodes, tasks)   = load(env::args().collect());
    let mut ass : Vec<_> = nodes.into_iter().map(Assignment::new).collect();
    let nass = ass.len();

    let mut source = random::default().seed([42, 1337]);

    for task in tasks.into_iter() {
        let i : usize = source.read::<usize>() % nass;
        ass[i].push(task);
    }

    write_out(ass);
}
