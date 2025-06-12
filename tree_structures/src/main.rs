mod binary_tree;
use binary_tree::BinaryTreeNode;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binarytree_test() {
        let tree_test = BinaryTreeNode{
            value: 5,
            left: Some(Box::new(BinaryTreeNode{
                value: 3,
                left: Some(Box::new(BinaryTreeNode{
                    value: 2,
                    left: None,
                    right: None
                })),
                right: None
            })),
            right: Some(Box::new(BinaryTreeNode{
                value:7,
                left: None,
                right: None
            }))
        };

        let mut tree = BinaryTreeNode::new(5);  
        tree.insert(7);
        tree.insert(3);
        tree.insert(2);

        assert_eq!(tree_test, tree)
    }
}
