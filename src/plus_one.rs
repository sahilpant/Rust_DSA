pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let len: usize = digits.len() - 1;
    let mut ans: Vec<i32> = Vec::new();
    if digits[len] != 9 {
        ans = digits;
        ans[len] += 1
    }
    else {
        let mut ctr: usize = 0;
        for x  in (0..digits.len()).rev()  {
            if digits[x] != 9 { 
                break;
            }
            ctr += 1;
        }
        if ctr == digits.len() {
            ans.push(1);
            for x in 0..digits.len() {
                ans.push(0);
            }
        } else {
            for x in 0..digits.len() {
                if x <= (digits.len() - ctr) - 1 {
                    if x == (digits.len() - ctr) - 1 {
                        ans.push(digits[x] + 1);
                        continue;
                    }
                    ans.push(digits[x]);
                    continue;
                }
                ans.push(0);
            }
        }
        
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let x = plus_one(vec![8,9,9]);
        println!("{:?}",x);
    }
}