fn hamming_counter(number: u64) -> u64 {
    let mut x = number;
    let mut ones = 0;
    let mut length = 0;
    let mut shrinking = true;
    while shrinking {
        let last_bit = x & 0x01;
        x = x >> 1;
        length += 1;
        if last_bit == 0 && ones > 0 {
            shrinking = false;
        }
        else if last_bit == 1 {
            ones += 1;
        }
    }
    // growing...
    if length > 64 {
        panic!("No more results.");
    }
    x = (x << 1) | 0x01;
    ones -= 1;
    length -= 1;
    while length > 0 {
        if ones == length {
            x = (x << 1) | 0x01;
            ones -= 1;
        }
        else {
            x = x << 1;
        }
        length -= 1;
    }

    x
}


fn main() {
    let mut x = 0b0000000000000000000000000000000000000000000000000000000000000111;
    println!("{:064b}", x);

    loop {
        x = hamming_counter(x);
        println!("{:064b}", x);
    }
}
