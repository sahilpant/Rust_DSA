pub fn bubble_sort<T>(mut list: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Default,
{
    let length = list.len() - 1;
    let mut i = 0;
    let mut j = 0;
    let mut flag = true;
    let mut ctr = 0;
    while i <= length {
        ctr+=1;
        j = 0;
        flag = true;
        while j < length - i {
            if list[j] > list[j+1]  {
                let tmp = std::mem::take(&mut list[j]);
                list[j] = std::mem::take(&mut list[j+1]);
                list[j+1] = tmp;
                flag = false;
            }
            j += 1;
        }
        if flag {
            break
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
    fn test_bubble_sort_pass() {
        // let mut val = vec![9,8,7,6,5,4,3,2,1];
        let mut val = vec![2,1,3,4,5,6,7,8,9];
        let res = bubble_sort(val.clone());
        val.sort();
        assert_eq!(res,val);
    }

    #[test]
    fn benchmark_bubble_sort() {
        let mut val: Vec<i32> = (0..10000).rev().map(|x| x).collect();
        let t1 = Instant::now();
        let res = bubble_sort(val.clone());
        println!("With Bubble Sort algo ==> {:?}",t1.elapsed().as_millis());
        let t2 = Instant::now();
        val.sort();
        println!("With TimSort Rust's default algo ==> {:?}",t2.elapsed().as_millis());
        assert_eq!(res,val);
    }
}
