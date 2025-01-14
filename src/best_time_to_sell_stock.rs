use std::cmp::min_by;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit:i32 = 0;
    let mut min_price:i32 = prices[0];
    for x in prices {
        if max_profit < (x - min_price) {
            max_profit = x - min_price;
        }
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let temp = max_profit(vec![2,4,1]);

        println!("{temp}");
    }
    #[test]
    fn test2() {
        for x in (0..5).rev() {
            println!("{x}");
        }
    }
}