use crate::data::board::Board;

#[derive(Clone)]
pub struct Node<T> {
    pub children: Vec<Node<T>>,
    data: Option<T>,
}

impl Node<Board> {
    pub fn new(data: Board) -> Self {
        Self {
            data: Some(data),
            children: vec![],
        }
    }
    pub fn add_child(&mut self, node: &Node<Board>) {
        self.children.push(node.clone());
    }
    pub fn from_board(board: Board) -> Self {
        Self {
            children: vec![],
            data: Some(board),
        }
    }
    pub fn get_date(&self) -> &Option<Board> {
        &self.data
    }
}
