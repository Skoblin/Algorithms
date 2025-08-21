use std::cmp::Ordering;

fn main() {
// 

}

fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    if arr.is_empty() {
       return None;
    }

    if arr.len() == 1 {
        return if &arr[0] == target { Some(0) } else { None };
    }

    let mut first: usize = 0;
    let mut last: usize = arr.len() - 1;
    let mut mid: usize;

    while first <= last {
        mid = first + (last - first) / 2;
        match target.cmp(&arr[mid]) {
            Ordering::Less => {
                if mid == 0 {
                    break;
                }
                last = mid - 1
            },
            Ordering::Equal => return Some(mid),
            Ordering::Greater => first = mid + 1,
        }
    }
    None

}

#[cfg(test)]
mod tests {
    use super::*;
    
#[test]
    fn test_binary_search() {
        let mut arr: Vec<i32> = vec![77, 56, 84, 21, 10, 63, 5, 8, 56, 18, 98];
        let one_arr: Vec<i32> = vec![1];
        let empty_arr: Vec<i32> = vec![];
        arr.sort();
        println!("{:?}", arr);
        assert_eq!(binary_search(&arr, &5), Some(0));
        assert_eq!(binary_search(&arr, &84), Some(9));
        assert_eq!(binary_search(&arr, &100), None);
        assert_eq!(binary_search(&arr, &0), None); 
        assert_eq!(binary_search(&one_arr, &1), Some(0)); 
        assert_eq!(binary_search(&empty_arr, &5), None); 
    }

}