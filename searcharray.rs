
use std::cmp::Ordering;

fn main() {
    let arr:[i32;11] = [-1, 1, 2, 3, 4, 5, 6, 7, 8, 42 ,132];
    let result = bin_search(&arr, 5);
    match result {
        Some((found_value, found_index)) => println!("found {} at index {}", found_value, found_index),
        None => println!("not found")
    }
}
fn bin_search(arr: &[i32], target: i32) -> Option<(i32,usize)>  {
    if arr.is_empty() {
        return None;
    }
    let mut min:usize = 0;
    let mut max:usize = arr.len() - 1;
    while min <= max {
        let mid = (min + max) / 2;
        let mid_value = arr[mid];
        match mid_value.cmp(&target) {
            Ordering::Equal => return Some((mid_value, mid)),
            Ordering::Greater => {
                max = match mid.checked_sub(1) {
                    Some(result) => result,
                    _ => return None
                }
            },
            Ordering::Less => min = mid + 1,
        }
    }

    None
}
#[cfg(test)]
mod tests {
    use super::*;

    const ARR: [i32; 10] = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];

    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }

    #[test]
    fn element_not_found() {
        let result = bin_search(&ARR, 1234);

        assert!(result.is_none());
    }

    #[test]
    fn smallest_element_not_found() {
        let result = bin_search(&ARR, -100);

        assert!(result.is_none());
    }

    #[test]
    fn empty_array() {
        let result: Option<(i32, usize)> = bin_search(&[], -2);
        assert!(result.is_none());
    }
}

