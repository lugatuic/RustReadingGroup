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

        for x in list {}
    }
}
