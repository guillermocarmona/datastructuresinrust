#[derive(Debug, PartialEq)]
pub struct BinaryTreeNode {
    pub value: u32,
    pub left: Option<Box<BinaryTreeNode>>,
    pub right: Option<Box<BinaryTreeNode>>
}

impl BinaryTreeNode {
    pub fn new(value: u32) -> Self {
        BinaryTreeNode {
            value,
            left: None,
            right: None
        }  
    }

    pub fn insert(&mut self, value: u32) {
        if self.value == value { return }

        if self.value > value {
            match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(BinaryTreeNode::new(value)))
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(BinaryTreeNode::new(value)))
            }
        }
    }

}
    
