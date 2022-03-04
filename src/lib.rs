use std::fmt::Debug;

#[derive(Debug)]
pub enum ErrDefine {
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
pub struct QuickArray<T: Sized + Default + Copy + Debug> {
    max_size: u32,
    free_head: u32,
    valid_head: u32,
    valid_tail: u32,
    valid_count: u32,
    internal_vec: Vec<QuickElement<T>>,
}

impl<T: Sized + Default + Copy + Debug> QuickArray<T> {
    const INVALID_INDEX: u32 = 1994090994;

    pub fn new(_max_size: u32) -> Self {
        assert!(_max_size < Self::INVALID_INDEX, "Quick array is too large to init!");
        if _max_size < 1 {
            let _max_size = 1;
        }
        let mut new_array = Self {
            max_size: _max_size,
            internal_vec: Vec::with_capacity(_max_size as usize),
            free_head: 0,
            valid_head: Self::INVALID_INDEX,
            valid_tail: Self::INVALID_INDEX,
            valid_count: 0,
        };

        for _ in 0.._max_size {
            new_array.internal_vec.push(QuickElement::<T>::default());
        }

        new_array.init();
        new_array
    }

    pub fn clear(&mut self) {
        self.free_head = 0;
        self.valid_head = Self::INVALID_INDEX;
        self.valid_tail = Self::INVALID_INDEX;
        self.valid_count = 0;

        self.init();
    }

    #[inline]
    pub fn get_valid_count(&self) -> u32 {
        self.valid_count
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.valid_count == self.max_size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.valid_count == 0
    }

    #[inline]
    pub fn get_max_size(&self) -> u32 {
        self.max_size
    }

    pub fn get_head_element(&self) -> Option<&T> {
        match self.valid_head {
            Self::INVALID_INDEX => None,
            _ => Some(&(self.internal_vec[self.valid_head as usize].data))
        }
    }

    pub fn get_tail_element(&self) -> Option<&T> {
        match self.valid_tail {
            Self::INVALID_INDEX => None,
            _ => Some(&(self.internal_vec[self.valid_tail as usize].data))
        }
    }

    pub fn get_head_index(&self) -> Option<u32> {
        match self.valid_head {
            Self::INVALID_INDEX => None,
            _ => Some(self.internal_vec[self.valid_head as usize].cur)
        }
    }

    pub fn get_tail_index(&self) -> Option<u32> {
        match self.valid_tail {
            Self::INVALID_INDEX => None,
            _ => Some(self.internal_vec[self.valid_tail as usize].cur)
        }
    }

    pub fn get_element(&self, index: u32) -> Option<&T> {
        if index >= self.max_size {
            return None;
        }

        let e = &(self.internal_vec[index as usize]);
        if !e.valid {
            None
        } else {
            Some(&e.data)
        }
    }

    pub fn get_pre_index(&self, index: u32) -> Option<u32> {
        if index >= self.max_size {
            return None;
        }

        let e = &(self.internal_vec[index as usize]);
        if !e.valid || e.pre == Self::INVALID_INDEX {
            None
        } else {
            Some(e.pre)
        }
    }

    pub fn get_next_index(&self, index: u32) -> Option<u32> {
        if index >= self.max_size {
            return None;
        }

        let e = &(self.internal_vec[index as usize]);
        if !e.valid || e.next == Self::INVALID_INDEX {
            None
        } else {
            Some(e.next)
        }
    }

    pub fn insert_before(&mut self, index: u32, data: &T) -> Result<u32, ErrDefine> {
        if index >= self.max_size {
            return Err(ErrDefine::InvalidIndex);
        }

        let target = &self.internal_vec[index as usize];
        assert_eq!(target.cur, index, "index calculation goes wrong");

        let target_valid = target.valid;
        let target_pre = target.pre;
        let target_cur = target.cur;

        if target_valid {
            let free_index = self.consume_ele();

            match free_index {
                Self::INVALID_INDEX => { Err(ErrDefine::ArrayIsFull) }
                _ => {
                    if self.valid_head == target_cur {
                        self.valid_head = free_index;
                    } else {
                        self.internal_vec[target_pre as usize].next = free_index;
                    }
                    self.internal_vec[free_index as usize].pre = target_pre;
                    self.internal_vec[free_index as usize].next = target_cur;
                    self.internal_vec[free_index as usize].data = *data;
                    self.internal_vec[target_cur as usize].pre = free_index;

                    Ok(free_index)
                }
            }
        } else {
            Err(ErrDefine::InvalidIndex)
        }
    }

