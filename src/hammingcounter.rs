pub struct HammingCounter {
    pub curr: u64,
    done: bool
}

impl HammingCounter {
    pub fn new(value: u64) -> HammingCounter {
        HammingCounter{
            curr: value,
            done: false
        }
    }
}

impl Iterator for HammingCounter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.done {
            return None;
        }
        let curr = self.curr;
        let mut next = self.curr;
        let mut ones = 0;
        let mut length = 0;

        // "shrink" the number
        loop {
            let last_bit = next & 0x01;
            next = next >> 1;
            length += 1;
            if last_bit == 0 && ones > 0 {
                break;
            }
            else if last_bit == 1 {
                ones += 1;
            }
        }

        // length will tell us if we're at the end of the sequence.
        // if length is greater than the size of our number, we can't produce any more numbers.
        if length > 64 {
            self.done = true;
            return Some(curr);
        }

        // Replace the last 0 with a 1
        next = (next << 1) | 0x01;
        ones -= 1;
        length -= 1;

        // "reset" the right side of our number with the lowest possible value that has the
        // number of ones that we need
        while length > 0 {
            if ones == length {
                next = (next << 1) | 0x01;
                ones -= 1;
            }
            else {
                next = next << 1;
            }
            length -= 1;
        }
        self.curr = next;
        Some(curr)
    }
}
