pub struct DoubleLinkedList<'a, T> {
    next: Option<Box<Node<'a, T>>>,
}

impl<'a, T> DoubleLinkedList<'a, T> {
    pub fn new() -> Self {
        Self {
            next: None,
        }
    }

    pub fn new_with_next(subnode: Node<'a, T>) -> Self {
        Self {
            next: Some(Box::new(subnode)),
        }
    }
}

impl<'a, T> Default for DoubleLinkedList<'a, T> {
    fn default() -> Self {
        Self {
            next: None,
        }
    }
}

pub struct Node<'a, T> {
    prev: Option<&'a Node<'a, T>>,
    next: Option<Box<Node<'a, T>>>,
    value: Option<T>,
}

impl<'a, T> Node<'a, T> {
    pub fn new() -> Self {
        Self {
            prev: None,
            next: None,
            value: None,
        }
    }

    pub fn new_with_prev(supernode: &'a Self) -> Self {
        Self {
            prev: Some(supernode),
            next: None,
            value: None,
        }
    }

    pub fn new_with_value(value: T) -> Self {
        Self {
            prev: None,
            next: None,
            value: Some(value),
        }
    }

    pub fn new_with_value_and_prev(supernode: &'a Self, value: T) -> Self {
        Self {
            prev: Some(supernode),
            next: None,
            value: Some(value),
        }
    }
}

impl<'a, T> Default for Node<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
