
// A Persistent, shared list - like in functional programming :-)

// Without garbage collection, instead use REFERENCE COUNTING

use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // Create a new list
    pub fn append(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            // This will increment the count of the tail
            next: self.head.clone(),
        }))}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|rc_node| {
            &rc_node.elem
        })
    }

    // Return the tail if it exists
    // Note that this is different than the docs
    pub fn tail(&self) -> Option<List<T>> {
        self.head.as_ref().map(|ref rc_node| {
            List { head: rc_node.next.clone() }
        })

    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail().unwrap();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let listm: Option<List<i32>> = list.tail();
        assert_eq!(listm.is_none(), true);
    }
}

// Can only implement regular Iter - not IntoIter ot IterMut
// only ever get shared access

pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[test]
fn iter() {
    let list = List::new().append(1).append(2).append(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}
