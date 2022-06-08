use utils::assembunny::Computer;


fn p1a(input: &str) -> i64 {
    let c = Computer::parse(input);
    for x in 0.. {
        let mut c2 = c.clone();
        c2.set_reg('a', x);
        for y in 0..50 {
            while c2.output.is_empty() && c2.running() {
                if c2.instruction_pointer == 0 {
                    c2.set_reg('d', c2.get_reg('a') + 365 * 7);
                    c2.set_reg('c', 0);
                    c2.set_reg('b', 0);
                    c2.instruction_pointer = 9;
                } else if c2.instruction_pointer == 9 {
                } else {
                    c2.step();
                }
            }
            let next = c2.output.get(0).cloned();
            c2.output.clear();
            if next != Some(y % 2) {
                break;
            } else if y == 50 {
                return x;
            }
        }
    }
    0
}
