extern crate csv;
extern crate libcommon;

use std::env;
use libcommon::*;

fn main() {
    let (nodes, tasks)   = load(env::args().collect());
    let mut ass : Vec<_> = nodes.into_iter().map(Assignment::new).collect();

    for task in tasks.into_iter() {
        for mut a in ass.iter_mut() {
            a.tick();
        }
        ass.sort_by(|a, b| a.len().cmp(&b.len()));
        ass[0].push(task);
    }

    write_out(ass);
}
