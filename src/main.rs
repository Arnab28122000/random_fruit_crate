use rand_fruits::generate_fruits;

fn main() {
    let arg = 5;
    let answer = generate_fruits(arg);
    assert_eq!(5, answer.len());
    println!("{:?}", answer);
}
