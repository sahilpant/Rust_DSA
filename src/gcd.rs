pub fn find_gcd(val1:usize, val2:usize) -> usize{
    let mut min = 0; 
    if val1 < val2 { min = val1 } else { min = val2 };
    let mut gcd = 1;
    for i in 2..min+1 {
        if val1%i == 0 && val2%i == 0 {
            gcd = i;
        }
        println!("{i}")
    }
    gcd
}