extern crate csv;
extern crate rustc_serialize;

use csv::{Reader, Writer};
use rustc_serialize::{Encoder, Encodable};

#[derive(Clone, Debug)]
pub struct Node(String);

impl Node {
    pub fn new(name: String) -> Node { Node(name) }
    pub fn name(&self) -> &String { &self.0 }
}

#[derive(Clone, Debug)]
pub struct Task {
    pub name:   String,
    pub req:    u64,
}

impl Task {
    pub fn new(name: String, req: u64) -> Task {
        Task { name: name, req: req }
    }
}

#[derive(Clone, Debug)]
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

    pub fn calc(&self) -> u64 {
        self.tasks.iter().fold(0, |mut acc, elem| {
            acc += elem.req;
            acc
        })
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl Encodable for Assignment {

    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        try!(s.emit_str(&self.node.0));
        try!(s.emit_usize(self.tasks.len()));
        try!(s.emit_u64(self.calc()));
        Ok(())
    }

}

pub fn load(args: Vec<String>) -> (Vec<Node>, Vec<Task>) {
    if args.len() != 3 {
        panic!("Too few ({}) args: 1 = nodes.csv, 2 = tasks.csv expected", args.len());
    }

    let mut nrdr  = csv::Reader::from_file(args[1].clone()).unwrap().has_headers(false);
    let nodes = nrdr.decode().map(|row| {
        let name : String = row.unwrap();
        Node::new(name)
    });

    let mut trdr  = csv::Reader::from_file(args[2].clone()).unwrap().has_headers(false);
    let tasks = trdr.decode().map(|row| {
        let (a, b) = row.unwrap();
        Task::new(a, b)
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
