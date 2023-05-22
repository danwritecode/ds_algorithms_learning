fn main() {}

fn binary_search(search_val: i32, list: Vec<i32>) -> i32 {
    if list.len() == 0 {
        return -1;
    }

    let mut low_index = 0;
    let mut high_index = list.len() - 1;

    while low_index <= high_index {
        let mid_index = (low_index + high_index) / 2;
        let guess = list[mid_index];

        if guess == search_val {
            return mid_index as i32;
        }

        // if guess is greater than the search val, then set high index to mid -1
        if guess > search_val {
            high_index = mid_index - 1; // don't need to include mid since we checked it with guess
            continue;
        }

        if guess < search_val {
            low_index = mid_index + 1; // don't need to include mid since we checked it with guess
        }
    }

    return -1;
}



#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn test() {
          assert_eq!(-1, binary_search(3, vec![]));
          assert_eq!(-1, binary_search(3, vec![1]));
          assert_eq!(0,  binary_search(1, vec![1]));
          assert_eq!(0,  binary_search(1, vec![1, 3, 5]));
          assert_eq!(2,  binary_search(5, vec![1, 3, 5]));
          assert_eq!(1,  binary_search(3, vec![1, 3, 5]));
          assert_eq!(2,  binary_search(5, vec![1, 3, 5]));
          assert_eq!(-1, binary_search(0, vec![1, 3, 5]));
          // assert_eq!(-1, binary_search(2, vec![1, 3, 5]));
          // assert_eq!(-1, binary_search(4, vec![1, 3, 5]));
          // assert_eq!(-1, binary_search(6, vec![1, 3, 5]));
          // assert_eq!(0,  binary_search(1, vec![1, 3, 5, 7]));
          // assert_eq!(1,  binary_search(3, vec![1, 3, 5, 7]));
          // assert_eq!(2,  binary_search(5, vec![1, 3, 5, 7]));
          // assert_eq!(3,  binary_search(7, vec![1, 3, 5, 7]));
          // assert_eq!(-1, binary_search(0, vec![1, 3, 5, 7]));
          // assert_eq!(-1, binary_search(2, vec![1, 3, 5, 7]));
          // assert_eq!(-1, binary_search(4, vec![1, 3, 5, 7]));
          // assert_eq!(-1, binary_search(6, vec![1, 3, 5, 7]));
          // assert_eq!(-1, binary_search(8, vec![1, 3, 5, 7]));
    }
}
