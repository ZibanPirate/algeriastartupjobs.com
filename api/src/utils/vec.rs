pub fn sort_and_dedup_vec<T: Ord + Clone>(vec: &mut Vec<T>) {
    vec.sort();
    vec.dedup();
}
