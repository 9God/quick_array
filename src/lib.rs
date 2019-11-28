use std::fmt::Debug;

enum ErrDefine {
    InvalidIndex = 1,
    ArrayIsFull = 2,
    ArrayIsEmpty = 3,
    ArraySizeError = 4,
}

#[derive(Default, Copy, Clone, Debug)]
struct QuickElement<T: Sized + Default + Copy + Debug> {
    pub data: T,
    pub pre: u32,
    pub next: u32,
    pub cur: u32,
    pub valid: bool,
}

#[derive(Debug)]
struct QuickArray<T: Sized + Default + Copy + Debug> {
    max_length: u32,
    free_head: u32,
    valid_head: u32,
    valid_tail: u32,
    valid_count: u32,
    cur_iter_index: u32,
    internal_vec: Vec<QuickElement<T>>,
}

impl<T: Sized + Default + Copy + Debug> QuickArray<T> {
    const INVALID_INDEX: u32 = 1994090994;

    pub fn new(max_len: u32) -> Self {
        assert!(max_len < Self::INVALID_INDEX, "Quick array is too large to init!");
        let mut new_array = Self {
            max_length: max_len,
            internal_vec: Vec::with_capacity(max_len as usize),
            free_head: 0,
            valid_head: Self::INVALID_INDEX,
            valid_tail: Self::INVALID_INDEX,
            valid_count: 0,
            cur_iter_index: Self::INVALID_INDEX,
        };

        for _ in 0..max_len {
            new_array.internal_vec.push(QuickElement::<T>::default());
        }

        new_array.init();
        new_array
    }

    pub fn get_valid_count(&self) -> u32 {
        self.valid_count
    }

    pub fn get_max_size(&self) -> u32 {
        self.max_length
    }

    pub fn get_head_element(&self) -> Option<&QuickElement<T>> {
        match self.valid_head {
            Self::INVALID_INDEX => None,
            _ => Some(&(self.internal_vec[self.valid_head as usize]))
        }
    }

    pub fn get_tail_element(&self) -> Option<&QuickElement<T>> {
        match self.valid_tail {
            Self::INVALID_INDEX => None,
            _ => Some(&(self.internal_vec[self.valid_tail as usize]))
        }
    }

    pub fn get_element(&self, index: u32) -> Option<&QuickElement<T>> {
        if index >= self.max_length {
            return None;
        }

        let e = &(self.internal_vec[index as usize]);
        if !e.valid {
            None
        } else {
            Some(&e)
        }
    }

    pub fn get_next_index(&self, index: u32) -> Option<u32> {
        if index >= self.max_length {
            return None;
        }

        let e = &(self.internal_vec[index as usize]);
        if !e.valid {
            None
        } else {
            Some(e.next)
        }
    }

    pub fn insert_after(&mut self, index: u32, data: &T) -> Result<u32, ErrDefine> {
        if index >= self.max_length {
            return Err(ErrDefine::InvalidIndex);
        }

        let target = &self.internal_vec[index as usize];
        assert_eq!(target.cur, index, "index calculation goes wrong");

        let target_valid = target.valid;
        let target_next = target.next;
        let target_cur = target.cur;

        if target_valid {
            let free_index = self.consume_ele();

            match free_index {
                Self::INVALID_INDEX => { Err(ErrDefine::ArrayIsFull) }
                _ => {
                    if self.valid_tail == target_cur {
                        self.valid_tail = free_index;
                    } else {
                        self.internal_vec[target_next as usize].pre = free_index;
                    }
                    self.internal_vec[free_index as usize].pre = target_cur;
                    self.internal_vec[free_index as usize].next = target_next;
                    self.internal_vec[free_index as usize].data = *data;
                    self.internal_vec[target_cur as usize].next = free_index;

                    Ok(free_index)
                }
            }
        } else {
            Err(ErrDefine::InvalidIndex)
        }
    }

    pub fn push(&mut self, data: &T) -> Result<u32, ErrDefine> {
        if self.valid_tail == Self::INVALID_INDEX {
            let free_index = self.consume_ele();
            match free_index {
                Self::INVALID_INDEX => { Err(ErrDefine::ArrayIsFull) }
                _ => {
                    self.internal_vec[free_index as usize].data = *data;
                    self.valid_tail = free_index;
                    self.valid_head = free_index;
                    self.cur_iter_index = free_index;
                    Ok(free_index)
                }
            }
        } else {
            self.insert_after(self.valid_tail, data)
        }
    }

