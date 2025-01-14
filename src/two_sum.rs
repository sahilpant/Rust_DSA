pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut temp: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut ctr = 0;
    let mut ans: Vec<i32> = Vec::new();
    for x in nums {
        if temp.get(&(target - x)).is_some() {
            ans.push(ctr);
            ans.push(*temp.get(&(target - x)).unwrap());
            break;
        } else {
            temp.insert(x, ctr);
            ctr += 1;
        }
    }
    ans
}