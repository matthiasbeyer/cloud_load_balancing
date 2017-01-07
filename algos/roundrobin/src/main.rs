extern crate csv;
extern crate libcommon;
extern crate two_lock_queue;

use std::env;
use std::collections::HashMap;
use std::time::Duration;
use std::thread::JoinHandle;
use libcommon::*;

use two_lock_queue::channel;

static N: usize = 10;

fn main() {
    let (nodes, tasks) = load(env::args().collect());

    let (tx, rx) = channel(100);

    let ass = (0..N).into_iter().map(|n| {
        let thread_rx = rx.clone();
        let thread_nodes = nodes.clone();
        ::std::thread::Builder::new()
            .name(format!("{}", n))
            .spawn(move || {
                let thread_name = ::std::thread::current().name().map(String::from).unwrap();
                let mut n_timeouts = 0;

                let mut ass : Vec<_> = thread_nodes.into_iter().map(Assignment::new).collect();
                let nass = ass.len();
                let mut i = 0;

                loop {
                    match thread_rx.recv_timeout(Duration::from_secs(1)) {
                        Err(e) => { n_timeouts += 1; }
                        Ok(t) => {
                            ass[i].push(t);

                            i += 1;
                            if i >= nass {
                                i = 0;
                            }
                        },
                    }

                    if !thread_rx.is_open() || n_timeouts > 5 {
                        break;
                    }
                }

                ass
            })
            .unwrap()
        })
        .collect::<Vec<JoinHandle<_>>>();

    for task in tasks {
        println!("Send: {}", task.name);
        tx.send(task);
    }

    let ass = ass.into_iter()
        .map(|jh| jh.join().unwrap())
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
