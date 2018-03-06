pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let node =  Box::new(Node {
            value: element,
            next: self.head.take()
        });
        self.len += 1;
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map( |node| {
            let node = *node;
            self.len -= 1;
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value )
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();
        let mut head = self.head.as_ref();
        while let Some(next) = head {
            new_list.push(next.value.clone());
            head = next.next.as_ref();
        }
        new_list
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in item {
            list.push(i.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(value) = self.pop() {
            result.insert(0, value);
        }
        result
    }
}
