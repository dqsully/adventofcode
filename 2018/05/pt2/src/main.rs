use std::fmt::Debug;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct ListItem<T> {
    next_id: Option<usize>,
    prev_id: Option<usize>,
    data: T,
}

pub struct LinkedList<T> {
    items: Vec<ListItem<T>>,
    start: Option<usize>,
    end: Option<usize>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            items: Vec::new(),
            start: None,
            end: None,
        }
    }
    pub fn push_back(&mut self, item: T) {
        let key = self.items.len();

        self.items.push(ListItem {
            next_id: None,
            prev_id: self.end,
            data: item,
        });

        if self.start.is_none() {
            self.start = Some(key);
        }

        if let Some(end) = self.end {
            self.items[end].next_id = Some(key);
        }

        self.end = Some(key);
    }
    pub fn push_front(&mut self, item: T) {
        let key = self.items.len();

        self.items.push(ListItem {
            next_id: self.start,
            prev_id: None,
            data: item,
        });

        if self.end.is_none() {
            self.end = Some(key);
        }

        if let Some(start) = self.start {
            self.items[start].prev_id = Some(key);
        }

        self.end = Some(key);
    }
    pub fn pop_back(&mut self) {
        if let Some(end) = self.end {
            let item = &mut self.items[end];

            match item.prev_id {
                Some(prev_id) => {
                    self.end = Some(prev_id);
                    self.items[prev_id].next_id = None;
                }
                None => self.end = None,
            }
        }
    }
    pub fn pop_front(&mut self) {
        if let Some(start) = self.start {
            let item = &mut self.items[start];

            match item.next_id {
                Some(next_id) => {
                    self.start = Some(next_id);
                    self.items[next_id].prev_id = None;
                }
                None => self.start = None,
            }
        }
    }
    pub fn iter<'a>(&'a mut self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            index: self.start.clone(),
            list: self,
        }
    }
    pub fn into_vec(mut self) -> Vec<T>
    where
        T: Debug
    {
        let mut out = Vec::new();
        let mut index_opt = self.start;

        while let Some(index) = index_opt {
            // println!("{}, {:?}", index, self.items[index]);
            // println!("{:?}", self.items);

            // Manage the pointers to the last item, since the last item will move
            {
                let prev_id = self.items[self.items.len() - 1].prev_id;
                let next_id = self.items[self.items.len() - 1].next_id;

                if let Some(prev_id) = prev_id {
                    self.items[prev_id].next_id = Some(index);
                }
                if let Some(next_id) = next_id {
                    self.items[next_id].prev_id = Some(index);
                }
            }

            let item = self.items.swap_remove(index);

            if let Some(start) = self.start {
                if start == index {
                    self.start = item.next_id;
                }
            }
            if let Some(end) = self.end {
                if end == index {
                    self.end = item.prev_id;
                }
            }
            if let Some(prev_id) = item.prev_id {
                self.items[prev_id].next_id = item.next_id;
            }
            if let Some(next_id) = item.next_id {
                self.items[next_id].prev_id = item.prev_id;
            }

            out.push(item.data);

            index_opt = item.next_id;
        }

        out
    }
}

pub struct LinkedListIter<'a, T> {
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}
impl<'a, T> LinkedListIter<'a, T> {
    pub fn next(&mut self) -> Option<&T> {
        self.index = self.list.items[self.index?].next_id;

        Some(&self.list.items[self.index?].data)
    }
    pub fn prev(&mut self) -> Option<&T> {
        self.index = self.list.items[self.index?].prev_id;

        Some(&self.list.items[self.index?].data)
    }
    pub fn next_mut(&mut self) -> Option<&mut T> {
        self.index = self.list.items[self.index?].next_id;

        Some(&mut self.list.items[self.index?].data)
    }
    pub fn prev_mut(&mut self) -> Option<&mut T> {
        self.index = self.list.items[self.index?].prev_id;

        Some(&mut self.list.items[self.index?].data)
    }
    pub fn peek_next(&mut self) -> Option<&T> {
        let index = self.list.items[self.index?].next_id?;

        Some(&self.list.items[index].data)
    }
    pub fn peek_prev(&mut self) -> Option<&T> {
        let index = self.list.items[self.index?].prev_id?;

        Some(&self.list.items[index].data)
    }
    pub fn delete(&mut self) -> Option<T> {
        let index = self.index?;

        // Manage the pointers to the last item, since the last item will move
        {
            let prev_id = self.list.items[self.list.items.len() - 1].prev_id;
            let next_id = self.list.items[self.list.items.len() - 1].next_id;

            if let Some(prev_id) = prev_id {
                self.list.items[prev_id].next_id = Some(index);
            }
            if let Some(next_id) = next_id {
                self.list.items[next_id].prev_id = Some(index);
            }
        }

        let item = self.list.items.swap_remove(index);

        if let Some(start) = self.list.start {
            if start == index {
                self.list.start = item.next_id;
            }
        }
        if let Some(end) = self.list.end {
            if end == index {
                self.list.end = item.prev_id;
            }
        }
        if let Some(prev_id) = item.prev_id {
            self.list.items[prev_id].next_id = item.next_id;
        }
        if let Some(next_id) = item.next_id {
            self.list.items[next_id].prev_id = item.prev_id;
        }

        self.index = item.prev_id.or(item.next_id);

        Some(item.data)
    }
}

const PARITY: u8 = b'a' - b'A';

fn solve_polymer(input: Vec<u8>) -> Vec<u8> {
    let mut polymer = LinkedList::new();
    input.into_iter().for_each(|b| polymer.push_back(b));

    let mut p_iter = polymer.iter();

    while let Some(&l1) = p_iter.next() {
        if let Some(&l2) = p_iter.peek_prev() {
            if l1 + PARITY == l2 || l1 == l2 + PARITY {
                p_iter.delete();
                p_iter.delete();
            }
        }
    }

    drop(p_iter);

    polymer.into_vec()
}

fn main() {
    let input = read_to_string("input").unwrap();

    let base_output = solve_polymer(input.trim().to_owned().into_bytes());

    let best_len = (b'A'..b'Z')
        .map(|byte_remove|
            base_output.iter()
                .map(|byte| *byte)
                .filter(|byte| *byte != byte_remove && *byte != byte_remove + PARITY)
                .collect::<Vec<_>>()
        )
        .map(|input| solve_polymer(input).len())
        .min()
        .unwrap();

    println!("Best polymer w/o byte: {}", best_len);
}
