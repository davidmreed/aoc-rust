fn does_count(pw: i32) -> i32 {
    let str_pw = pw.to_string().into_bytes();
    let mut has_double = false;
    let mut last_elem: u8 = 0;

    for b in str_pw {
        if b == last_elem {
            has_double = true;
        }

        if b < last_elem {
            return 0;
        }
        last_elem = b
    }

    if has_double {
        1
    } else {
        0
    }
}

fn main() {
    println!(
        "{}",
        (137683..596253).fold(
            0,
            |acc, x| acc + does_count(x)
        )
    );
}
