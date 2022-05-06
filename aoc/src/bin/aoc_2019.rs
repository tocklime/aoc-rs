use seq_macro::seq;

seq!(D in 01..=25 {
    mod solutions::y2019::y19_d~D;
});

pub fn main() {
    solutions::y2019::y19_d01::main();
}
