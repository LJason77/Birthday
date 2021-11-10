#[test]
fn main() {
    let rounds = 100_000;
    let mut i = 0;
    for _ in 0..rounds {
        let birthdays = birthday::group_birthdays(23);
        if birthday::is_duplicate(birthdays) {
            i += 1;
        }
    }
    println!("{:?}%", (i as f32 / rounds as f32) * 100.0);
}
