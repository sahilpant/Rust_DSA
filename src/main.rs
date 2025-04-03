use jemallocator::Jemalloc; #[cfg(not(target_env = "msvc"))] #[global_allocator] static GLOBAL: Jemalloc = Jemalloc;
use std::{io::{self, Read}, time::Instant};
pub mod two_sum;
pub mod best_time_to_sell_stock;
pub mod plus_one;
pub mod linear_search;
pub mod gcd;
pub mod reverse;
pub mod binary_search;
pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod link_list;
pub mod example_for_dynamic_dispatch;
pub mod link_list_2;
fn main() -> io::Result<()> {
    // println!("Enter the number to find GCD/HCF for::");
    // let mut input  = Vec::new();
    // for val in io::stdin().lines().take(2) {
    //     input.push(val.unwrap().parse::<usize>().unwrap());
    // }
    // let mut input2 = String::new();
    // io::stdin().read_line(&mut input2)?;
    // input2.pop();
    // println!("GCD/HCF of {:?} is => {}",input,find_gcd(input[0], input[1]));
    // println!("reversed string {} => {}",input2,reverse::reverse(input2.clone()));

    // Linear Search
    let t = Instant::now(); 
    let val: Vec<i32> = (0..1000000000).map(|x| x).collect();
    println!("{}",t.elapsed().as_micros());

    let t = Instant::now();
    let res = linear_search::linear_search(999999999, &val);
    println!("{}",t.elapsed().as_micros());
    println!("{:?}",res);
    let t = Instant::now();
    let res = binary_search::binary_search(999999999, &val);
    println!("{}",t.elapsed().as_micros());
    println!("{:?}",res);

    Ok(())
}

#[test]
fn benchmark_insertion_sort() {
    let val: Vec<i32> = (0..10000).rev().map(|x| x).collect();
    let t1 = Instant::now();
    let res = insertion_sort::insertion_sort(val.clone());
    println!("With Insertion Sort algo ==> {:?}",t1.elapsed().as_millis());
    let t2 = Instant::now();
    let res2 = bubble_sort::bubble_sort(val.clone());
    println!("With buble Rust's default algo ==> {:?}",t2.elapsed().as_millis());
    assert_eq!(res,res2);
}
