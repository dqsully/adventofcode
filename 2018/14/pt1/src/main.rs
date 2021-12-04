#[derive(Debug)]
pub struct ListItem<T> {
    next_id: usize,
    prev_id: usize,
    data: T,
}

pub struct LinkedList<T> {
    items: Vec<ListItem<T>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            items: Vec::new(),
        }
    }
    pub fn iter<'a>(&'a mut self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            index: {
                if self.items.len() > 0 {
                    Some(0)
                } else {
                    None
                }
            },
            list: self,
        }
    }
}

pub struct IndexId(usize);

pub struct LinkedListIter<'a, T> {
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
    indexes: Vec<usize>,
}
impl<'a, T> LinkedListIter<'a, T> {
    pub fn new_index(&mut self) -> Option<IndexId> {
        let index = self.index?;

        self.indexes.push(index);

        Some(IndexId(self.indexes.len() - 1))
    }
    pub fn use_index(&mut self, id: IndexId) {
        self.index = self.indexes.get(id.0).map(|i| *i);
    }

    pub fn current(&self) -> Option<&T> {
        self.list.items.get(self.index?).map(|i| &i.data)
    }
    pub fn next(&mut self) -> Option<&T> {
        self.index = Some(self.list.items[self.index?].next_id);

        Some(&self.list.items[self.index?].data)
    }
    pub fn prev(&mut self) -> Option<&T> {
        self.index = Some(self.list.items[self.index?].prev_id);

        Some(&self.list.items[self.index?].data)
    }
    pub fn peek_next(&mut self) -> Option<&T> {
        let index = self.list.items[self.index?].next_id;

        Some(&self.list.items[index].data)
    }
    pub fn peek_prev(&mut self) -> Option<&T> {
        let index = self.list.items[self.index?].prev_id;

        Some(&self.list.items[index].data)
    }
    pub fn peek(&self) -> Option<&T> {
        Some(&self.list.items[self.index?].data)
    }
    pub fn delete(&mut self) -> Option<T> {
        let index = self.index?;

        // Manage the pointers to the last item, since the last item will move
        {
            let prev_id = self.list.items[self.list.items.len() - 1].prev_id;
            let next_id = self.list.items[self.list.items.len() - 1].next_id;

            self.list.items[prev_id].next_id = index;
            self.list.items[next_id].prev_id = index;
        }

        let item = self.list.items.swap_remove(index);

        if item.next_id != index {
            self.list.items[item.prev_id].next_id = item.next_id;
            self.list.items[item.next_id].prev_id = item.prev_id;

            self.index = Some(item.prev_id);
        } else {
            self.index = None;
        }

        Some(item.data)
    }
    pub fn insert_after(&mut self, data: T) {
        match self.index {
            None => {
                self.list.items.push(ListItem {
                    data,
                    next_id: 0,
                    prev_id: 0,
                });

                self.index = Some(0);
            },
            Some(index) => {
                let new_id = self.list.items.len();

                let next_id = {
                    let before = &mut self.list.items[index];
                    let ret = before.next_id;

                    before.next_id = new_id;

                    ret
                };
                self.list.items[next_id].prev_id = new_id;

                self.list.items.push(ListItem {
                    data,
                    next_id,
                    prev_id: index,
                });

                self.index = Some(new_id);
            }
        }
    }
    pub fn insert_before(&mut self, data: T) {
        match self.index {
            None => {
                self.list.items.push(ListItem {
                    data,
                    next_id: 0,
                    prev_id: 0,
                });

                self.index = Some(0);
            },
            Some(index) => {
                let new_id = self.list.items.len();

                let prev_id = {
                    let after = &mut self.list.items[index];
                    let ret = after.prev_id;

                    after.prev_id = new_id;

                    ret
                };
                self.list.items[prev_id].prev_id = new_id;

                self.list.items.push(ListItem {
                    data,
                    next_id: index,
                    prev_id,
                });

                self.index = Some(new_id);
            }
        }
    }
}

fn main() {
    let mut recipies = LinkedList::new();
    let mut recipe_iter = recipies.iter();

    recipe_iter.insert_after(3);
    recipe_iter.insert_after(7);


    println!("{:?}", recipe_iter.current());
}
