extern crate csv;
extern crate libcommon;

use std::env;
use libcommon::*;

fn main() {
    let (nodes, tasks) = load(env::args().collect());

    let mut ass : Vec<_> = nodes.into_iter().map(Assignment::new).collect();
    let nass = ass.len();
    let mut i = 0;

    for task in tasks.into_iter() {
        for mut a in ass.iter_mut() {
            a.tick();
        }

        ass[i].push(task);

        i += 1;
        if i >= nass {
            i = 0;
        }
    }

    write_out(ass);
}
