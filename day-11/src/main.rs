use day_11::MonkeyInTheMiddle;

fn main() {
    let input = include_str!("input.txt");

    let mut game = MonkeyInTheMiddle::init(input, false);
    for _ in 0..20 { game.round() };
    println!("{}", game.monkey_business());

    let mut game_pt_2 = MonkeyInTheMiddle::init(input, true);
    for _ in 0..10000 { game_pt_2.round() };
    println!("{}", game_pt_2.monkey_business());
}
