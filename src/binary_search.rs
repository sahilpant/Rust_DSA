use std::ops::{Add, Div};

pub fn binary_search<T>(key: T, list: &Vec<T>) -> Result<usize, bool>
where
    T: PartialEq + Add<Output =  T> + Div<Output = T> + PartialOrd,
{
    let mut low = 0;
    let mut high = list.len() - 1;
    let mut mid = (high + low) / 2;
    while high >= low {
        if list[mid] == key {
            return Ok(mid);
        } else if key > list[low] {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
        mid = (high + low) / 2;
    }
    Err(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let val = vec![1,4,6,8,3,4];
        let res = binary_search(8, &val);
        assert_eq!(Ok(3),res);
    }

    #[test]
    fn test_binary_search_fail() {
        let val = vec![1,4,6,8,3,4];
        let res = binary_search(9, &val);
        assert_eq!(Err(false),res);
    }
}