#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal, clippy::must_use_candidate)]

use rand::Rng;

/// 随机生成生日
fn create_birthday() -> String {
    let mut rng = rand::thread_rng();
    let month: u8 = rng.gen_range(1..=12);
    let day: u8 = rng.gen_range(1..=31);
    format!("{:0>2}.{:0>2}", month, day)
}

/// 生成 n 个人的生日
pub fn group_birthdays(n: usize) -> Vec<String> {
    let mut birthdays: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        birthdays.push(create_birthday());
    }
    birthdays
}

/// 判断是否有重复元素
pub fn is_duplicate(mut birthdays: Vec<String>) -> bool {
    birthdays.sort();
    for i in 1..birthdays.len() {
        if birthdays[i] == birthdays[i - 1] {
            // println!("{:?}\n", birthdays);
            return true;
        }
    }
    false
}
