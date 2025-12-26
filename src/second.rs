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

    pub fn push(&mut self, new_number: T) {
        let new_node = Node {
            elem: new_number,
            next: self.head.take(),
        };
        let new_link = Some(Box::new(new_node));
        self.head = new_link;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|first| {
            self.head = first.next;
            first.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|first| &first.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // We'll look at this error next time!
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// A lifetime is just a named section of our
// code for which some object exists.
// We need to say: "This iterator cannot be used
//  after the list it references is freed."
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    pub fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // peek tests:
        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.peek_mut(), Some(&mut 5));

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None::<&i32>);
        assert_eq!(list.peek_mut(), None::<&mut i32>);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // match list.peek_mut() {
        //     Some(&mut top) => top = 42,
        //     None => todo!(),
        // }
        // let &mut x = &mut 5;
        list.peek_mut().map(|top_ref| *top_ref = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    // Macros in C:
    // ```c
    // #define A_MACRO 5
    // #define min(a, b) (((a) < (b)) ? (a) : (b))
    // #define foo(x) { int a = 5;  }
    // Source - https://stackoverflow.com/a
    // Posted by mouviciel, modified by community. See post 'Timeline' for change history
    // Retrieved 2025-12-19, License - CC BY-SA 4.0

    // #define INTMAX(A, B) ({ int _a = (A), _b = (B) ; _a > _b ? _a : _b ; })

    //
    // int a = 5;
    // printf("%d\n", min(a,6));
    // printf("%d\n", ((foo(5) < (6)) ? foo(5) : (6)));
    // ```
    // Preprocessor (shared between C and a buncha other langs) -> Compiler Itself -> ...
    //
    // Rust's macro system is less cleanly separated (the compiler handles macros itself), but avoids shenanigans because it is smarter than simple text replacement. It always knows the whole language.

    #[test]
    fn into_iter_for_loop() {
        let mut list = List::new();
        list.push(3);
        list.push(3);
        list.push(3);

        for x in list.into_iter() {
            assert_eq!(x, 3);
        }
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        assert_eq!(list.peek(), Some(&3));

        let mut l_iter;
        {
            let mut l = List::new();
            l.push(5);
            l_iter = l.iter();
        }
        assert_eq!(l_iter.next(), Some(&5));
    }
}
