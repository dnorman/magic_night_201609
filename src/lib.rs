#[allow(non_snake_case)]
use std::mem;

pub struct List <T> {
    head: Link<T>
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl <T> Drop for List<T> {
    fn drop(&mut self){
        //let mut link = self.head;
        let mut link = mem::replace(&mut self.head,Link::Empty);
        //drop(self);

        loop {
            match link {
                Link::Empty => {
                    break;
                },
                Link::More(boxed_node) => {
                    let node = *boxed_node;
                    link = node.next;
                }
            }
        }
    }
}

impl <T> List<T> {
    pub fn new () -> Self {
        List {
            head: Link::Empty
        }
    }
    pub fn push( &mut self, elem: T ) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        };

        self.head = Link::More(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            },
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

mod test {
    use super::List;

    #[test]
    fn basics() {
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

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn hurtlocker(){
        let mut list = List::new();
        for i in 1..50000 {
            list.push(i);
        }
        println!("IM ALIVE");
    }
}
