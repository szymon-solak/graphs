#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub edges: Vec<String>,
}

impl Node {
    pub fn new(id: String, edges: Vec<String>) -> Node {
        Node { id, edges }
    }
}
