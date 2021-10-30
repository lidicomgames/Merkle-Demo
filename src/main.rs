use sha2::{Digest, Sha256};

// Merkle Tree Binary

#[derive(Debug)]
struct Node {
    value: String,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
        }
    }
}

struct MerkleTree {
    root: String,
    layers: Vec<Vec<Node>>,
}

impl MerkleTree {
    fn new(values: Vec<&str>) -> MerkleTree {
        let mut layers: Vec<Vec<Node>> = vec![Vec::new()];

        for value in values {
            layers[0].push(Node {
                value: value.to_string(),
            });
        }

        let mut nodes = layers[0].clone();

        while nodes.len() > 1 {
            let layer_index = layers.len();
            layers.push(Vec::new());

            for i in 0..((nodes.len() as f32) / 2.0).ceil() as usize {
                let is_last = i * 2 + 1 == nodes.len();

                if is_last {
                    let mut hasher = Sha256::new();
                    hasher.update(format!("{}", nodes[i * 2].value).as_bytes());

                    let node = Node {
                        value: format!("{:x}", hasher.finalize())
                    };

                    layers[layer_index].push(node);
                    continue;
                }

                let left = nodes[i * 2].clone();
                let right = nodes[i * 2 + 1].clone();

                // get sha256 string hash of left and right
                let mut hasher = Sha256::new();
                hasher.update(format!("{}{}", left.value, right.value).as_bytes());

                let node = Node {
                    value: format!("{:x}", hasher.finalize()),
                };

                layers[layer_index].push(node);
            }

            nodes = layers[layer_index].clone();
        }

        MerkleTree {
            root: nodes[0].value.clone(),
            layers,
        }
    }
}

fn main() {
    let leaves = ["1", "2", "3", "4", "5"].to_vec();
    let tree = MerkleTree::new(leaves);
    println!("{:?}", tree.layers);
    println!("{}", tree.root);
}
