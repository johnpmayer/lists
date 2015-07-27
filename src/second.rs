
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // mem::replace(&mut option, None)
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // Steal the value of self.head
        self.head.take().map(|boxed_node| {
            let node = *boxed_node;
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // by default, we moved self.head by value
        // could do this ourselves with explicit match
        // easier to keep using map
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// IntoIter: iterator when we OWN the linked list

// Just a quick wrapper, new syntax!
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
}

// Iter: iterator when we only have a borrow reference to the list

// the lifetime of the iterator is the lifetime of the node it might hold
// Also need to say the inner type has the same lifetime?
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

// List doesn't have a lifetime, only the actual creation of the iter
impl<T> List<T> {
    // the iter has a lifetime, it comes from the list
    pub fn iter(&self) -> Iter<T> {
        // need a ref to the boxed node
        Iter { next: self.head.as_ref().map(|boxed_node| &**boxed_node) }
    }
}

// but we do need it here, as Iter does have a lifetime
impl<'a, T> Iterator for Iter<'a, T> {
    // lifetime of each element is same as the iterator
    type Item = &'a T;
    
    // everything else just works (yiss)
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // need a ref to the boxed node
            // Exercise - expand the &** crap
            self.next = node.next.as_ref().map(|boxed_node| &**boxed_node);
            &node.elem
        })
    }
}

#[test]
fn iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}
