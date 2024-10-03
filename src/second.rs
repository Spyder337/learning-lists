use std::mem;

//  Option<T>::take() => mem::replace(self, None)

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) -> () {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        //  Option<T>::map() => match option { None => None, Some(x) => Some(y), }
        //  take head and map on Some(node) => replace head; return node.elem;
        self.head.take().map(|node| {
            self.head  = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
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

//  There are three different kinds of iterators to implement:
//  - IntoIter: `T`
//  - IterMut - `&mut T`
//  - Iter - `&T`

//  Create a tuple struct as a wrapper for the list.
//  Acts a wrapper struct to handle the conversion to an iterator.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        //  Return the current node and wrap it in
        //  IntoIter
        IntoIter(self)
    }
}

//  The type Item must be used for the Option<Self::Item>.
//  Iterator requires a next function that returns the next item.
//  It can be a wrapper around pop in this case.
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

//  The struct is a helper for impl Iterator.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl< T> List<T> {
    pub fn iter(&self) -> Iter<'_, T>  {
        Iter { next: self.head.as_deref() }
    }
}

//  Note next returns a reference to a value.
//  next() -> Option<&T> where T is the value. 
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

//  Helper struct for iter_mut()
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        //  Use map same as before however take is required
        //  in order to mutate the reference.
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

mod test {
    type List = super::List<i32>;

    #[test]
    fn push_empty() -> () {
        let mut list = List::new();
        list.push(32);
        assert_eq!(list.pop(), Some(32));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_value() -> () {
        let mut list = List::new();
        list.push(32);
        list.push(64);
        list.push(96);
        list.push(128);
        assert_eq!(list.pop(), Some(128));
        assert_eq!(list.pop(), Some(96));
        assert_eq!(list.pop(), Some(64));
        assert_eq!(list.pop(), Some(32));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(),None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn peek_mut()  {
        let mut list = List::new();
        list.push(32);
        list.push(64);
        list.push(96);
        assert_eq!(list.peek(), Some(&96));

        list.peek_mut().map(|value| {
            *value = 32
        });

        assert_eq!(list.peek(), Some(&32));
        assert_eq!(list.pop(), Some(32));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(32);
        list.push(64);
        list.push(96);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(),  Some(96));
        assert_eq!(iter.next(),  Some(64));
        assert_eq!(iter.next(),  Some(32));
        assert_eq!(iter.next(),  None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(32);
        list.push(64);
        list.push(96);
        let mut iter = list.iter();
        assert_eq!(iter.next(),  Some(&96));
        assert_eq!(iter.next(),  Some(&64));
        assert_eq!(iter.next(),  Some(&32));
        assert_eq!(iter.next(),  None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(32);
        list.push(64);
        list.push(96);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(),  Some(&mut 96));
        assert_eq!(iter.next(),  Some(&mut 64));
        assert_eq!(iter.next(),  Some(&mut 32));
        assert_eq!(iter.next(),  None);
    }
}