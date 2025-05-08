/// Merge two sorted slices into a single sorted Vec<i32>.
pub fn merge(x: &[i32], y: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(x.len() + y.len());
    let (mut i, mut j) = (0, 0);

    // 1) consume both until one runs out
    while i < x.len() && j < y.len() {
        if x[i] <= y[j] {
            out.push(x[i]);
            i += 1;
        } else {
            out.push(y[j]);
            j += 1;
        }
    }

    // 2) append whatever's left (only **once**)
    if i < x.len() {
        out.extend_from_slice(&x[i..]);
    }
    if j < y.len() {
        out.extend_from_slice(&y[j..]);
    }

    out
}

/// Sort a slice using merge sort, returning a new Vec<i32>.
pub fn merge_sort(x: &[i32]) -> Vec<i32> {
    if x.len() <= 1 {
        return x.to_vec();
    }
    let mid = x.len() / 2;
    // x[..mid] is already a &[i32], so pass it directly:
    let left  = merge_sort(&x[..mid]);
    let right = merge_sort(&x[mid..]);
    merge(&left, &right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_both_empty() {
        assert_eq!(merge(&[], &[]), vec![]);
    }

    #[test]
    fn test_merge_left_empty() {
        assert_eq!(merge(&[], &[1, 3, 5]), vec![1, 3, 5]);
    }

    #[test]
    fn test_merge_right_empty() {
        assert_eq!(merge(&[2, 4, 6], &[]), vec![2, 4, 6]);
    }

    #[test]
    fn test_merge_interleaved() {
        assert_eq!(
            merge(&[1, 4, 7], &[2, 3, 5, 6, 8]),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn test_merge_with_duplicates() {
        assert_eq!(
            merge(&[1, 2, 2, 4], &[2, 3, 4]),
            vec![1, 2, 2, 2, 3, 4, 4]
        );
    }

    #[test]
    fn test_merge_sort_empty() {
        let a: Vec<i32> = vec![];
        assert_eq!(merge_sort(&a), vec![]);
    }

    #[test]
    fn test_merge_sort_single() {
        let a = vec![42];
        assert_eq!(merge_sort(&a), vec![42]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(&a), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse() {
        let a = vec![5, 4, 3, 2, 1];
        assert_eq!(merge_sort(&a), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_random() {
        let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        assert_eq!(merge_sort(&a), vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}