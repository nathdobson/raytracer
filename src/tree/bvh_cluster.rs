use std::cell::RefCell;
use std::cmp;
use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};
use std::collections::btree_map::Entry;
use std::iter::{Peekable, Rev};
use ordered_float::NotNan;
use crate::Bvh;
use crate::mesh::TriMesh;
use unordered_pair::UnorderedPair;
use crate::geo::bounds::Bounds;
use priority_queue::PriorityQueue;
use crate::tree::bv_kd_tree::{BvKdEntry, BvKdQuery, BvKdTree};
use crate::tree::bvh::{BvhForest, BvhTree};
use crate::tree::kd_tree::KdIter;
use crate::util::itertools2::{Itertools2, Peeker};



#[derive(Debug)]
struct EdgeSet<'a> {
    iter: Peeker<KdIter<'a, BvKdEntry<usize>, BvKdQuery>>,
}

impl<'a> EdgeSet<'a> {
    fn peek(&self) -> NotNan<f64> { NotNan::new(self.iter.peek().unwrap().distance).unwrap() }
}

impl<'a> Eq for EdgeSet<'a> {}

impl<'a> PartialEq<Self> for EdgeSet<'a> {
    fn eq(&self, other: &Self) -> bool { self.peek() == other.peek() }
}

impl<'a> PartialOrd<Self> for EdgeSet<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.peek().partial_cmp(&other.peek()) }
}

impl<'a> Ord for EdgeSet<'a> {
    fn cmp(&self, other: &Self) -> Ordering { self.peek().cmp(&other.peek()) }
}

impl BvhForest {
    pub fn contract(mut self) -> BvhTree {
        while self.trees.len() > 1 {
            self.contract_once();
        }
        self.trees.into_iter().next().unwrap()
    }
    pub fn contract_once(&mut self) {
        let kdtree = BvKdTree::new(
            self.trees.iter().enumerate()
                .map(|(index, tree)| BvKdEntry::new(*tree.bounds(), index))
                .collect());
        let mut graph = PriorityQueue::<usize, EdgeSet>::new();
        let mut reverse = HashMap::<_, Vec<_>>::new();
        for (index, tree) in self.trees.iter().enumerate() {
            let mut edge_set = EdgeSet { iter: kdtree.nearest_iter(*tree.bounds()).peeker() };
            if *edge_set.iter.peek().unwrap().entry.value().value() == index {
                edge_set.iter.next();
            }
            reverse.entry(*edge_set.iter.peek().unwrap().entry.value().value()).or_default().push(index);
            graph.push(index, edge_set);
        }
        let mut contracted = HashMap::new();
        let mut contractions = vec![];
        while graph.len() >= 2 {
            let (n1, mut es) = graph.pop().unwrap();
            let n2: usize = *es.iter.next().unwrap().entry.value().value();
            graph.remove(&n2);
            assert!(contracted.insert(n1, n2).is_none());
            assert!(contracted.insert(n2, n1).is_none());
            contractions.push((n1, n2));
            for n in [n1, n2] {
                if let Some(nos) = reverse.remove(&n) {
                    for no in nos {
                        if no != n1 && no != n2 {
                            graph.change_priority_by(&no, |to_fix| {
                                loop {
                                    if let Some(head) = to_fix.iter.peek() {
                                        let head = head.entry.value().value();
                                        if contracted.contains_key(head) || *head == no {
                                            to_fix.iter.next();
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            });
                            if let Some(non) = graph.get(&no) {
                                if let Some(head) = non.1.iter.peek() {
                                    reverse.entry(*head.entry.value().value()).or_default().push(no);
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut new_nodes = vec![];
        let mut old_nodes: HashMap<usize, BvhTree> = self.trees.drain(..).enumerate().collect();
        for (i, j) in contractions {
            let i = old_nodes.remove(&i).unwrap();
            let j = old_nodes.remove(&j).unwrap();
            let mut node = BvhTree::new();
            node.add_node(i);
            node.add_node(j);
            new_nodes.push(node);
        }
        assert!(old_nodes.len() == 0 || old_nodes.len() == 1);
        if let Some((_, old)) = old_nodes.into_iter().next() {
            new_nodes.push(old)
        }
        self.trees = new_nodes;
    }
}
