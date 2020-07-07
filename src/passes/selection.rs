use petgraph::dot::{Config, Dot};
use petgraph::graph;
use petgraph::prelude::Graph;
use petgraph::visit::{DfsPostOrder, Dfs};

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Any,
    Ref,
    Add,
    Mul,
    Reg,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Loc {
    Any,
    Gen,
    Lut,
    Dsp,
    Equal(String),
}

#[derive(Clone, Debug)]
pub struct PlacedOp {
    op: Op,
    loc: Loc,
}

impl PlacedOp {
    pub fn new(op: Op, loc: Loc) -> PlacedOp {
        PlacedOp { op: op, loc: loc }
    }

    pub fn new_gen_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Gen,
        }
    }

    pub fn new_lut_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Lut,
        }
    }

    pub fn new_dsp_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Dsp,
        }
    }

    pub fn new_any_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Any,
        }
    }

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }

    pub fn cost(&self) -> i32 {
        match (&self.op, &self.loc) {
            (Op::Add, Loc::Gen) => 9,
            (Op::Add, Loc::Lut) => 8,
            (Op::Add, Loc::Dsp) => 2,
            (Op::Mul, Loc::Gen) => 9,
            (Op::Mul, Loc::Lut) => 8,
            (Op::Mul, Loc::Dsp) => 2,
            (Op::Reg, Loc::Lut) => -4,
            (Op::Reg, Loc::Dsp) => -1,
            (_, _) => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    placed_op: PlacedOp,
}

impl Node {
    pub fn new(name: &str, placed_op: PlacedOp) -> Node {
        Node {
            name: name.to_string(),
            placed_op: placed_op,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pattern {
    name: String,
    cost: i32,
    ops: Vec<PlacedOp>,
}

impl Pattern {
    pub fn new(name: &str, cost: i32) -> Pattern {
        Pattern {
            name: name.to_string(),
            cost: cost,
            ops: Vec::new(),
        }
    }

    pub fn push_op(&mut self, op: PlacedOp) {
        self.ops.push(op);
    }
}

fn pat_dsp_muladd() -> Pattern {
    let mut pat = Pattern::new("dsp_muladd", 1);
    pat.push_op(PlacedOp::new_dsp_op(Op::Add));
    pat.push_op(PlacedOp::new_dsp_op(Op::Mul));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat
}

fn pat_dsp_mul() -> Pattern {
    let mut pat = Pattern::new("dsp_mul", 4);
    pat.push_op(PlacedOp::new_dsp_op(Op::Mul));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat
}

fn pat_dsp_add() -> Pattern {
    let mut pat = Pattern::new("dsp_add", 4);
    pat.push_op(PlacedOp::new_dsp_op(Op::Add));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat
}

pub type DAG = Graph<Node, ()>;
pub type DAGIx = graph::NodeIndex;

fn estimate_cost(dag: &DAG, start: DAGIx) -> i32 {
    let mut cost: i32 = 0;
    let mut visit = Dfs::new(dag, start);
    while let Some(ix) = visit.next(dag) {
        if let Some(node) = dag.node_weight(ix) {
            cost += node.placed_op.cost();
        }
    }
    cost
}

fn select(dag: &mut DAG, ix: DAGIx, pattern: &Pattern) {
    let mut root = DfsPostOrder::new(&*dag, ix);
    while let Some(root_ix) = root.next(&*dag) {
        let mut pattern_ops = pattern.ops.iter();
        // check if there is a pattern match
        let mut is_match: bool = true;
        let mut subgraph = Dfs::new(&*dag, root_ix);
        while let Some(sub_ix) = subgraph.next(&*dag) {
            if let Some(pattern_placed_op) = pattern_ops.next() {
                if let Some(node) = dag.node_weight(sub_ix) {
                    if pattern_placed_op.op != Op::Any && node.placed_op.op != pattern_placed_op.op {
                        is_match = false;
                    }
                }
            } else {
                break;
            }
        }
        if is_match && pattern_ops.len() == 0 {
            let cost = estimate_cost(&*dag, root_ix);
            if pattern.cost < cost {
                if let Some(node) = dag.node_weight_mut(root_ix) {
                    println!("new candidate, pattern:{} pattern-cost:{} node:{} node-cost:{}",
                        pattern.name, pattern.cost, node.name, cost);
                }
                let mut is_first: bool = true;
                let mut pattern_ops = pattern.ops.iter();
                let mut subgraph = Dfs::new(&*dag, root_ix);
                let node_id: String = dag.node_weight(root_ix).unwrap().name.to_string();
                while let Some(sub_ix) = subgraph.next(&*dag) {
                    if let Some(pattern_placed_op) = pattern_ops.next() {
                        if let Some(node) = dag.node_weight_mut(sub_ix) {
                            if pattern_placed_op.op != Op::Any {
                                if is_first {
                                    node.placed_op.loc = pattern_placed_op.loc.clone();
                                    is_first = false;
                                } else {
                                    node.placed_op.loc = Loc::Equal(node_id.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn main() {
    let mut graph = DAG::new();
    let a = graph.add_node(Node::new("a", PlacedOp::new_gen_op(Op::Ref)));
    let b = graph.add_node(Node::new("b", PlacedOp::new_gen_op(Op::Ref)));
    let c = graph.add_node(Node::new("c", PlacedOp::new_gen_op(Op::Ref)));
    let t0 = graph.add_node(Node::new("t0", PlacedOp::new_gen_op(Op::Mul)));
    let t1 = graph.add_node(Node::new("t1", PlacedOp::new_gen_op(Op::Add)));

    graph.add_edge(t0, a, ());
    graph.add_edge(t0, b, ());
    graph.add_edge(t1, t0, ());
    graph.add_edge(t1, c, ());

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let patterns = vec![pat_dsp_mul(), pat_dsp_add(), pat_dsp_muladd()];

    for p in patterns.iter() {
        select(&mut graph, t1, p);
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    }
}
