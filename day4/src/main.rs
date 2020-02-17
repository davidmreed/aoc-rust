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

fn does_count_part_2(pw: i32) -> i32 {
    let str_pw = pw.to_string().into_bytes();
    let mut has_double = false;
    let mut last_elem: u8 = 0;
    let mut elem_count = 0;

    for b in str_pw {
        if b == last_elem {
            elem_count += 1;
        } else {
            if elem_count == 2 {
                has_double = true;
            }
            elem_count = 1;
        }

        if b < last_elem {
            return 0;
        }
        last_elem = b
    }

    if elem_count == 2 {
        has_double = true;
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

    println!(
        "{}",
        (137683..596253).fold(
            0,
            |acc, x| acc + does_count_part_2(x)
        )
    );
}