    pub fn remove_at(&mut self, index: u32) -> Result<(), ErrDefine> {
        if index >= self.max_length {
            return Err(ErrDefine::InvalidIndex);
        }

        let target = &self.internal_vec[index as usize];
        assert_eq!(target.cur, index, "index calculation goes wrong");

        let target_valid = target.valid;
        let target_pre = target.pre;
        let target_next = target.next;
        let target_cur = target.cur;

        if target_valid {
            if self.valid_head == target_cur {
                self.valid_head = target_next;
            }

            if self.valid_tail == target_cur {
                self.valid_tail = target_pre;
            }

            if self.cur_iter_index == target_cur {
                self.cur_iter_index = target_next;
            }

            self.recycle_ele(target_cur);

            Ok(())
        } else {
            Err(ErrDefine::InvalidIndex)
        }
    }

    pub fn pop_last(&mut self) -> Result<(), ErrDefine> {
        if self.valid_tail == Self::INVALID_INDEX {
            Err(ErrDefine::ArrayIsEmpty)
        } else {
            self.remove_at(self.valid_tail)
        }
    }

    pub fn update_at(&mut self, index: u32, data: &T) -> Result<(), ErrDefine> {
        if index >= self.max_length {
            return Err(ErrDefine::InvalidIndex);
        }

        let target = &mut self.internal_vec[index as usize];
        assert_eq!(target.cur, index, "index calculation goes wrong");

        if target.valid {
            target.data = *data;
            Ok(())
        } else {
            Err(ErrDefine::InvalidIndex)
        }
    }

    pub fn reset_iterator(&mut self) {
        self.cur_iter_index = self.valid_head;
    }

    pub fn expand_to(&mut self, new_length: u32) -> Result<(), ErrDefine> {
        if new_length <= self.max_length || new_length >= Self::INVALID_INDEX {
            Err(ErrDefine::ArraySizeError)
        } else {
            let mut expand_vec: Vec<QuickElement<T>> = Vec::with_capacity(new_length as usize);
            for _ in 0..new_length {
                expand_vec.push(QuickElement::<T>::default());
            }

            for i in 0..self.max_length {
                expand_vec[i as usize] = self.internal_vec[i as usize];
            }

            for i in (self.max_length + 1)..(new_length - 1) {
                expand_vec[i as usize].pre = i - 1;
                expand_vec[i as usize].next = i + 1;
                expand_vec[i as usize].cur = i;
            }

            expand_vec[self.max_length as usize].pre = Self::INVALID_INDEX;
            expand_vec[self.max_length as usize].next = self.max_length + 1;
            expand_vec[self.max_length as usize].cur = self.max_length;

            expand_vec[new_length as usize - 1].pre = new_length - 2;
            expand_vec[new_length as usize - 1].next = self.free_head;
            expand_vec[new_length as usize - 1].cur = new_length - 1;

            self.internal_vec = expand_vec;
            self.free_head = self.max_length;
            self.max_length = new_length;

            Ok(())
        }
    }

    fn init(&mut self) {
        for i in 1..(self.max_length - 1) {
            self.internal_vec[i as usize].pre = (i as usize - 1) as u32;
            self.internal_vec[i as usize].next = (i as usize + 1) as u32;
            self.internal_vec[i as usize].cur = i;
        }

        self.internal_vec[0].pre = Self::INVALID_INDEX;
        self.internal_vec[0].next = 1;
        self.internal_vec[0].cur = 0;

        self.internal_vec[self.max_length as usize - 1].pre = self.max_length - 2;
        self.internal_vec[self.max_length as usize - 1].next = Self::INVALID_INDEX;
        self.internal_vec[self.max_length as usize - 1].cur = self.max_length - 1;
    }

    fn recycle_ele(&mut self, index: u32) {
        let target_pre = self.internal_vec[index as usize].pre;
        let target_next = self.internal_vec[index as usize].next;

        if target_pre != Self::INVALID_INDEX {
            self.internal_vec[target_pre as usize].next = target_next;
        }

        if target_next != Self::INVALID_INDEX {
            self.internal_vec[target_next as usize].pre = target_pre;
        }

        self.internal_vec[index as usize].pre = Self::INVALID_INDEX;
        self.internal_vec[index as usize].next = self.free_head;
        self.internal_vec[index as usize].valid = false;

        if self.free_head != Self::INVALID_INDEX {
            self.internal_vec[self.free_head as usize].pre = index;
        }
        self.free_head = index;
        self.valid_count -= 1;
    }

