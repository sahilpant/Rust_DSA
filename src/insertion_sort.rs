pub fn insertion_sort<T>(mut list: Vec<T>) -> Vec<T> 

where 
    T: PartialOrd + Default
{
    let mut i = 1;
    let mut j = 0;
    while i < list.len() {
        j = i;
        while j > 0 {
            if list[j] < list [j - 1] {
                let tmp = std::mem::take(&mut list[j]);
                list[j] = std::mem::take(&mut list[j - 1]);
                list[j - 1] = tmp;
            }
            j -= 1;
        }
        i += 1;
    }
    list
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_insertion_sort_pass() {
        // let mut val = vec![9,8,7,6,5,4,3,2,1];
        let mut val = vec![2,1,3,4,5,6,7,8,9];
        let res = insertion_sort(val.clone());
        val.sort();
        assert_eq!(res,val);
    }

    #[test]
    fn benchmark_insertion_sort() {
        let mut val: Vec<i32> = (0..10000).rev().map(|x| x).collect();
        let t1 = Instant::now();
        let res = insertion_sort(val.clone());
        println!("With Insertion Sort algo ==> {:?}",t1.elapsed().as_millis());
        let t2 = Instant::now();
        val.sort();
        println!("With TimSort Rust's default algo ==> {:?}",t2.elapsed().as_millis());
        assert_eq!(res,val);
    }
}
