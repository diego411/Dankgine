pub fn get_two_mut<'a, T>(
    i: usize,
    k: usize,
    vec: &'a mut Vec<T>,
) -> Option<(&'a mut T, &'a mut T)> {
    let vec_length = vec.len();
    if i == k {
        return None;
    } else if i >= vec_length || k >= vec_length {
        return None;
    }

    if i < k {
        //we want i in the left half since k will be in the right
        let (left, right) = vec.split_at_mut(i + 1);
        return Some((left.last_mut().unwrap(), right.get_mut(k - i - 1).unwrap()));
    } else {
        //i > k
        //we want i in the right half since k will be in the left
        let (left, right) = vec.split_at_mut(i);
        return Some((right.first_mut().unwrap(), left.get_mut(k).unwrap()));
    }
}
