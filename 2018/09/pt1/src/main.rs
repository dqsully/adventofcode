use std::collections::BTreeSet;
use std::fmt::Debug;

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

pub struct LinkedListIter<'a, T> {
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}
impl<'a, T> LinkedListIter<'a, T> {
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
    const PLAYERS: usize = 413;
    const FINAL_MARBLE: u32 = 71082;
    // const PLAYERS: usize = 10;
    // const FINAL_MARBLE: u32 = 1618;

    let mut marbles = LinkedList::new();
    let mut marble_iter = marbles.iter();

    let mut next_marble: u32 = 0;
    let mut scores: [u32; PLAYERS] = [0; PLAYERS];
    let mut player: usize = PLAYERS - 1;

    while next_marble <= FINAL_MARBLE {
        if next_marble % 23 == 0 && next_marble > 0 {
            scores[player] += next_marble;

            for _ in 0..7 {
                marble_iter.prev();
            }

            let scored = marble_iter.delete().expect("can delete %23 marble");
            scores[player] += scored;

            // marble_iter.insert_after(*marble_iter.peek().expect("marbles left after delete") + 1);
            marble_iter.next();
        } else {
            marble_iter.next();
            marble_iter.insert_after(next_marble);
        }

        next_marble += 1;
        player = (player + 1) % PLAYERS;

        // {
        //     let start_id = 0;
        //     let mut cur_id = start_id;
        //
        //     print!("[{}]\t", next_marble);
        //
        //     loop {
        //         if cur_id == marble_iter.index.unwrap() {
        //             print!("\x1b[1m");
        //         }
        //
        //         print!("{}\t", marble_iter.list.items[cur_id].data);
        //
        //         if cur_id == marble_iter.index.unwrap() {
        //             print!("\x1b[0m");
        //         }
        //
        //
        //         cur_id = marble_iter.list.items[cur_id].next_id;
        //
        //         if start_id == cur_id {
        //             break;
        //         }
        //     }
        //
        //     println!();
        // }
    }

    println!("scores: {:?}", scores.iter().max());
}
