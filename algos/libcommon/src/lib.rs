extern crate csv;
extern crate rustc_serialize;

use csv::{Reader, Writer};
use rustc_serialize::{Encoder, Encodable};

pub struct Node {
    pub name: String,
    pub mem:  u64,
    pub cpu:  u8,
    pub net:  u32,
}

impl Node {
    pub fn new(name: String, cpu: u8, mem: u64, net: u32) -> Node {
        Node { name: name, mem: mem, cpu: cpu, net: net }
    }
}

pub struct Task {
    pub name:    String,
    pub req_mem: u64,
    pub req_cpu: u8,
    pub req_net: u32,
}

impl Task {
    pub fn new(name: String, req_cpu: u8, req_mem: u64, req_net: u32) -> Task {
        Task { name: name, req_mem: req_mem, req_cpu: req_cpu, req_net: req_net }
    }
}

pub struct Assignment {
    pub node: Node,
    pub tasks: Vec<Task>,
}

impl Assignment {

    pub fn new(n: Node) -> Assignment {
        Assignment { node: n, tasks: vec![] }
    }

    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn calc(&self) -> (u64, u64, u64) {
        self.tasks.iter().fold((0, 0, 0), |mut acc, elem| {
            acc.0 += elem.req_mem;
            acc.1 += elem.req_cpu as u64;
            acc.2 += elem.req_net as u64;

            acc
        })
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl Encodable for Assignment {

    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let (mem, cpu, net) = self.calc();
        try!(s.emit_str(&self.node.name));
        try!(s.emit_u64(mem));
        try!(s.emit_u64(cpu));
        try!(s.emit_u64(net));
        Ok(())
    }

}

pub fn load(args: Vec<String>) -> (Vec<Node>, Vec<Task>) {
    if args.len() != 3 {
        panic!("Too few ({}) args: 1 = nodes.csv, 2 = tasks.csv expected", args.len());
    }

    let mut nrdr  = csv::Reader::from_file(args[1].clone()).unwrap().has_headers(false);
    let nodes = nrdr.decode().map(|row| {
        let (a, b, c, d) = row.unwrap();
        Node::new(a, b, c, d)
    });

    let mut trdr  = csv::Reader::from_file(args[2].clone()).unwrap().has_headers(false);
    let tasks = trdr.decode().map(|row| {
        let (a, b, c, d) = row.unwrap();
        Task::new(a, b, c, d)
    });

    (nodes.collect(), tasks.collect())
}

pub fn write_out(assignments: Vec<Assignment>) {
    use std::io::stdout;
    let mut w = Writer::from_writer(stdout());

    for a in assignments {
        w.encode(a);
    }
}
