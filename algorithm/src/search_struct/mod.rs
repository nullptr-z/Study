mod binary_tree;
mod fifo;
mod hash;
mod list;
mod priority_queue;

pub use binary_tree::*;
pub use fifo::*;
pub use hash::*;
pub use priority_queue::*;

fn get_index<T: PartialEq>(vec: &[T], val: &T) -> Option<usize> {
    let mut index = None;
    for (i, item) in vec.iter().enumerate() {
        if *item == *val {
            index = Some(i);
        }
    }

    index
}
