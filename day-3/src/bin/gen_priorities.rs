const LOWERCASE_A: u8 = 97;
const UPPERCASE_A: u8 = 65;

fn main() {
    (LOWERCASE_A..=(LOWERCASE_A + 25)).for_each(|n| {
        let c: char = n.try_into().expect("could not convert into char");
        println!("    '{}' => {},", c, n - LOWERCASE_A + 1);
    });

    (UPPERCASE_A..=(UPPERCASE_A + 25)).for_each(|n| {
        let c: char = n.try_into().expect("could not convert into char");
        println!("    '{}' => {},", c, n - UPPERCASE_A + 27);
    });
}
