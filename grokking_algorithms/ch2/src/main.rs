fn main() {}

fn binary_search(search_val: i32, list: Vec<i32>) -> i32 {

}



#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn test() {
          assert_eq!(-1, binary_search(3, vec![]));
    }
}