    fn consume_ele(&mut self) -> u32 {
        if self.free_head ==  Self::INVALID_INDEX {
            Self::INVALID_INDEX
        } else {
            let free_real_index = self.free_head;
            self.free_head = self.internal_vec[free_real_index as usize].next as u32;

            if self.free_head != Self::INVALID_INDEX {
                self.internal_vec[self.free_head as usize].pre = Self::INVALID_INDEX;
            }

            self.internal_vec[free_real_index as usize].next = Self::INVALID_INDEX;
            self.internal_vec[free_real_index as usize].valid = true;
            self.valid_count += 1;
            free_real_index
        }
    }

    pub fn enumerate<'life_of_array> (self: &'life_of_array Self) -> QuickArrayIterator<'life_of_array, T> {
        QuickArrayIterator::<'life_of_array, T> {
            array: self,
            index: self.valid_head,
        }
    }
}

struct QuickArrayIterator<'a, T: Sized + Default + Copy + Debug> {
    pub array : &'a QuickArray<T>,
    pub index: u32,
}

impl<'a, T: Sized + Default + Copy + Debug> Iterator for QuickArrayIterator<'a, T> {
    type Item = (u32, T);

    fn next(&mut self) -> Option<Self::Item> {
        let cur_ele = self.array.get_element(self.index);
        let cur_index = self.index;
        let next_index = self.array.get_next_index(self.index);
        match next_index {
            Some(i) => { self.index = i; }
            None => {}
        }

        match cur_ele {
            None => { None }
            _ => { Some((cur_index, cur_ele.unwrap().data)) }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use std::borrow::Borrow;

    fn display_array<T: Sized + Default + Copy + Debug>(array: &QuickArray<T>) {
        println!("{:?}", array);
        println!("=================================================");
    }

    #[test]
    fn it_works() {
        println!("array init");
        let mut test_array = QuickArray::<u32>::new(5);
        display_array(&test_array);

        println!("array push 111");
        let result: Result<u32, ErrDefine> = test_array.push(111_u32.borrow());
        display_array(&test_array);

        println!("array insert 222 after 0");
        let result: Result<u32, ErrDefine> = test_array.insert_after(0, 222_u32.borrow());
        display_array(&test_array);

        println!("array insert 333 after 0");
        let result: Result<u32, ErrDefine> = test_array.insert_after(0, 333_u32.borrow());
        display_array(&test_array);

        println!("array remove at 1");
        let result: Result<(), ErrDefine> = test_array.remove_at(1);
        display_array(&test_array);

        println!("array pop last");
        let result: Result<(), ErrDefine> = test_array.pop_last();
        display_array(&test_array);

        println!("array pop last");
        let result: Result<(), ErrDefine> = test_array.pop_last();
        display_array(&test_array);

        println!("array pop last");
        let result: Result<(), ErrDefine> = test_array.pop_last();
        display_array(&test_array);
        match result {
            Err(ErrDefine::ArrayIsEmpty) => { println!("Array is empty") }
            _ => { () }
        }

        println!("array push 5 111");
        let result: Result<u32, ErrDefine> = test_array.push(444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push(4444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push(44444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push(444444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push(4444444_u32.borrow());
        display_array(&test_array);

        println!("array push 6 111");
        let result: Result<u32, ErrDefine> = test_array.push(111_u32.borrow());
        display_array(&test_array);
        match result {
            Err(ErrDefine::ArrayIsFull) => { println!("Array is full") }
            _ => { () }
        }

        println!("array update 999 at 0");
        let result = test_array.update_at(0, 999_u32.borrow());
        display_array(&test_array);

        test_array.reset_iterator();

        let ele = test_array.get_head_element();
        println!("head value is {}", ele.unwrap().data);

        let ele = test_array.get_tail_element();
        println!("tail value is {}", ele.unwrap().data);

        display_array(&test_array);

        println!("expand array to 10");
        test_array.expand_to(10);
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push(888_u32.borrow());
        display_array(&test_array);

        let result: Result<(), ErrDefine> = test_array.pop_last();
        display_array(&test_array);

        for (i, e) in test_array.enumerate() {
            println!("{}:{}", i, e)
        }
    }
}