use std::{fmt, ops::Index};

// RUN BACON CLIPPY BRO
// Result either contains an Ok() or an error type of your choosing

pub struct SingleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: i32,
}

impl<T: std::cmp::PartialEq> SingleLinkedList<T> {

    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn new_with_node(node: Node<T>) -> Self {
        Self {
            head: Some(Box::new(node)),
            length: 0,
        }
    }

    pub fn add_node(&mut self, node: Node<T>) {
        match self.tail_mut() {
            Some(subnode) => subnode.next = Some(Box::new(node)),
            None => self.head = Some(Box::new(node)),
        };
    }

    pub fn add(&mut self, value: T) {
        match self.tail_mut() {
            Some(subnode) => subnode.next = Some(Box::new(Node::new_with_value(value))),
            None => self.head = Some(Box::new(Node::new_with_value(value))),
        };
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Result<&Node<T>, IndexOutOfBounds> {
        if index >= self.length {
            return Err(IndexOutOfBounds { });
        }
        let mut reference = &self.head;
        let mut counter = 0;
        while counter < index {
            reference = &reference.as_ref().unwrap().next;
            counter = counter + 1;
        }
        Ok(reference.as_ref().unwrap())
    }

    pub fn insert(&mut self, index: i32, value: T) -> Result<bool, IndexOutOfBounds> {
        if index >= self.length { 
            return Err(IndexOutOfBounds{ }); 
        }
        let mut prev_node = self.get(index - 1);
        let mut post_node = self.get(index);
        let new_node = Node::new_with_ref_and_value(post_node.ok().unwrap().to_owned(), value);
        self.length += 1;

        Ok(true)
    }


    ///
    /// # Errors
    /// 
    /// Will return `Err` if no valid node exists
    pub fn search(&self, value: &T) -> Result<&Node<T>, NodeNotFoundError>  {
        let mut reference = &self.head;
        // Run while has next node and subnode_value != value
        while reference.as_ref().map_or(false, |subnode| subnode.has_next()) &
            reference.as_ref().map_or(false, |subnode| subnode.value.as_ref().map_or(true, |subnode_value| subnode_value == value)) {
                reference = match reference {
                    Some(subnode) => &subnode.next,
                    None => break,
                };
        }

        reference.as_deref().ok_or(NodeNotFoundError{})
    }

    #[must_use]
    pub fn tail(&self) -> Option<&Node<T>> {
        let mut reference = &self.head;
        while reference.as_ref().map_or(false, |node| node.has_next()) {
            reference = match reference {
                Some(subnode) => &subnode.next,
                None => break,
            };
        }
        reference.as_deref()
    }

    pub fn tail_mut(&mut self) -> Option<&mut Node<T>> {
        let mut reference = &mut self.head;
        while reference.as_mut().map_or(false, |node| node.has_next()) {
            reference = match reference {
                Some(subnode) => &mut subnode.next,
                None => break,
            };
        }
        reference.as_deref_mut()
    }
}

pub struct NodeNotFoundError { }
pub struct IndexOutOfBounds { }

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: Option<T>,
}

impl<T> Node<T> {

    #[must_use]
    pub const fn new() -> Self {
        Self {
            next: None,
            value: None,
        }
    }

    pub fn new_with_next(next: Self) -> Self {
        Self {
            next: Some(Box::new(next)),
            value: None,
        }
    }

    pub const fn new_with_value(value: T) -> Self {
        Self {
            next: None,
            value: Some(value),
        }
    }

    pub fn new_with_ref_and_value(next: Self, value: T) -> Self {
        Self {
            next: Some(Box::new(next)),
            value: Some(value),
        }
    }

    pub const fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {:?} }}", self.value)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tail_exists_test() {
        let list: SingleLinkedList<i32> = SingleLinkedList::new_with_node(Node {
                next: None,
                value: Some(8),
            });

        assert!(list.tail().is_some());
    }

    #[test]
    fn tail_not_exists_test() {
        let list: SingleLinkedList<i32> = SingleLinkedList::new();

        assert!(list.tail().is_none());
    }

    #[test]
    fn valid_search() {
        let list: SingleLinkedList<i32> = SingleLinkedList::new_with_node(Node::new_with_value(8));
        
        assert!(list.search(&8).is_ok());
    }

    #[test]
    fn invalid_search() {
        let list: SingleLinkedList<i32> = SingleLinkedList::new();
        
        assert!(list.search(&8).is_err());
    }

    #[test]
    fn add() {
        let mut list: SingleLinkedList<i32> = SingleLinkedList::new();
        list.add_node(Node::new_with_value(8));
        assert_eq!(list.tail().expect("test failed, node does not exist or has wrong value.").value, Some(8));
    }

    #[test]
    fn get() {
        let mut list: SingleLinkedList<i32> = SingleLinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(4);

        assert!(match list.get(3) {
            Ok(node) => match node.value {
                Some(4) => true,
                _ => false,
            },
            _ => false,
        });
    }

}
