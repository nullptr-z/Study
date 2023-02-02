use std::{borrow::BorrowMut, mem::swap};

pub struct QuickSort<'a, T, F>
where
    F: Fn(&T, &T) -> bool,
{
    inner: &'a mut Vec<T>,
    predicate: F,
}

impl<'a, T, F> QuickSort<'a, T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    pub fn new(init: &'a mut Vec<T>, predicate: F) -> Self {
        QuickSort {
            inner: init,
            predicate,
        }
    }
    // 7 2 5 6 3 8 9 1
    fn divide_conquer(&mut self, start: usize, end: usize) {
        let mut start_index = start;
        let mut end_index = end;

        let seed = self.inner[start_index].clone();
        let mut seed_index = start_index;

        let mut move_status = false;

        while start_index < end_index {
            match &move_status {
                true => {
                    start_index += 1;
                    let compare = (self.predicate)(&seed, &self.inner[start_index]);
                    if compare {
                        self.inner[seed_index] = self.inner[start_index].clone();
                        seed_index = start_index;
                        move_status = false;
                    }
                }
                false => {
                    let compare = (self.predicate)(&self.inner[end_index], &seed);
                    if compare {
                        self.inner[seed_index] = self.inner[end_index].clone();
                        seed_index = end_index;
                        move_status = true;
                    }
                    end_index -= 1;
                }
            }
        }
        self.inner[seed_index] = seed;
        if seed_index > 1 && start < seed_index - 1 {
            self.divide_conquer(start, seed_index - 1);
        }
        if (seed_index + 1) < end {
            self.divide_conquer(seed_index + 1, end);
        }
    }

    pub fn sort(&mut self) -> Vec<T> {
        let start_index: usize = 0;
        let end_index = self.inner.len() - 1;
        if end_index > 1 {
            self.divide_conquer(start_index, end_index);
        }

        self.inner.to_vec()
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::sort::{get_gen_range_array, Message};

    use super::QuickSort;

    #[test]
    fn test_quick_sort() {
        let mut array = get_gen_range_array(10, |num| {
            Message::new(num, format!("wawa-{}", num).as_str())
        });

        let mut sort = QuickSort::new(&mut array, |f, b| f.num < b.num);

        let now = Instant::now();

        sort.sort();

        let consume_time_micros = now.elapsed().as_micros();
        let consume_time_millis = now.elapsed().as_millis();

        for (index, item) in array.iter().enumerate() {
            assert_eq!(item.num, index);
        }
    }
}
