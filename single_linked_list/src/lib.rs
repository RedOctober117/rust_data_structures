use std::fmt;

// RUN BACON CLIPPY BRO

pub struct SingleLinkedList<T> {
    next: Option<Box<Node<T>>>,
}

impl<T> SingleLinkedList<T> {

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
    // pub fn tail(&self) -> Option<&Node<T>> {
    //     let mut reference = &self.next;
    //     while match reference { Some(_) => true, _ => false, } {
    //         reference = match reference.as_ref().unwrap().get_next() {
    //             Some(node) => node.as_ref().get_next(),
    //             _ => break,
    //         }
    //     }
    //     match reference.as_ref() {
    //         Some(node) => Some(node),
    //         None => None,
    //     }
    // }

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


    // pub fn get_next(&self) -> Option<&Box<Node<T>>> {
    //     match &self.next {
    //         Some(subnode) => Some(subnode),
    //         None => None,
    //     }
    // }

    // pub fn get_next_mut(&mut self) -> Option<&mut Box<Node<T>>> {
    //     match &mut self.next {
    //         Some(subnode) => Some(subnode),
    //         None => None,
    //     }
    // }

    // pub fn get_value(&self) ->  Option<&T> {
    //     match &self.value {
    //         Some(value) => Some(value),
    //         None => None,
    //     }
    // }

    // pub fn get_value_mut(&mut self) -> Option<&mut T> {
    //     match &mut self.value {
    //         Some(value) => Some(value),
    //         None => None,
    //     }
    // }

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

    // #[test]
    // fn tail_mut_exists_test() {
    //     let mut list: SingleLinkedList<i32> = SingleLinkedList {
    //         next: Some(Box::new(Node {
    //             next: None,
    //             value: Some(8),
    //         })),
    //     };

    //     assert!(match list.tail_mut() {
    //         Some(_) => true,
    //         None => false,
    //     });
    // }

    // #[test]
    // fn tail_mut_not_exists_test() {
    //     let mut list: SingleLinkedList<i32> = SingleLinkedList {
    //         next: None,
    //     };

    //     assert!(match list.tail_mut() {
    //         Some(_) => false,
    //         None => true,
    //     });
    // }

    // #[test]
    // fn tail_value() {

    // }

    // #[test]
    // fn tail_change() {
    //     let mut list: SingleLinkedList<i32> = SingleLinkedList {
    //         next: Some(Box::new(Node {
    //             next: None,
    //             value: Some(8),
    //         })),
    //     };

    //     match list.tail_mut() {
    //         Some(subnode) => *subnode.get_value_mut() = Some(10),
    //         None => panic!("you fucked up"),
    //     };

    //     assert!(match list.tail() {
    //         Some(subnode) => match subnode.get_value() {
    //             Some(value) => match value {
    //                 10 => true,
    //                 _ => false,
    //             },
    //             _ => false, 
    //         }
    //         None => false,
    //     });
    // }
}
