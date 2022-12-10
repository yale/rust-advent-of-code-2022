use day_10::CPU;

fn main() {
    let contents = include_str!("input.txt").trim();
    let mut cpu = CPU::init();
    cpu.push_instr(contents);

    let strength: isize = cpu
        .filter(|s| (s.cycle + 20) % 40 == 0)
        .map(|s| s.signal_strength())
        .sum();

    println!("{}", strength);
}
