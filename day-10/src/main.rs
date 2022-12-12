use day_10::{print_crt, to_crt_output_buffer, CPU};

fn main() {
    let contents = include_str!("input.txt").trim();
    let mut cpu = CPU::init();
    cpu.push_instr(contents);

    let strength: isize = cpu
        .filter(|s| (s.cycle + 20) % 40 == 0)
        .map(|s| s.signal_strength())
        .sum();

    println!("{}", strength);

    // Part 2
    let mut cpu_2 = CPU::init();
    cpu_2.push_instr(contents);
    print_crt(&to_crt_output_buffer(cpu_2));
}
