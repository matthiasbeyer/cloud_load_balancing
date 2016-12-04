extern crate csv;
extern crate libcommon;

use std::env;
use libcommon::*;

fn calc_factor(a: &Assignment) -> u64 {
    let node_capability = a.node.mem / 10 + (a.node.cpu as u64) * 10 + (a.node.net as u64) * 5;

    let (umem, ucpu, unet) = a.tasks.iter()
        .map(|t| (t.req_mem as u64, t.req_cpu as u64, t.req_net as u64))
        .fold((0, 0, 0), |(acca, accb, accc), (a, b, c)| {
            (acca + a, accb + b, accc + c)
        });

    let (umem, ucpu, unet) = (umem / 10, ucpu * 10, unet * 5);

    if node_capability < (ucpu + umem + unet) {
        0
    } else {
        node_capability - umem - ucpu - unet
    }
}

fn main() {
    let (nodes, tasks)   = load(env::args().collect());
    let mut ass : Vec<_> = nodes.into_iter().map(Assignment::new).collect();

    for task in tasks.into_iter() {
        ass.sort_by(|a, b| calc_factor(b).cmp(&calc_factor(a)));
        ass[0].push(task);
    }

    write_out(ass);
}
