mod hammingcounter;

use hammingcounter::HammingCounter;

fn main() {
    for i in HammingCounter::new(0xff) {
        println!("{:064b}", i);

        // This would be a pain to generate all of the numbers first then loop through all of them,
        // so here's proof that it's lazy-evaluated:
        if i == 0x000ff000 {
            break;
        }
    }
}
