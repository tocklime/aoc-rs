use super::*;
use std::str::FromStr;

pub fn t(input: &str, out_ix: isize) -> isize {
    Computer::from_str(input).unwrap().run().abs_load(out_ix)
}

#[test]
pub fn day2_tests() {
    assert_eq!(t("1,0,0,0,99", 0), 2);
    assert_eq!(t("2,3,0,3,99", 3), 6);
    assert_eq!(t("2,4,4,5,99,0", 5), 9801);
    assert_eq!(t("1,1,1,4,99,5,6,0,99", 0), 30);
}

pub fn t2(input: &str, i_val: isize) -> isize {
    let mut c = Computer::from_str(input).unwrap();
    c.with_input(i_val).run().get_last_output()
}
#[test]
pub fn cmp_tests() {
    let eq8p = "3,9,8,9,10,9,4,9,99,-1,8";
    assert_eq!(t2(eq8p, 7), 0);
    assert_eq!(t2(eq8p, 8), 1);
    assert_eq!(t2(eq8p, 9), 0);
    let lt8p = "3,9,7,9,10,9,4,9,99,-1,8";
    assert_eq!(t2(lt8p, 7), 1);
    assert_eq!(t2(lt8p, 8), 0);
    assert_eq!(t2(lt8p, 9), 0);
    let eq8i = "3,3,1108,-1,8,3,4,3,99";
    assert_eq!(t2(eq8i, 7), 0);
    assert_eq!(t2(eq8i, 8), 1);
    assert_eq!(t2(eq8i, 9), 0);
    let lt8i = "3,3,1107,-1,8,3,4,3,99";
    assert_eq!(t2(lt8i, 7), 1);
    assert_eq!(t2(lt8i, 8), 0);
    assert_eq!(t2(lt8i, 9), 0);
}

#[test]
pub fn jmp_tests() {
    let p = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let i = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";

    assert_eq!(t2(p, 0), 0);
    assert_eq!(t2(p, 1), 1);
    assert_eq!(t2(p, 2), 1);
    assert_eq!(t2(i, 0), 0);
    assert_eq!(t2(i, 1), 1);
    assert_eq!(t2(i, 2), 1);
}

#[test]
pub fn d5_test() {
    let t= "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(t2(t, 7), 999);
    assert_eq!(t2(t, 8), 1000);
    assert_eq!(t2(t, 9), 1001);
}
