use wasm_bindgen::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    x: usize,
    y: usize,
    direction: Direction,
}

#[wasm_bindgen]
pub struct Maze {
    height: usize,
    width: usize,
    edges: Vec<Edge>,
    nodes: Vec<Node>,
}

pub struct Node {
    x: usize,
    y: usize,
    parent: usize,
    rank: usize,
    index: usize,
}

fn opposite(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Top => (x, y - 1),
        Direction::Left => (x - 1, y),
        _ => (x, y),
    }
}


impl Node {
    fn new (x: usize, y: usize, id: usize) -> Self {
        Node {
            x: x,
            y: y,
            parent: id,
            rank: 0,
            index: id
        }
    }
}

#[wasm_bindgen]
impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut edges = Vec::new();
        let mut nodes = Vec::new();

        for x in 0..width {
            for y in 0..height {
                let id = nodes.len();
                nodes.push(Node::new(x, y, id));

                if x > 0 {
                    edges.push(Edge {x: x, y: y, direction: Direction::Left});
                }

                if y > 0 {
                    edges.push(Edge {x: x, y: y, direction: Direction::Top});
                }
            }
        }

        Maze {
            width,
            height,
            edges,
            nodes,
        }
    }

    fn union (&mut self, x: usize, y: usize) {
        let x_root = self.find_node_set(&self.nodes[x]);
        let y_root = self.find_node_set(&self.nodes[y]);

        if self.nodes[x_root].parent == self.nodes[y_root].parent {
            return
        }

        if self.nodes[x_root].rank > self.nodes[y_root].rank {
            self.nodes[y_root].parent = self.nodes[x_root].index;
        } else {
            self.nodes[x_root].parent = self.nodes[y_root].index;

            if self.nodes[x_root].rank == self.nodes[y_root].rank {
                self.nodes[y_root].rank = self.nodes[x_root].rank + 1
            }
        }
    }

    fn find (&self, x: usize, y: usize) -> usize {
        let index = self.get_index(x, y);
        self.find_node_set(&self.nodes[index])
    }

    fn get_index(&self, x: usize, y:usize) -> usize {
        x * self.height + y
    }

    fn find_node_set (&self, node: &Node) -> usize {
        if node.parent != node.index {
            self.find_node_set(&self.nodes[node.parent])
        } else {
            node.parent
        }
    }

    pub fn tick (&mut self) -> JsValue {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0,self.edges.len());
        let choice = self.edges.remove(index);
        let choice_direction = choice.direction.clone();

        let (x, y) = (choice.x, choice.y);
        let (dx, dy) = opposite(x, y, choice_direction);

        let set1 = self.find(x, y);
        let set2 = self.find(dx, dy);

        if set1 != set2 {
            let pos1 = self.get_index(x, y);
            let pos2 = self.get_index(dx, dy);

            self.union(pos1, pos2);

            return JsValue::from_serde(&choice).unwrap();
        } else {
            return JsValue::null();
        }
    }
}
