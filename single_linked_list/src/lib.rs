use std::fmt;

// RUN BACON CLIPPY BRO

pub struct SingleLinkedList<T> {
    next: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq> SingleLinkedList<T> {

    #[must_use]
    pub const fn new() -> Self {
        Self {
            next: None,
        }
    }

    pub fn new_with_node(node: Node<T>) -> Self {
        Self {
            next: Some(Box::new(node))
        }
    }

    pub fn search(&self, value: &T) -> Option<&Node<T>> {
        let mut reference = &self.next;
        // Run while has next node and subnode_value != value
        while reference.as_ref().map_or(false, |subnode| subnode.has_next()) &
            reference.as_ref().map_or(false, |subnode| subnode.value.as_ref().map_or(true, |subnode_value| subnode_value == value)) {
                reference = match reference {
                    Some(subnode) => &subnode.next,
                    None => break,
                };
        }
        reference.as_deref()
    }

    #[must_use]
    pub fn tail(&self) -> Option<&Node<T>> {
        let mut reference = &self.next;
        while reference.as_ref().map_or(false, |node| node.has_next()) {
            reference = match reference {
                Some(subnode) => &subnode.next,
                None => break,
            };
        }
        reference.as_deref()
    }
}

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
        
        assert!(list.search(&8).is_some());
    }

    #[test]
    fn invalid_search() {
        let list: SingleLinkedList<i32> = SingleLinkedList::new();
        
        assert!(list.search(&8).is_none());
    }

}