    pub fn insert_after(&mut self, index: u32, data: &T) -> Result<u32, ErrDefine> {
        if index >= self.max_size {
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

    pub fn push_back(&mut self, data: &T) -> Result<u32, ErrDefine> {
        if self.valid_tail == Self::INVALID_INDEX {
            let free_index = self.consume_ele();
            match free_index {
                Self::INVALID_INDEX => { Err(ErrDefine::ArrayIsFull) }
                _ => {
                    self.internal_vec[free_index as usize].data = *data;
                    self.valid_tail = free_index;
                    self.valid_head = free_index;
                    Ok(free_index)
                }
            }
        } else {
            self.insert_after(self.valid_tail, data)
        }
    }

    pub fn push_front(&mut self, data: &T) -> Result<u32, ErrDefine> {
        if self.valid_head == Self::INVALID_INDEX {
            self.push_back(data)
        } else {
            let free_index = self.consume_ele();
            match free_index {
                Self::INVALID_INDEX => { Err(ErrDefine::ArrayIsFull) }
                _ => {
                    self.internal_vec[free_index as usize].data = *data;
                    self.internal_vec[free_index as usize].next = self.valid_head;
                    self.internal_vec[self.valid_head as usize].pre = free_index;
                    self.valid_head = free_index;
                    Ok(free_index)
                }
            }
        }
    }

    pub fn remove_at(&mut self, index: u32) -> Result<(), ErrDefine> {
        if index >= self.max_size {
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
        if index >= self.max_size {
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

    pub fn expand_to(&mut self, new_size: u32) -> Result<(), ErrDefine> {
        if new_size <= self.max_size || new_size >= Self::INVALID_INDEX {
            Err(ErrDefine::ArraySizeError)
        } else {
            let mut expand_vec: Vec<QuickElement<T>> = Vec::with_capacity(new_size as usize);
            for _ in 0..new_size {
                expand_vec.push(QuickElement::<T>::default());
            }

            for i in 0..self.max_size {
                expand_vec[i as usize] = self.internal_vec[i as usize];
            }

            for i in (self.max_size + 1)..(new_size - 1) {
                expand_vec[i as usize].pre = i - 1;
                expand_vec[i as usize].next = i + 1;
                expand_vec[i as usize].cur = i;
            }

            expand_vec[self.max_size as usize].pre = Self::INVALID_INDEX;
            expand_vec[self.max_size as usize].next = self.max_size + 1;
            expand_vec[self.max_size as usize].cur = self.max_size;

            expand_vec[new_size as usize - 1].pre = new_size - 2;
            expand_vec[new_size as usize - 1].next = self.free_head;
            expand_vec[new_size as usize - 1].cur = new_size - 1;

            self.internal_vec = expand_vec;
            self.free_head = self.max_size;
            self.max_size = new_size;

            Ok(())
        }
    }

    fn init(&mut self) {
        match self.max_size {
            1 => {
                self.internal_vec[0].pre = Self::INVALID_INDEX;
                self.internal_vec[0].next = Self::INVALID_INDEX;
                self.internal_vec[0].cur = 0;
            },
            _ => {
                for i in 1..(self.max_size - 1) {
                    self.internal_vec[i as usize].pre = (i as usize - 1) as u32;
                    self.internal_vec[i as usize].next = (i as usize + 1) as u32;
                    self.internal_vec[i as usize].cur = i;
                }

                self.internal_vec[0].pre = Self::INVALID_INDEX;
                self.internal_vec[0].next = 1;
                self.internal_vec[0].cur = 0;

                self.internal_vec[self.max_size as usize - 1].pre = self.max_size - 2;
                self.internal_vec[self.max_size as usize - 1].next = Self::INVALID_INDEX;
                self.internal_vec[self.max_size as usize - 1].cur = self.max_size - 1;
            }
        }

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

pub struct QuickArrayIterator<'a, T: Sized + Default + Copy + Debug> {
    pub array : &'a QuickArray<T>,
    pub index: u32,
}

impl<'a, T: Sized + Default + Copy + Debug> Iterator for QuickArrayIterator<'a, T> {
    type Item = (u32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cur_ele = self.array.get_element(self.index);
        let cur_index = self.index;
        let next_index = self.array.get_next_index(self.index);
        match next_index {
            Some(i) => { self.index = i; }
            None => { self.index = QuickArray::<T>::INVALID_INDEX; }
        }

        match cur_ele {
            None => { None }
            _ => { Some((cur_index, cur_ele.unwrap())) }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use std::borrow::Borrow;
    use std::fmt::Debug;
    use crate::quick_array::ErrDefine;

    fn display_array<T: Sized + Default + Copy + Debug>(array: &QuickArray<T>) {
        println!("{:?}", array);
        println!("=================================================");
    }

    #[test]
    fn it_works() {
        println!("array with 1 element init");
        let mut test_array = QuickArray::<u32>::new(1);

        println!("array init");
        let mut test_array = QuickArray::<u32>::new(5);
        display_array(&test_array);

        println!("array push 111");
        let result: Result<u32, ErrDefine> = test_array.push_back(111_u32.borrow());
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

        println!("array push 4 numbers");
        let result: Result<u32, ErrDefine> = test_array.push_back(444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push_front(4444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push_back(44444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push_back(444444_u32.borrow());
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push_back(4444444_u32.borrow());
        display_array(&test_array);

        println!("array push 6 111");
        let result: Result<u32, ErrDefine> = test_array.push_back(111_u32.borrow());
        display_array(&test_array);
        match result {
            Err(ErrDefine::ArrayIsFull) => { println!("Array is full") }
            _ => { () }
        }

        println!("array update 999 at 0");
        let result = test_array.update_at(0, 999_u32.borrow());
        display_array(&test_array);

        let ele = test_array.get_head_element();
        println!("head value is {}", ele.unwrap());

        let ele = test_array.get_tail_element();
        println!("tail value is {}", ele.unwrap());
        display_array(&test_array);

        println!("expand array to 10");
        test_array.expand_to(10);
        display_array(&test_array);

        let result: Result<u32, ErrDefine> = test_array.push_back(888_u32.borrow());
        display_array(&test_array);

        let result: Result<(), ErrDefine> = test_array.pop_last();
        display_array(&test_array);

        test_array.push_front(666_u32.borrow());
        display_array(&test_array);

        for (i, e) in test_array.enumerate() {
            println!("{}:{}", i, e)
        }
    }

    #[test]
    fn test_lru(){
        const LRU_LEN:u32=3;
        let mut array_obj= QuickArray::<i32>::new(LRU_LEN);
        let mut push_fn=|val: i32|{
            if array_obj.is_full(){
                {
                    let last_item= array_obj.get_tail_element().unwrap();
                    assert_eq!(*last_item, val-(LRU_LEN as i32));
                }
                array_obj.pop_last().expect("pop last error");
            }

            array_obj.push_front(&val).expect("push error");
            if val>=LRU_LEN as i32{
                assert_eq!( array_obj.get_valid_count(),LRU_LEN);
            }else{
                assert_eq!( array_obj.get_valid_count(),val as u32);
            }
        };

        push_fn(1);
        push_fn(2);
        push_fn(3);
        push_fn(4);
        push_fn(5);
        push_fn(6);
        push_fn(7);
        push_fn(8);
        push_fn(9);
        push_fn(10);
    }

    #[test]
    fn test_normal(){
        const LRU_LEN:u32=5;
        // 缓存未满的情况
        let total_data=vec![1,2];
        let mut array_obj= QuickArray::<i32>::new(LRU_LEN);
        for item in &total_data{
            array_obj.push_front(item).expect("push error");
        }
        let array_val:Vec<i32>= array_obj.enumerate().map(|item|*item.1) .collect();
        assert_eq!(total_data.len(),array_val.len());
        assert_eq!(total_data.len(),array_obj.get_valid_count() as usize);
        for index in 0..array_val.len(){
            assert_eq!(array_val[index],total_data[total_data.len()-index-1])
        }

        // 清空测试
        array_obj.clear();
        assert_eq!(array_obj.get_valid_count(),0);
        let array_val:Vec<i32>= array_obj.enumerate().map(|item|*item.1) .collect();
        assert_eq!(array_val.len(),0);

        // 链满的情况
        let total_data=vec![1,2,3,4,5,6,7,8,9];
        for item in &total_data{
            if array_obj.is_full(){
                array_obj.pop_last().expect("pop error");
            }
            array_obj.push_front(item).expect("push error");
        }
        let array_val:Vec<i32>= array_obj.enumerate().map(|item|*item.1) .collect();
        assert_eq!(array_val.len(),LRU_LEN as usize);
        assert_eq!(array_obj.get_valid_count (),LRU_LEN);
        for index in 0..array_val.len(){
            assert_eq!(array_val[index],total_data[total_data.len()-index-1])
        }
    }
}
