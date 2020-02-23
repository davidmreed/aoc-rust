fn nth_digit(num: u32, digit: u32) -> u32 {
    // Counting digits UP from the right.
    // Because we know the length of all of our
    // numbers, we can get away with this.
    let plus_one: u32 = 10_u32.pow(digit + 1);
    let this_one: u32 = 10_u32.pow(digit);
    ((num % plus_one ) - (num % this_one)) / this_one
}

fn value_slices(vec: &[u32]) -> Vec<&[u32]> {
    let mut ret = Vec::new();
    let mut start_index = 0;
    let mut val: u32 = vec[0];

    for index in 1..vec.len() {
        if vec[index] != val {
            ret.push(&vec[start_index..index]);
            val = vec[index];
            start_index = index;
        }
    }
    ret.push(&vec[start_index..vec.len()]);

    ret
}

fn does_count(pw: u32, f: fn(&&[u32]) -> bool) -> bool {
    let digits: Vec<u32> = (0..6).map(
        |digit| nth_digit(pw, 5 - digit)
    ).collect();

    (1..6).map(
        |index| digits[index] >= digits[index - 1]
    ).all(|item| item) 
    && value_slices(&digits).iter().any(f)
}

fn main() {
    println!(
        "{}",
        (137683..596253).filter(
            |x| does_count(*x, |&slice| slice.len() >= 2)
        ).collect::<Vec<u32>>().len()
    );

    println!(
        "{}",
        (137683..596253).filter(
            |x| does_count(*x, |&slice| slice.len() == 2)
        ).collect::<Vec<u32>>().len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_digit() {
        assert_eq!(6, nth_digit(123456, 0));
        assert_eq!(4, nth_digit(654321, 3));
        assert_eq!(8, nth_digit(892109, 5));
    }

    #[test]
    fn test_value_slices() {
        let test_data = vec![1, 2, 2, 2, 4, 4, 3];

        assert_eq!(
            vec![&test_data[0..1], &test_data[1..4], &test_data[4..6], &test_data[6..]],
            value_slices(&test_data)
        );
        let test_data = vec![1, 2, 3, 4];

        assert_eq!(
            vec![&test_data[0..1], &test_data[1..2], &test_data[2..3], &test_data[3..]],
            value_slices(&test_data)
        );
    }
}
