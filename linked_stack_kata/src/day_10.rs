use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(item: T, next: Link<T>) -> Link<T> {
        Some(Box::new(Node { item, next }))
    }
}

pub struct Stack<T> {
    head: Link<T>
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.item
        })
    }

    pub fn push(&mut self, item: T) {
        self.head = Node::new(item, self.head.take())
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIter { stack: self }
    }
}

pub struct StackIter<T> {
    stack: Stack<T>
}

impl<T> Iterator for StackIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<II: IntoIterator<Item=T>>(iter: II) -> Self {
        let mut stack = Stack::default();

        for item in iter {
            stack.push(item)
        }

        stack
    }
}

impl<T> AsRef<Self> for Stack<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'s, T> IntoIterator for &'s Stack<T> {
    type Item = &'s T;
    type IntoIter = StackRefIter<'s, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackRefIter { node: self.head.as_ref().map(|node| &**node)}
    }
}

pub struct StackRefIter<'s, T> {
    node: Option<&'s Node<T>>
}

impl<'s, T> Iterator for StackRefIter<'s, T> {
    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

impl<T> AsMut<Self> for Stack<T> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'s, T> IntoIterator for &'s mut Stack<T> {
    type Item = &'s mut T;
    type IntoIter = StackMutRefIter<'s, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackMutRefIter { node: self.head.as_mut().map(|node| &mut **node)}
    }
}

pub struct StackMutRefIter<'s, T> {
    node: Option<&'s mut Node<T>>
}

impl<'s, T> Iterator for StackMutRefIter<'s, T> {
    type Item = &'s mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_mut().map(|node| &mut **node);
            &mut node.item
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();

        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_from_empty_stack() {
        let mut stack: Stack<i32> = Stack::default();

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pop_push_single_item() {
        let mut stack = Stack::default();

        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pop_push_many_items() {
        let mut stack = Stack::default();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn iterator() {
        let stack = Stack::from_iter(1..=3);

        let mut iter = stack.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ref_iterator() {
        let stack = Stack::from_iter(1..=3);

        let mut iter = stack.as_ref().into_iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ref_mut_iterator() {
        let mut stack = Stack::from_iter(1..=3);

        let mut iter = stack.as_mut().into_iter();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
