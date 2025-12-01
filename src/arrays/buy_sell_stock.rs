use std::cmp::{min, max};

pub fn optimize_approach(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut best_buy = prices[0];

    for i in 1..prices.len() {
        if prices[i] > best_buy {
            let today_profit = prices[i] - best_buy; 
            max_profit = max(today_profit, max_profit); 
        }

        best_buy = min(best_buy, prices[i]);
    }

    max_profit
}

pub fn brute_force(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;

    for i in 1..prices.len() {
        for j in 0..i {
            let today_profit = prices[i] - prices[j];
            max_profit = max(today_profit, max_profit);
        }
    }
    max_profit
}