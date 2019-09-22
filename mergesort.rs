

fn mergesort<T: Ord + Clone>(array: &mut [T]) {
    let mut size: usize = 2;
    let mut memory = Vec::with_capacity(size.next_power_of_two() / 2);
    while size <= array.len().next_power_of_two() {
        for chunk in array.chunks_mut(size) {
            let split = chunk.len().min(size / 2);
            let (left_iter, right_iter) = chunk.split_at(split);
            let mut left_iter = left_iter.iter().peekable();
            let mut right_iter = right_iter.iter().peekable();
            while let (Some(&a), Some(&b)) = (left_iter.peek(), right_iter.peek()) {
                if a < b {
                    memory.push(a.clone());
                    left_iter.next();
                } else {
                    memory.push(b.clone());
                    right_iter.next();
                }
            }
            memory.extend(left_iter.cloned());
            memory.extend(right_iter.cloned());
            chunk.swap_with_slice(&mut memory[..]);
            memory.clear();
        }
        size = size * 2;
    }
}


fn main() {
    let mut what = vec![8,3,7,2,4,3,5,8,1];
    mergesort(&mut what[..]);
    println!{"{:?}", what};
}