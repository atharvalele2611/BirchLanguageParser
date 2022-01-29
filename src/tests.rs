use super::{Command::*, Prog, ProgExecError, ProgParseError};
use std::fs;

#[test]
fn test_parse_00_example_00() {
    let src = fs::read_to_string("./assets/00_example_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_00_res())
}
fn test_parse_00_example_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_00())
}
fn prog_00_example_00() -> Prog {
    Prog(vec![Num(1), Num(2), Num(3)])
}
#[test]
fn test_exec_00_example_00() {
    assert_eq!(
        prog_00_example_00().exec(false),
        test_exec_00_example_00_res()
    )
}
fn test_exec_00_example_00_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_00_example_01() {
    let src = fs::read_to_string("./assets/00_example_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_01_res())
}
fn test_parse_00_example_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_01())
}
fn prog_00_example_01() -> Prog {
    Prog(vec![Add, Num(2), Num(2)])
}
#[test]
fn test_exec_00_example_01() {
    assert_eq!(
        prog_00_example_01().exec(false),
        test_exec_00_example_01_res()
    )
}
fn test_exec_00_example_01_res() -> Result<i64, ProgExecError> {
    Ok(4)
}

#[test]
fn test_parse_00_example_02() {
    let src = fs::read_to_string("./assets/00_example_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_02_res())
}
fn test_parse_00_example_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_02())
}
fn prog_00_example_02() -> Prog {
    Prog(vec![Exec, Cmds(vec![Mul, Num(3)]), Num(2)])
}

#[test]
fn test_exec_00_example_02() {
    assert_eq!(
        prog_00_example_02().exec(false),
        test_exec_00_example_02_res()
    )
}
fn test_exec_00_example_02_res() -> Result<i64, ProgExecError> {
    Ok(6)
}

#[test]
fn test_parse_00_example_03() {
    let src = fs::read_to_string("./assets/00_example_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_03_res())
}
fn test_parse_00_example_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_03())
}
fn prog_00_example_03() -> Prog {
    Prog(vec![Exec, Swap, Num(2), Cmds(vec![Mul, Num(3)])])
}
#[test]
fn test_exec_00_example_03() {
    assert_eq!(
        prog_00_example_03().exec(false),
        test_exec_00_example_03_res()
    )
}
fn test_exec_00_example_03_res() -> Result<i64, ProgExecError> {
    Ok(6)
}

#[test]
fn test_parse_00_example_05() {
    let src = fs::read_to_string("./assets/00_example_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_05_res())
}
fn test_parse_00_example_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_05())
}
fn prog_00_example_05() -> Prog {
    Prog(vec![Sub, Num(5), Num(10)])
}
#[test]
fn test_exec_00_example_05() {
    assert_eq!(
        prog_00_example_05().exec(false),
        test_exec_00_example_05_res()
    )
}
fn test_exec_00_example_05_res() -> Result<i64, ProgExecError> {
    Ok(-5)
}

#[test]
fn test_parse_00_example_06() {
    let src = fs::read_to_string("./assets/00_example_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_06_res())
}
fn test_parse_00_example_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_06())
}
fn prog_00_example_06() -> Prog {
    Prog(vec![Div, Num(16), Num(8), Num(4), Num(2)])
}
#[test]
fn test_exec_00_example_06() {
    assert_eq!(
        prog_00_example_06().exec(false),
        test_exec_00_example_06_res()
    )
}
fn test_exec_00_example_06_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_00_example_07() {
    let src = fs::read_to_string("./assets/00_example_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_07_res())
}
fn test_parse_00_example_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_07())
}
fn prog_00_example_07() -> Prog {
    Prog(vec![
        Mul,
        Add,
        Mul,
        Add,
        Dup,
        Num(-1),
        Dup,
        Num(1),
        Num(1),
        Num(2),
        Num(3),
    ])
}
#[test]
fn test_exec_00_example_07() {
    assert_eq!(
        prog_00_example_07().exec(false),
        test_exec_00_example_07_res()
    )
}
fn test_exec_00_example_07_res() -> Result<i64, ProgExecError> {
    Ok(21)
}

#[test]
fn test_parse_00_example_08() {
    let src = fs::read_to_string("./assets/00_example_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_08_res())
}
fn test_parse_00_example_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_08())
}
fn prog_00_example_08() -> Prog {
    Prog(vec![Dup, Num(3), Num(20), Num(1)])
}
#[test]
fn test_exec_00_example_08() {
    assert_eq!(
        prog_00_example_08().exec(false),
        test_exec_00_example_08_res()
    )
}
fn test_exec_00_example_08_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_09() {
    let src = fs::read_to_string("./assets/00_example_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_09_res())
}
fn test_parse_00_example_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_09())
}
fn prog_00_example_09() -> Prog {
    Prog(vec![Exec, Swap, Num(3), Cmds(vec![Mul, Num(4)])])
}
#[test]
fn test_exec_00_example_09() {
    assert_eq!(
        prog_00_example_09().exec(false),
        test_exec_00_example_09_res()
    )
}
fn test_exec_00_example_09_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_00_example_10() {
    let src = fs::read_to_string("./assets/00_example_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_10_res())
}
fn test_parse_00_example_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_10())
}
fn prog_00_example_10() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![]),
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Gt,
        Num(0),
        Dup,
        Num(0),
        Num(-7),
    ])
}
#[test]
fn test_exec_00_example_10() {
    assert_eq!(
        prog_00_example_10().exec(false),
        test_exec_00_example_10_res()
    )
}
fn test_exec_00_example_10_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_00_example_12() {
    let src = fs::read_to_string("./assets/00_example_12.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_12_res())
}
fn test_parse_00_example_12_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_12())
}
fn prog_00_example_12() -> Prog {
    Prog(vec![Exec, Swap, Num(3), Cmds(vec![Mul, Num(4)])])
}
#[test]
fn test_exec_00_example_12() {
    assert_eq!(
        prog_00_example_12().exec(false),
        test_exec_00_example_12_res()
    )
}
fn test_exec_00_example_12_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_00_example_13() {
    let src = fs::read_to_string("./assets/00_example_13.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_13_res())
}
fn test_parse_00_example_13_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_00_example_14() {
    let src = fs::read_to_string("./assets/00_example_14.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_14_res())
}
fn test_parse_00_example_14_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_00_example_15() {
    let src = fs::read_to_string("./assets/00_example_15.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_15_res())
}
fn test_parse_00_example_15_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_15())
}
fn prog_00_example_15() -> Prog {
    Prog(vec![Add, Num(2)])
}
#[test]
fn test_exec_00_example_15() {
    assert_eq!(
        prog_00_example_15().exec(false),
        test_exec_00_example_15_res()
    )
}
fn test_exec_00_example_15_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_16() {
    let src = fs::read_to_string("./assets/00_example_16.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_16_res())
}
fn test_parse_00_example_16_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_01_num_cmd_00() {
    let src = fs::read_to_string("./assets/01_num_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_00_res())
}
fn test_parse_01_num_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_00())
}
fn prog_01_num_cmd_00() -> Prog {
    Prog(vec![Num(42)])
}
#[test]
fn test_exec_01_num_cmd_00() {
    assert_eq!(
        prog_01_num_cmd_00().exec(false),
        test_exec_01_num_cmd_00_res()
    )
}
fn test_exec_01_num_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(42)
}

#[test]
fn test_parse_01_num_cmd_01() {
    let src = fs::read_to_string("./assets/01_num_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_01_res())
}
fn test_parse_01_num_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_01())
}
fn prog_01_num_cmd_01() -> Prog {
    Prog(vec![Num(-42)])
}
#[test]
fn test_exec_01_num_cmd_01() {
    assert_eq!(
        prog_01_num_cmd_01().exec(false),
        test_exec_01_num_cmd_01_res()
    )
}
fn test_exec_01_num_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(-42)
}

#[test]
fn test_parse_01_num_cmd_02() {
    let src = fs::read_to_string("./assets/01_num_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_02_res())
}
fn test_parse_01_num_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_02())
}
fn prog_01_num_cmd_02() -> Prog {
    Prog(vec![Num(1), Num(2), Num(3), Num(4), Num(5), Num(42)])
}
#[test]
fn test_exec_01_num_cmd_02() {
    assert_eq!(
        prog_01_num_cmd_02().exec(false),
        test_exec_01_num_cmd_02_res()
    )
}
fn test_exec_01_num_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_01_num_cmd_03() {
    let src = fs::read_to_string("./assets/01_num_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_03_res())
}
fn test_parse_01_num_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_03())
}
fn prog_01_num_cmd_03() -> Prog {
    Prog(vec![Num(-1), Num(-2), Num(-3), Num(-4), Num(-5), Num(-42)])
}
#[test]
fn test_exec_01_num_cmd_03() {
    assert_eq!(
        prog_01_num_cmd_03().exec(false),
        test_exec_01_num_cmd_03_res()
    )
}
fn test_exec_01_num_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_01_num_cmd_04() {
    let src = fs::read_to_string("./assets/01_num_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_04_res())
}
fn test_parse_01_num_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_04())
}
fn prog_01_num_cmd_04() -> Prog {
    Prog(vec![Num(2147483647)])
}
#[test]
fn test_exec_01_num_cmd_04() {
    assert_eq!(
        prog_01_num_cmd_04().exec(false),
        test_exec_01_num_cmd_04_res()
    )
}
fn test_exec_01_num_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(2147483647)
}

#[test]
fn test_parse_01_num_cmd_05() {
    let src = fs::read_to_string("./assets/01_num_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_05_res())
}
fn test_parse_01_num_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_05())
}
fn prog_01_num_cmd_05() -> Prog {
    Prog(vec![Num(2147483648)])
}
#[test]
fn test_exec_01_num_cmd_05() {
    assert_eq!(
        prog_01_num_cmd_05().exec(false),
        test_exec_01_num_cmd_05_res()
    )
}
fn test_exec_01_num_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(2147483648)
}

#[test]
fn test_parse_01_num_cmd_06() {
    let src = fs::read_to_string("./assets/01_num_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_06_res())
}
fn test_parse_01_num_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_06())
}
fn prog_01_num_cmd_06() -> Prog {
    Prog(vec![Num(2147483649), Num(2147483648), Num(2147483647)])
}
#[test]
fn test_exec_01_num_cmd_06() {
    assert_eq!(
        prog_01_num_cmd_06().exec(false),
        test_exec_01_num_cmd_06_res()
    )
}
fn test_exec_01_num_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(2147483649)
}

#[test]
fn test_parse_01_num_cmd_07() {
    let src = fs::read_to_string("./assets/01_num_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_07_res())
}
fn test_parse_01_num_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_07())
}
fn prog_01_num_cmd_07() -> Prog {
    Prog(vec![Num(9223372036854775807)])
}
#[test]
fn test_exec_01_num_cmd_07() {
    assert_eq!(
        prog_01_num_cmd_07().exec(false),
        test_exec_01_num_cmd_07_res()
    )
}
fn test_exec_01_num_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(9223372036854775807)
}

#[test]
fn test_parse_01_num_cmd_10() {
    let src = fs::read_to_string("./assets/01_num_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_10_res())
}
fn test_parse_01_num_cmd_10_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_02_add_cmd_001() {
    let src = fs::read_to_string("./assets/02_add_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_001_res())
}
fn test_parse_02_add_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_001())
}
fn prog_02_add_cmd_001() -> Prog {
    Prog(vec![Add, Num(-1), Num(2147483647)])
}
#[test]
fn test_exec_02_add_cmd_001() {
    assert_eq!(
        prog_02_add_cmd_001().exec(false),
        test_exec_02_add_cmd_001_res()
    )
}
fn test_exec_02_add_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(2147483646)
}

#[test]
fn test_parse_02_add_cmd_002() {
    let src = fs::read_to_string("./assets/02_add_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_002_res())
}
fn test_parse_02_add_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_002())
}
fn prog_02_add_cmd_002() -> Prog {
    Prog(vec![Add, Num(-2147483646), Num(1)])
}
#[test]
fn test_exec_02_add_cmd_002() {
    assert_eq!(
        prog_02_add_cmd_002().exec(false),
        test_exec_02_add_cmd_002_res()
    )
}
fn test_exec_02_add_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(-2147483645)
}

#[test]
fn test_parse_02_add_cmd_003() {
    let src = fs::read_to_string("./assets/02_add_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_003_res())
}
fn test_parse_02_add_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_003())
}
fn prog_02_add_cmd_003() -> Prog {
    Prog(vec![Add, Num(-2), Num(2)])
}
#[test]
fn test_exec_02_add_cmd_003() {
    assert_eq!(
        prog_02_add_cmd_003().exec(false),
        test_exec_02_add_cmd_003_res()
    )
}
fn test_exec_02_add_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_02_add_cmd_004() {
    let src = fs::read_to_string("./assets/02_add_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_004_res())
}
fn test_parse_02_add_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_004())
}
fn prog_02_add_cmd_004() -> Prog {
    Prog(vec![Add, Num(-10), Num(-2147483648)])
}
#[test]
fn test_exec_02_add_cmd_004() {
    assert_eq!(
        prog_02_add_cmd_004().exec(false),
        test_exec_02_add_cmd_004_res()
    )
}
fn test_exec_02_add_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(-2147483658)
}

#[test]
fn test_parse_02_add_cmd_005() {
    let src = fs::read_to_string("./assets/02_add_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_005_res())
}
fn test_parse_02_add_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_005())
}
fn prog_02_add_cmd_005() -> Prog {
    Prog(vec![Add, Num(2147483647), Num(5)])
}
#[test]
fn test_exec_02_add_cmd_005() {
    assert_eq!(
        prog_02_add_cmd_005().exec(false),
        test_exec_02_add_cmd_005_res()
    )
}
fn test_exec_02_add_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(2147483652)
}

#[test]
fn test_parse_02_add_cmd_007() {
    let src = fs::read_to_string("./assets/02_add_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_007_res())
}
fn test_parse_02_add_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_007())
}
fn prog_02_add_cmd_007() -> Prog {
    Prog(vec![Add, Num(-3), Num(2147483646)])
}
#[test]
fn test_exec_02_add_cmd_007() {
    assert_eq!(
        prog_02_add_cmd_007().exec(false),
        test_exec_02_add_cmd_007_res()
    )
}
fn test_exec_02_add_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(2147483643)
}

#[test]
fn test_parse_02_add_cmd_008() {
    let src = fs::read_to_string("./assets/02_add_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_008_res())
}
fn test_parse_02_add_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_008())
}
fn prog_02_add_cmd_008() -> Prog {
    Prog(vec![Add, Num(-2147483648), Num(-2)])
}
#[test]
fn test_exec_02_add_cmd_008() {
    assert_eq!(
        prog_02_add_cmd_008().exec(false),
        test_exec_02_add_cmd_008_res()
    )
}
fn test_exec_02_add_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(-2147483650)
}

#[test]
fn test_parse_02_add_cmd_009() {
    let src = fs::read_to_string("./assets/02_add_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_009_res())
}
fn test_parse_02_add_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_009())
}
fn prog_02_add_cmd_009() -> Prog {
    Prog(vec![Add, Num(1), Num(-5)])
}
#[test]
fn test_exec_02_add_cmd_009() {
    assert_eq!(
        prog_02_add_cmd_009().exec(false),
        test_exec_02_add_cmd_009_res()
    )
}
fn test_exec_02_add_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_02_add_cmd_011() {
    let src = fs::read_to_string("./assets/02_add_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_011_res())
}
fn test_parse_02_add_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_011())
}
fn prog_02_add_cmd_011() -> Prog {
    Prog(vec![Add, Num(-2147483647), Num(2147483648)])
}
#[test]
fn test_exec_02_add_cmd_011() {
    assert_eq!(
        prog_02_add_cmd_011().exec(false),
        test_exec_02_add_cmd_011_res()
    )
}
fn test_exec_02_add_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_02_add_cmd_012() {
    let src = fs::read_to_string("./assets/02_add_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_012_res())
}
fn test_parse_02_add_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_012())
}
fn prog_02_add_cmd_012() -> Prog {
    Prog(vec![Add, Num(2147483646), Num(-1)])
}
#[test]
fn test_exec_02_add_cmd_012() {
    assert_eq!(
        prog_02_add_cmd_012().exec(false),
        test_exec_02_add_cmd_012_res()
    )
}
fn test_exec_02_add_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(2147483645)
}

#[test]
fn test_parse_02_add_cmd_013() {
    let src = fs::read_to_string("./assets/02_add_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_013_res())
}
fn test_parse_02_add_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_013())
}
fn prog_02_add_cmd_013() -> Prog {
    Prog(vec![Add, Num(10), Num(3)])
}
#[test]
fn test_exec_02_add_cmd_013() {
    assert_eq!(
        prog_02_add_cmd_013().exec(false),
        test_exec_02_add_cmd_013_res()
    )
}
fn test_exec_02_add_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(13)
}

#[test]
fn test_parse_02_add_cmd_014() {
    let src = fs::read_to_string("./assets/02_add_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_014_res())
}
fn test_parse_02_add_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_014())
}
fn prog_02_add_cmd_014() -> Prog {
    Prog(vec![Add, Num(2147483648), Num(-3)])
}
#[test]
fn test_exec_02_add_cmd_014() {
    assert_eq!(
        prog_02_add_cmd_014().exec(false),
        test_exec_02_add_cmd_014_res()
    )
}
fn test_exec_02_add_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(2147483645)
}

#[test]
fn test_parse_02_add_cmd_015() {
    let src = fs::read_to_string("./assets/02_add_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_015_res())
}
fn test_parse_02_add_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_015())
}
fn prog_02_add_cmd_015() -> Prog {
    Prog(vec![Add, Num(3), Num(-2147483647)])
}
#[test]
fn test_exec_02_add_cmd_015() {
    assert_eq!(
        prog_02_add_cmd_015().exec(false),
        test_exec_02_add_cmd_015_res()
    )
}
fn test_exec_02_add_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(-2147483644)
}

#[test]
fn test_parse_02_add_cmd_016() {
    let src = fs::read_to_string("./assets/02_add_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_016_res())
}
fn test_parse_02_add_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_016())
}
fn prog_02_add_cmd_016() -> Prog {
    Prog(vec![Add, Num(2), Num(10)])
}
#[test]
fn test_exec_02_add_cmd_016() {
    assert_eq!(
        prog_02_add_cmd_016().exec(false),
        test_exec_02_add_cmd_016_res()
    )
}
fn test_exec_02_add_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_02_add_cmd_018() {
    let src = fs::read_to_string("./assets/02_add_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_018_res())
}
fn test_parse_02_add_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_018())
}
fn prog_02_add_cmd_018() -> Prog {
    Prog(vec![Add, Num(0), Num(0)])
}
#[test]
fn test_exec_02_add_cmd_018() {
    assert_eq!(
        prog_02_add_cmd_018().exec(false),
        test_exec_02_add_cmd_018_res()
    )
}
fn test_exec_02_add_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_02_add_cmd_019() {
    let src = fs::read_to_string("./assets/02_add_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_019_res())
}
fn test_parse_02_add_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_019())
}
fn prog_02_add_cmd_019() -> Prog {
    Prog(vec![Add, Num(0)])
}
#[test]
fn test_exec_02_add_cmd_019() {
    assert_eq!(
        prog_02_add_cmd_019().exec(false),
        test_exec_02_add_cmd_019_res()
    )
}
fn test_exec_02_add_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_020() {
    let src = fs::read_to_string("./assets/02_add_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_020_res())
}
fn test_parse_02_add_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_020())
}
fn prog_02_add_cmd_020() -> Prog {
    Prog(vec![Add])
}
#[test]
fn test_exec_02_add_cmd_020() {
    assert_eq!(
        prog_02_add_cmd_020().exec(false),
        test_exec_02_add_cmd_020_res()
    )
}
fn test_exec_02_add_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_03_sub_cmd_000() {
    let src = fs::read_to_string("./assets/03_sub_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_000_res())
}
fn test_parse_03_sub_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_000())
}
fn prog_03_sub_cmd_000() -> Prog {
    Prog(vec![Sub, Num(2147483647), Num(-2147483646)])
}
#[test]
fn test_exec_03_sub_cmd_000() {
    assert_eq!(
        prog_03_sub_cmd_000().exec(false),
        test_exec_03_sub_cmd_000_res()
    )
}
fn test_exec_03_sub_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(4294967293)
}

#[test]
fn test_parse_03_sub_cmd_002() {
    let src = fs::read_to_string("./assets/03_sub_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_002_res())
}
fn test_parse_03_sub_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_002())
}
fn prog_03_sub_cmd_002() -> Prog {
    Prog(vec![Sub, Num(-2147483647), Num(-2147483647)])
}
#[test]
fn test_exec_03_sub_cmd_002() {
    assert_eq!(
        prog_03_sub_cmd_002().exec(false),
        test_exec_03_sub_cmd_002_res()
    )
}
fn test_exec_03_sub_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_03_sub_cmd_003() {
    let src = fs::read_to_string("./assets/03_sub_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_003_res())
}
fn test_parse_03_sub_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_003())
}
fn prog_03_sub_cmd_003() -> Prog {
    Prog(vec![Sub, Num(-2147483646), Num(-10)])
}
#[test]
fn test_exec_03_sub_cmd_003() {
    assert_eq!(
        prog_03_sub_cmd_003().exec(false),
        test_exec_03_sub_cmd_003_res()
    )
}
fn test_exec_03_sub_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(-2147483636)
}

#[test]
fn test_parse_03_sub_cmd_004() {
    let src = fs::read_to_string("./assets/03_sub_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_004_res())
}
fn test_parse_03_sub_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_004())
}
fn prog_03_sub_cmd_004() -> Prog {
    Prog(vec![Sub, Num(-2), Num(-3)])
}
#[test]
fn test_exec_03_sub_cmd_004() {
    assert_eq!(
        prog_03_sub_cmd_004().exec(false),
        test_exec_03_sub_cmd_004_res()
    )
}
fn test_exec_03_sub_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_03_sub_cmd_006() {
    let src = fs::read_to_string("./assets/03_sub_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_006_res())
}
fn test_parse_03_sub_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_006())
}
fn prog_03_sub_cmd_006() -> Prog {
    Prog(vec![Sub, Num(-10), Num(1)])
}
#[test]
fn test_exec_03_sub_cmd_006() {
    assert_eq!(
        prog_03_sub_cmd_006().exec(false),
        test_exec_03_sub_cmd_006_res()
    )
}
fn test_exec_03_sub_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(-11)
}

#[test]
fn test_parse_03_sub_cmd_007() {
    let src = fs::read_to_string("./assets/03_sub_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_007_res())
}
fn test_parse_03_sub_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_007())
}
fn prog_03_sub_cmd_007() -> Prog {
    Prog(vec![Sub, Num(2), Num(5)])
}
#[test]
fn test_exec_03_sub_cmd_007() {
    assert_eq!(
        prog_03_sub_cmd_007().exec(false),
        test_exec_03_sub_cmd_007_res()
    )
}
fn test_exec_03_sub_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_03_sub_cmd_008() {
    let src = fs::read_to_string("./assets/03_sub_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_008_res())
}
fn test_parse_03_sub_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_008())
}
fn prog_03_sub_cmd_008() -> Prog {
    Prog(vec![Sub, Num(5), Num(2147483646)])
}
#[test]
fn test_exec_03_sub_cmd_008() {
    assert_eq!(
        prog_03_sub_cmd_008().exec(false),
        test_exec_03_sub_cmd_008_res()
    )
}
fn test_exec_03_sub_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(-2147483641)
}

#[test]
fn test_parse_03_sub_cmd_009() {
    let src = fs::read_to_string("./assets/03_sub_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_009_res())
}
fn test_parse_03_sub_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_009())
}
fn prog_03_sub_cmd_009() -> Prog {
    Prog(vec![Sub, Num(10), Num(0)])
}
#[test]
fn test_exec_03_sub_cmd_009() {
    assert_eq!(
        prog_03_sub_cmd_009().exec(false),
        test_exec_03_sub_cmd_009_res()
    )
}
fn test_exec_03_sub_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(10)
}

#[test]
fn test_parse_03_sub_cmd_010() {
    let src = fs::read_to_string("./assets/03_sub_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_010_res())
}
fn test_parse_03_sub_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_010())
}
fn prog_03_sub_cmd_010() -> Prog {
    Prog(vec![Sub, Num(-1), Num(2)])
}
#[test]
fn test_exec_03_sub_cmd_010() {
    assert_eq!(
        prog_03_sub_cmd_010().exec(false),
        test_exec_03_sub_cmd_010_res()
    )
}
fn test_exec_03_sub_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_03_sub_cmd_011() {
    let src = fs::read_to_string("./assets/03_sub_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_011_res())
}
fn test_parse_03_sub_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_011())
}
fn prog_03_sub_cmd_011() -> Prog {
    Prog(vec![Sub, Num(2147483648), Num(2147483648)])
}
#[test]
fn test_exec_03_sub_cmd_011() {
    assert_eq!(
        prog_03_sub_cmd_011().exec(false),
        test_exec_03_sub_cmd_011_res()
    )
}
fn test_exec_03_sub_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_03_sub_cmd_013() {
    let src = fs::read_to_string("./assets/03_sub_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_013_res())
}
fn test_parse_03_sub_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_013())
}
fn prog_03_sub_cmd_013() -> Prog {
    Prog(vec![Sub, Num(2147483646), Num(-1)])
}
#[test]
fn test_exec_03_sub_cmd_013() {
    assert_eq!(
        prog_03_sub_cmd_013().exec(false),
        test_exec_03_sub_cmd_013_res()
    )
}
fn test_exec_03_sub_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(2147483647)
}

#[test]
fn test_parse_03_sub_cmd_015() {
    let src = fs::read_to_string("./assets/03_sub_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_015_res())
}
fn test_parse_03_sub_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_015())
}
fn prog_03_sub_cmd_015() -> Prog {
    Prog(vec![Sub, Num(-2147483648), Num(-2)])
}
#[test]
fn test_exec_03_sub_cmd_015() {
    assert_eq!(
        prog_03_sub_cmd_015().exec(false),
        test_exec_03_sub_cmd_015_res()
    )
}
fn test_exec_03_sub_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(-2147483646)
}

#[test]
fn test_parse_03_sub_cmd_016() {
    let src = fs::read_to_string("./assets/03_sub_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_016_res())
}
fn test_parse_03_sub_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_016())
}
fn prog_03_sub_cmd_016() -> Prog {
    Prog(vec![Sub, Num(3), Num(-5)])
}
#[test]
fn test_exec_03_sub_cmd_016() {
    assert_eq!(
        prog_03_sub_cmd_016().exec(false),
        test_exec_03_sub_cmd_016_res()
    )
}
fn test_exec_03_sub_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(8)
}

#[test]
fn test_parse_03_sub_cmd_017() {
    let src = fs::read_to_string("./assets/03_sub_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_017_res())
}
fn test_parse_03_sub_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_017())
}
fn prog_03_sub_cmd_017() -> Prog {
    Prog(vec![Sub, Num(1), Num(-2147483648)])
}
#[test]
fn test_exec_03_sub_cmd_017() {
    assert_eq!(
        prog_03_sub_cmd_017().exec(false),
        test_exec_03_sub_cmd_017_res()
    )
}
fn test_exec_03_sub_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(2147483649)
}

#[test]
fn test_parse_03_sub_cmd_018() {
    let src = fs::read_to_string("./assets/03_sub_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_018_res())
}
fn test_parse_03_sub_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_018())
}
fn prog_03_sub_cmd_018() -> Prog {
    Prog(vec![Sub, Num(-3), Num(10)])
}
#[test]
fn test_exec_03_sub_cmd_018() {
    assert_eq!(
        prog_03_sub_cmd_018().exec(false),
        test_exec_03_sub_cmd_018_res()
    )
}
fn test_exec_03_sub_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(-13)
}

#[test]
fn test_parse_03_sub_cmd_019() {
    let src = fs::read_to_string("./assets/03_sub_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_019_res())
}
fn test_parse_03_sub_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_019())
}
fn prog_03_sub_cmd_019() -> Prog {
    Prog(vec![Sub, Num(10)])
}
#[test]
fn test_exec_03_sub_cmd_019() {
    assert_eq!(
        prog_03_sub_cmd_019().exec(false),
        test_exec_03_sub_cmd_019_res()
    )
}
fn test_exec_03_sub_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_03_sub_cmd_020() {
    let src = fs::read_to_string("./assets/03_sub_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_020_res())
}
fn test_parse_03_sub_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_020())
}
fn prog_03_sub_cmd_020() -> Prog {
    Prog(vec![Sub])
}
#[test]
fn test_exec_03_sub_cmd_020() {
    assert_eq!(
        prog_03_sub_cmd_020().exec(false),
        test_exec_03_sub_cmd_020_res()
    )
}
fn test_exec_03_sub_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_000() {
    let src = fs::read_to_string("./assets/04_mul_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_000_res())
}
fn test_parse_04_mul_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_000())
}
fn prog_04_mul_cmd_000() -> Prog {
    Prog(vec![Mul, Num(-2147483646), Num(-2147483647)])
}
#[test]
fn test_exec_04_mul_cmd_000() {
    assert_eq!(
        prog_04_mul_cmd_000().exec(false),
        test_exec_04_mul_cmd_000_res()
    )
}
fn test_exec_04_mul_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(4611686011984936962)
}

#[test]
fn test_parse_04_mul_cmd_001() {
    let src = fs::read_to_string("./assets/04_mul_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_001_res())
}
fn test_parse_04_mul_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_001())
}
fn prog_04_mul_cmd_001() -> Prog {
    Prog(vec![Mul, Num(-10), Num(-2)])
}
#[test]
fn test_exec_04_mul_cmd_001() {
    assert_eq!(
        prog_04_mul_cmd_001().exec(false),
        test_exec_04_mul_cmd_001_res()
    )
}
fn test_exec_04_mul_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(20)
}

#[test]
fn test_parse_04_mul_cmd_002() {
    let src = fs::read_to_string("./assets/04_mul_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_002_res())
}
fn test_parse_04_mul_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_002())
}
fn prog_04_mul_cmd_002() -> Prog {
    Prog(vec![Mul, Num(1), Num(-2147483648)])
}
#[test]
fn test_exec_04_mul_cmd_002() {
    assert_eq!(
        prog_04_mul_cmd_002().exec(false),
        test_exec_04_mul_cmd_002_res()
    )
}
fn test_exec_04_mul_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(-2147483648)
}

#[test]
fn test_parse_04_mul_cmd_003() {
    let src = fs::read_to_string("./assets/04_mul_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_003_res())
}
fn test_parse_04_mul_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_003())
}
fn prog_04_mul_cmd_003() -> Prog {
    Prog(vec![Mul, Num(-3), Num(2147483646)])
}
#[test]
fn test_exec_04_mul_cmd_003() {
    assert_eq!(
        prog_04_mul_cmd_003().exec(false),
        test_exec_04_mul_cmd_003_res()
    )
}
fn test_exec_04_mul_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(-6442450938)
}

#[test]
fn test_parse_04_mul_cmd_004() {
    let src = fs::read_to_string("./assets/04_mul_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_004_res())
}
fn test_parse_04_mul_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_004())
}
fn prog_04_mul_cmd_004() -> Prog {
    Prog(vec![Mul, Num(-1), Num(10)])
}
#[test]
fn test_exec_04_mul_cmd_004() {
    assert_eq!(
        prog_04_mul_cmd_004().exec(false),
        test_exec_04_mul_cmd_004_res()
    )
}
fn test_exec_04_mul_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(-10)
}

#[test]
fn test_parse_04_mul_cmd_005() {
    let src = fs::read_to_string("./assets/04_mul_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_005_res())
}
fn test_parse_04_mul_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_005())
}
fn prog_04_mul_cmd_005() -> Prog {
    Prog(vec![Mul, Num(0), Num(2147483647)])
}
#[test]
fn test_exec_04_mul_cmd_005() {
    assert_eq!(
        prog_04_mul_cmd_005().exec(false),
        test_exec_04_mul_cmd_005_res()
    )
}
fn test_exec_04_mul_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_04_mul_cmd_006() {
    let src = fs::read_to_string("./assets/04_mul_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_006_res())
}
fn test_parse_04_mul_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_006())
}
fn prog_04_mul_cmd_006() -> Prog {
    Prog(vec![Mul, Num(2), Num(2)])
}
#[test]
fn test_exec_04_mul_cmd_006() {
    assert_eq!(
        prog_04_mul_cmd_006().exec(false),
        test_exec_04_mul_cmd_006_res()
    )
}
fn test_exec_04_mul_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(4)
}

#[test]
fn test_parse_04_mul_cmd_007() {
    let src = fs::read_to_string("./assets/04_mul_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_007_res())
}
fn test_parse_04_mul_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_007())
}
fn prog_04_mul_cmd_007() -> Prog {
    Prog(vec![Mul, Num(2147483647), Num(-2147483646)])
}
#[test]
fn test_exec_04_mul_cmd_007() {
    assert_eq!(
        prog_04_mul_cmd_007().exec(false),
        test_exec_04_mul_cmd_007_res()
    )
}
fn test_exec_04_mul_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(-4611686011984936962)
}

#[test]
fn test_parse_04_mul_cmd_008() {
    let src = fs::read_to_string("./assets/04_mul_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_008_res())
}
fn test_parse_04_mul_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_008())
}
fn prog_04_mul_cmd_008() -> Prog {
    Prog(vec![Mul, Num(2147483646), Num(-10)])
}
#[test]
fn test_exec_04_mul_cmd_008() {
    assert_eq!(
        prog_04_mul_cmd_008().exec(false),
        test_exec_04_mul_cmd_008_res()
    )
}
fn test_exec_04_mul_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(-21474836460)
}

#[test]
fn test_parse_04_mul_cmd_009() {
    let src = fs::read_to_string("./assets/04_mul_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_009_res())
}
fn test_parse_04_mul_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_009())
}
fn prog_04_mul_cmd_009() -> Prog {
    Prog(vec![Mul, Num(10), Num(0)])
}
#[test]
fn test_exec_04_mul_cmd_009() {
    assert_eq!(
        prog_04_mul_cmd_009().exec(false),
        test_exec_04_mul_cmd_009_res()
    )
}
fn test_exec_04_mul_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_04_mul_cmd_010() {
    let src = fs::read_to_string("./assets/04_mul_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_010_res())
}
fn test_parse_04_mul_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_010())
}
fn prog_04_mul_cmd_010() -> Prog {
    Prog(vec![Mul, Num(3), Num(3)])
}
#[test]
fn test_exec_04_mul_cmd_010() {
    assert_eq!(
        prog_04_mul_cmd_010().exec(false),
        test_exec_04_mul_cmd_010_res()
    )
}
fn test_exec_04_mul_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_04_mul_cmd_011() {
    let src = fs::read_to_string("./assets/04_mul_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_011_res())
}
fn test_parse_04_mul_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_011())
}
fn prog_04_mul_cmd_011() -> Prog {
    Prog(vec![Mul, Num(-2147483647), Num(-1)])
}
#[test]
fn test_exec_04_mul_cmd_011() {
    assert_eq!(
        prog_04_mul_cmd_011().exec(false),
        test_exec_04_mul_cmd_011_res()
    )
}
fn test_exec_04_mul_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(2147483647)
}

#[test]
fn test_parse_04_mul_cmd_013() {
    let src = fs::read_to_string("./assets/04_mul_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_013_res())
}
fn test_parse_04_mul_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_013())
}
fn prog_04_mul_cmd_013() -> Prog {
    Prog(vec![Mul, Num(5), Num(-5)])
}
#[test]
fn test_exec_04_mul_cmd_013() {
    assert_eq!(
        prog_04_mul_cmd_013().exec(false),
        test_exec_04_mul_cmd_013_res()
    )
}
fn test_exec_04_mul_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(-25)
}

#[test]
fn test_parse_04_mul_cmd_015() {
    let src = fs::read_to_string("./assets/04_mul_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_015_res())
}
fn test_parse_04_mul_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_015())
}
fn prog_04_mul_cmd_015() -> Prog {
    Prog(vec![Mul, Num(-5), Num(-3)])
}
#[test]
fn test_exec_04_mul_cmd_015() {
    assert_eq!(
        prog_04_mul_cmd_015().exec(false),
        test_exec_04_mul_cmd_015_res()
    )
}
fn test_exec_04_mul_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(15)
}

#[test]
fn test_parse_04_mul_cmd_016() {
    let src = fs::read_to_string("./assets/04_mul_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_016_res())
}
fn test_parse_04_mul_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_016())
}
fn prog_04_mul_cmd_016() -> Prog {
    Prog(vec![Mul, Num(2147483648), Num(1)])
}
#[test]
fn test_exec_04_mul_cmd_016() {
    assert_eq!(
        prog_04_mul_cmd_016().exec(false),
        test_exec_04_mul_cmd_016_res()
    )
}
fn test_exec_04_mul_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(2147483648)
}

#[test]
fn test_parse_04_mul_cmd_019() {
    let src = fs::read_to_string("./assets/04_mul_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_019_res())
}
fn test_parse_04_mul_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_019())
}
fn prog_04_mul_cmd_019() -> Prog {
    Prog(vec![Mul, Num(5)])
}
#[test]
fn test_exec_04_mul_cmd_019() {
    assert_eq!(
        prog_04_mul_cmd_019().exec(false),
        test_exec_04_mul_cmd_019_res()
    )
}
fn test_exec_04_mul_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_020() {
    let src = fs::read_to_string("./assets/04_mul_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_020_res())
}
fn test_parse_04_mul_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_020())
}
fn prog_04_mul_cmd_020() -> Prog {
    Prog(vec![Mul])
}
#[test]
fn test_exec_04_mul_cmd_020() {
    assert_eq!(
        prog_04_mul_cmd_020().exec(false),
        test_exec_04_mul_cmd_020_res()
    )
}
fn test_exec_04_mul_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_001() {
    let src = fs::read_to_string("./assets/05_div_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_001_res())
}
fn test_parse_05_div_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_001())
}
fn prog_05_div_cmd_001() -> Prog {
    Prog(vec![Div, Num(1), Num(2147483646)])
}
#[test]
fn test_exec_05_div_cmd_001() {
    assert_eq!(
        prog_05_div_cmd_001().exec(false),
        test_exec_05_div_cmd_001_res()
    )
}
fn test_exec_05_div_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_002() {
    let src = fs::read_to_string("./assets/05_div_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_002_res())
}
fn test_parse_05_div_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_002())
}
fn prog_05_div_cmd_002() -> Prog {
    Prog(vec![Div, Num(5), Num(2)])
}
#[test]
fn test_exec_05_div_cmd_002() {
    assert_eq!(
        prog_05_div_cmd_002().exec(false),
        test_exec_05_div_cmd_002_res()
    )
}
fn test_exec_05_div_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_05_div_cmd_003() {
    let src = fs::read_to_string("./assets/05_div_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_003_res())
}
fn test_parse_05_div_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_003())
}
fn prog_05_div_cmd_003() -> Prog {
    Prog(vec![Div, Num(3), Num(0)])
}
#[test]
fn test_exec_05_div_cmd_003() {
    assert_eq!(
        prog_05_div_cmd_003().exec(false),
        test_exec_05_div_cmd_003_res()
    )
}
fn test_exec_05_div_cmd_003_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_004() {
    let src = fs::read_to_string("./assets/05_div_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_004_res())
}
fn test_parse_05_div_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_004())
}
fn prog_05_div_cmd_004() -> Prog {
    Prog(vec![Div, Num(10), Num(-2147483648)])
}
#[test]
fn test_exec_05_div_cmd_004() {
    assert_eq!(
        prog_05_div_cmd_004().exec(false),
        test_exec_05_div_cmd_004_res()
    )
}
fn test_exec_05_div_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_005() {
    let src = fs::read_to_string("./assets/05_div_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_005_res())
}
fn test_parse_05_div_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_005())
}
fn prog_05_div_cmd_005() -> Prog {
    Prog(vec![Div, Num(-2147483646), Num(-10)])
}
#[test]
fn test_exec_05_div_cmd_005() {
    assert_eq!(
        prog_05_div_cmd_005().exec(false),
        test_exec_05_div_cmd_005_res()
    )
}
fn test_exec_05_div_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(214748364)
}

#[test]
fn test_parse_05_div_cmd_006() {
    let src = fs::read_to_string("./assets/05_div_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_006_res())
}
fn test_parse_05_div_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_006())
}
fn prog_05_div_cmd_006() -> Prog {
    Prog(vec![Div, Num(-2147483648), Num(3)])
}
#[test]
fn test_exec_05_div_cmd_006() {
    assert_eq!(
        prog_05_div_cmd_006().exec(false),
        test_exec_05_div_cmd_006_res()
    )
}
fn test_exec_05_div_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(-715827882)
}

#[test]
fn test_parse_05_div_cmd_008() {
    let src = fs::read_to_string("./assets/05_div_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_008_res())
}
fn test_parse_05_div_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_008())
}
fn prog_05_div_cmd_008() -> Prog {
    Prog(vec![Div, Num(2), Num(2147483648)])
}
#[test]
fn test_exec_05_div_cmd_008() {
    assert_eq!(
        prog_05_div_cmd_008().exec(false),
        test_exec_05_div_cmd_008_res()
    )
}
fn test_exec_05_div_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_009() {
    let src = fs::read_to_string("./assets/05_div_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_009_res())
}
fn test_parse_05_div_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_009())
}
fn prog_05_div_cmd_009() -> Prog {
    Prog(vec![Div, Num(2147483648), Num(1)])
}
#[test]
fn test_exec_05_div_cmd_009() {
    assert_eq!(
        prog_05_div_cmd_009().exec(false),
        test_exec_05_div_cmd_009_res()
    )
}
fn test_exec_05_div_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(2147483648)
}

#[test]
fn test_parse_05_div_cmd_011() {
    let src = fs::read_to_string("./assets/05_div_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_011_res())
}
fn test_parse_05_div_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_011())
}
fn prog_05_div_cmd_011() -> Prog {
    Prog(vec![Div, Num(2147483646), Num(-3)])
}
#[test]
fn test_exec_05_div_cmd_011() {
    assert_eq!(
        prog_05_div_cmd_011().exec(false),
        test_exec_05_div_cmd_011_res()
    )
}
fn test_exec_05_div_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(-715827882)
}

#[test]
fn test_parse_05_div_cmd_012() {
    let src = fs::read_to_string("./assets/05_div_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_012_res())
}
fn test_parse_05_div_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_012())
}
fn prog_05_div_cmd_012() -> Prog {
    Prog(vec![Div, Num(-2147483647), Num(2147483647)])
}
#[test]
fn test_exec_05_div_cmd_012() {
    assert_eq!(
        prog_05_div_cmd_012().exec(false),
        test_exec_05_div_cmd_012_res()
    )
}
fn test_exec_05_div_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_05_div_cmd_013() {
    let src = fs::read_to_string("./assets/05_div_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_013_res())
}
fn test_parse_05_div_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_013())
}
fn prog_05_div_cmd_013() -> Prog {
    Prog(vec![Div, Num(-1), Num(-5)])
}
#[test]
fn test_exec_05_div_cmd_013() {
    assert_eq!(
        prog_05_div_cmd_013().exec(false),
        test_exec_05_div_cmd_013_res()
    )
}
fn test_exec_05_div_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_014() {
    let src = fs::read_to_string("./assets/05_div_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_014_res())
}
fn test_parse_05_div_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_014())
}
fn prog_05_div_cmd_014() -> Prog {
    Prog(vec![Div, Num(0), Num(5)])
}
#[test]
fn test_exec_05_div_cmd_014() {
    assert_eq!(
        prog_05_div_cmd_014().exec(false),
        test_exec_05_div_cmd_014_res()
    )
}
fn test_exec_05_div_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_016() {
    let src = fs::read_to_string("./assets/05_div_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_016_res())
}
fn test_parse_05_div_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_016())
}
fn prog_05_div_cmd_016() -> Prog {
    Prog(vec![Div, Num(-3), Num(-1)])
}
#[test]
fn test_exec_05_div_cmd_016() {
    assert_eq!(
        prog_05_div_cmd_016().exec(false),
        test_exec_05_div_cmd_016_res()
    )
}
fn test_exec_05_div_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_05_div_cmd_017() {
    let src = fs::read_to_string("./assets/05_div_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_017_res())
}
fn test_parse_05_div_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_017())
}
fn prog_05_div_cmd_017() -> Prog {
    Prog(vec![Div, Num(-10), Num(-2147483647)])
}
#[test]
fn test_exec_05_div_cmd_017() {
    assert_eq!(
        prog_05_div_cmd_017().exec(false),
        test_exec_05_div_cmd_017_res()
    )
}
fn test_exec_05_div_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_018() {
    let src = fs::read_to_string("./assets/05_div_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_018_res())
}
fn test_parse_05_div_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_018())
}
fn prog_05_div_cmd_018() -> Prog {
    Prog(vec![Div, Num(-2), Num(10)])
}
#[test]
fn test_exec_05_div_cmd_018() {
    assert_eq!(
        prog_05_div_cmd_018().exec(false),
        test_exec_05_div_cmd_018_res()
    )
}
fn test_exec_05_div_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_019() {
    let src = fs::read_to_string("./assets/05_div_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_019_res())
}
fn test_parse_05_div_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_019())
}
fn prog_05_div_cmd_019() -> Prog {
    Prog(vec![Div, Num(10)])
}
#[test]
fn test_exec_05_div_cmd_019() {
    assert_eq!(
        prog_05_div_cmd_019().exec(false),
        test_exec_05_div_cmd_019_res()
    )
}
fn test_exec_05_div_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_020() {
    let src = fs::read_to_string("./assets/05_div_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_020_res())
}
fn test_parse_05_div_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_020())
}
fn prog_05_div_cmd_020() -> Prog {
    Prog(vec![Div])
}
#[test]
fn test_exec_05_div_cmd_020() {
    assert_eq!(
        prog_05_div_cmd_020().exec(false),
        test_exec_05_div_cmd_020_res()
    )
}
fn test_exec_05_div_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_06_rem_cmd_001() {
    let src = fs::read_to_string("./assets/06_rem_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_001_res())
}
fn test_parse_06_rem_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_001())
}
fn prog_06_rem_cmd_001() -> Prog {
    Prog(vec![Rem, Num(-3), Num(-2147483647)])
}
#[test]
fn test_exec_06_rem_cmd_001() {
    assert_eq!(
        prog_06_rem_cmd_001().exec(false),
        test_exec_06_rem_cmd_001_res()
    )
}
fn test_exec_06_rem_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_06_rem_cmd_003() {
    let src = fs::read_to_string("./assets/06_rem_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_003_res())
}
fn test_parse_06_rem_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_003())
}
fn prog_06_rem_cmd_003() -> Prog {
    Prog(vec![Rem, Num(1), Num(0)])
}
#[test]
fn test_exec_06_rem_cmd_003() {
    assert_eq!(
        prog_06_rem_cmd_003().exec(false),
        test_exec_06_rem_cmd_003_res()
    )
}
fn test_exec_06_rem_cmd_003_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_06_rem_cmd_004() {
    let src = fs::read_to_string("./assets/06_rem_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_004_res())
}
fn test_parse_06_rem_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_004())
}
fn prog_06_rem_cmd_004() -> Prog {
    Prog(vec![Rem, Num(-5), Num(-10)])
}
#[test]
fn test_exec_06_rem_cmd_004() {
    assert_eq!(
        prog_06_rem_cmd_004().exec(false),
        test_exec_06_rem_cmd_004_res()
    )
}
fn test_exec_06_rem_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(-5)
}

#[test]
fn test_parse_06_rem_cmd_005() {
    let src = fs::read_to_string("./assets/06_rem_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_005_res())
}
fn test_parse_06_rem_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_005())
}
fn prog_06_rem_cmd_005() -> Prog {
    Prog(vec![Rem, Num(2147483648), Num(2147483647)])
}
#[test]
fn test_exec_06_rem_cmd_005() {
    assert_eq!(
        prog_06_rem_cmd_005().exec(false),
        test_exec_06_rem_cmd_005_res()
    )
}
fn test_exec_06_rem_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_006() {
    let src = fs::read_to_string("./assets/06_rem_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_006_res())
}
fn test_parse_06_rem_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_006())
}
fn prog_06_rem_cmd_006() -> Prog {
    Prog(vec![Rem, Num(2147483647), Num(2147483646)])
}
#[test]
fn test_exec_06_rem_cmd_006() {
    assert_eq!(
        prog_06_rem_cmd_006().exec(false),
        test_exec_06_rem_cmd_006_res()
    )
}
fn test_exec_06_rem_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_007() {
    let src = fs::read_to_string("./assets/06_rem_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_007_res())
}
fn test_parse_06_rem_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_007())
}
fn prog_06_rem_cmd_007() -> Prog {
    Prog(vec![Rem, Num(10), Num(2147483648)])
}
#[test]
fn test_exec_06_rem_cmd_007() {
    assert_eq!(
        prog_06_rem_cmd_007().exec(false),
        test_exec_06_rem_cmd_007_res()
    )
}
fn test_exec_06_rem_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(10)
}

#[test]
fn test_parse_06_rem_cmd_008() {
    let src = fs::read_to_string("./assets/06_rem_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_008_res())
}
fn test_parse_06_rem_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_008())
}
fn prog_06_rem_cmd_008() -> Prog {
    Prog(vec![Rem, Num(2), Num(3)])
}
#[test]
fn test_exec_06_rem_cmd_008() {
    assert_eq!(
        prog_06_rem_cmd_008().exec(false),
        test_exec_06_rem_cmd_008_res()
    )
}
fn test_exec_06_rem_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_06_rem_cmd_009() {
    let src = fs::read_to_string("./assets/06_rem_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_009_res())
}
fn test_parse_06_rem_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_009())
}
fn prog_06_rem_cmd_009() -> Prog {
    Prog(vec![Rem, Num(-1), Num(1)])
}
#[test]
fn test_exec_06_rem_cmd_009() {
    assert_eq!(
        prog_06_rem_cmd_009().exec(false),
        test_exec_06_rem_cmd_009_res()
    )
}
fn test_exec_06_rem_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_06_rem_cmd_010() {
    let src = fs::read_to_string("./assets/06_rem_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_010_res())
}
fn test_parse_06_rem_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_010())
}
fn prog_06_rem_cmd_010() -> Prog {
    Prog(vec![Rem, Num(-10), Num(-2147483646)])
}
#[test]
fn test_exec_06_rem_cmd_010() {
    assert_eq!(
        prog_06_rem_cmd_010().exec(false),
        test_exec_06_rem_cmd_010_res()
    )
}
fn test_exec_06_rem_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(-10)
}

#[test]
fn test_parse_06_rem_cmd_011() {
    let src = fs::read_to_string("./assets/06_rem_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_011_res())
}
fn test_parse_06_rem_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_011())
}
fn prog_06_rem_cmd_011() -> Prog {
    Prog(vec![Rem, Num(5), Num(2)])
}
#[test]
fn test_exec_06_rem_cmd_011() {
    assert_eq!(
        prog_06_rem_cmd_011().exec(false),
        test_exec_06_rem_cmd_011_res()
    )
}
fn test_exec_06_rem_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_012() {
    let src = fs::read_to_string("./assets/06_rem_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_012_res())
}
fn test_parse_06_rem_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_012())
}
fn prog_06_rem_cmd_012() -> Prog {
    Prog(vec![Rem, Num(-2), Num(10)])
}
#[test]
fn test_exec_06_rem_cmd_012() {
    assert_eq!(
        prog_06_rem_cmd_012().exec(false),
        test_exec_06_rem_cmd_012_res()
    )
}
fn test_exec_06_rem_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_06_rem_cmd_014() {
    let src = fs::read_to_string("./assets/06_rem_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_014_res())
}
fn test_parse_06_rem_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_014())
}
fn prog_06_rem_cmd_014() -> Prog {
    Prog(vec![Rem, Num(-2147483648), Num(-5)])
}
#[test]
fn test_exec_06_rem_cmd_014() {
    assert_eq!(
        prog_06_rem_cmd_014().exec(false),
        test_exec_06_rem_cmd_014_res()
    )
}
fn test_exec_06_rem_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_06_rem_cmd_015() {
    let src = fs::read_to_string("./assets/06_rem_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_015_res())
}
fn test_parse_06_rem_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_015())
}
fn prog_06_rem_cmd_015() -> Prog {
    Prog(vec![Rem, Num(-2147483647), Num(5)])
}
#[test]
fn test_exec_06_rem_cmd_015() {
    assert_eq!(
        prog_06_rem_cmd_015().exec(false),
        test_exec_06_rem_cmd_015_res()
    )
}
fn test_exec_06_rem_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_06_rem_cmd_017() {
    let src = fs::read_to_string("./assets/06_rem_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_017_res())
}
fn test_parse_06_rem_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_017())
}
fn prog_06_rem_cmd_017() -> Prog {
    Prog(vec![Rem, Num(-2147483646), Num(-3)])
}
#[test]
fn test_exec_06_rem_cmd_017() {
    assert_eq!(
        prog_06_rem_cmd_017().exec(false),
        test_exec_06_rem_cmd_017_res()
    )
}
fn test_exec_06_rem_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_06_rem_cmd_018() {
    let src = fs::read_to_string("./assets/06_rem_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_018_res())
}
fn test_parse_06_rem_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_018())
}
fn prog_06_rem_cmd_018() -> Prog {
    Prog(vec![Rem, Num(0), Num(-2147483648)])
}
#[test]
fn test_exec_06_rem_cmd_018() {
    assert_eq!(
        prog_06_rem_cmd_018().exec(false),
        test_exec_06_rem_cmd_018_res()
    )
}
fn test_exec_06_rem_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_06_rem_cmd_019() {
    let src = fs::read_to_string("./assets/06_rem_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_019_res())
}
fn test_parse_06_rem_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_019())
}
fn prog_06_rem_cmd_019() -> Prog {
    Prog(vec![Rem, Num(-2147483648)])
}
#[test]
fn test_exec_06_rem_cmd_019() {
    assert_eq!(
        prog_06_rem_cmd_019().exec(false),
        test_exec_06_rem_cmd_019_res()
    )
}
fn test_exec_06_rem_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_06_rem_cmd_020() {
    let src = fs::read_to_string("./assets/06_rem_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_020_res())
}
fn test_parse_06_rem_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_020())
}
fn prog_06_rem_cmd_020() -> Prog {
    Prog(vec![Rem])
}
#[test]
fn test_exec_06_rem_cmd_020() {
    assert_eq!(
        prog_06_rem_cmd_020().exec(false),
        test_exec_06_rem_cmd_020_res()
    )
}
fn test_exec_06_rem_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_07_eq_cmd_000() {
    let src = fs::read_to_string("./assets/07_eq_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_000_res())
}
fn test_parse_07_eq_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_000())
}
fn prog_07_eq_cmd_000() -> Prog {
    Prog(vec![Eq, Num(2), Num(-2147483648)])
}
#[test]
fn test_exec_07_eq_cmd_000() {
    assert_eq!(
        prog_07_eq_cmd_000().exec(false),
        test_exec_07_eq_cmd_000_res()
    )
}
fn test_exec_07_eq_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_001() {
    let src = fs::read_to_string("./assets/07_eq_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_001_res())
}
fn test_parse_07_eq_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_001())
}
fn prog_07_eq_cmd_001() -> Prog {
    Prog(vec![Eq, Num(5), Num(-10)])
}
#[test]
fn test_exec_07_eq_cmd_001() {
    assert_eq!(
        prog_07_eq_cmd_001().exec(false),
        test_exec_07_eq_cmd_001_res()
    )
}
fn test_exec_07_eq_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_005() {
    let src = fs::read_to_string("./assets/07_eq_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_005_res())
}
fn test_parse_07_eq_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_005())
}
fn prog_07_eq_cmd_005() -> Prog {
    Prog(vec![Eq, Num(-2147483648), Num(2147483647)])
}
#[test]
fn test_exec_07_eq_cmd_005() {
    assert_eq!(
        prog_07_eq_cmd_005().exec(false),
        test_exec_07_eq_cmd_005_res()
    )
}
fn test_exec_07_eq_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_006() {
    let src = fs::read_to_string("./assets/07_eq_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_006_res())
}
fn test_parse_07_eq_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_006())
}
fn prog_07_eq_cmd_006() -> Prog {
    Prog(vec![Eq, Num(0), Num(0)])
}
#[test]
fn test_exec_07_eq_cmd_006() {
    assert_eq!(
        prog_07_eq_cmd_006().exec(false),
        test_exec_07_eq_cmd_006_res()
    )
}
fn test_exec_07_eq_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_07_eq_cmd_007() {
    let src = fs::read_to_string("./assets/07_eq_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_007_res())
}
fn test_parse_07_eq_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_007())
}
fn prog_07_eq_cmd_007() -> Prog {
    Prog(vec![Eq, Num(-2147483647), Num(-2147483647)])
}
#[test]
fn test_exec_07_eq_cmd_007() {
    assert_eq!(
        prog_07_eq_cmd_007().exec(false),
        test_exec_07_eq_cmd_007_res()
    )
}
fn test_exec_07_eq_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_07_eq_cmd_008() {
    let src = fs::read_to_string("./assets/07_eq_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_008_res())
}
fn test_parse_07_eq_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_008())
}
fn prog_07_eq_cmd_008() -> Prog {
    Prog(vec![Eq, Num(-2), Num(10)])
}
#[test]
fn test_exec_07_eq_cmd_008() {
    assert_eq!(
        prog_07_eq_cmd_008().exec(false),
        test_exec_07_eq_cmd_008_res()
    )
}
fn test_exec_07_eq_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_009() {
    let src = fs::read_to_string("./assets/07_eq_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_009_res())
}
fn test_parse_07_eq_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_009())
}
fn prog_07_eq_cmd_009() -> Prog {
    Prog(vec![Eq, Num(3), Num(-3)])
}
#[test]
fn test_exec_07_eq_cmd_009() {
    assert_eq!(
        prog_07_eq_cmd_009().exec(false),
        test_exec_07_eq_cmd_009_res()
    )
}
fn test_exec_07_eq_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_010() {
    let src = fs::read_to_string("./assets/07_eq_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_010_res())
}
fn test_parse_07_eq_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_010())
}
fn prog_07_eq_cmd_010() -> Prog {
    Prog(vec![Eq, Num(2147483647), Num(5)])
}
#[test]
fn test_exec_07_eq_cmd_010() {
    assert_eq!(
        prog_07_eq_cmd_010().exec(false),
        test_exec_07_eq_cmd_010_res()
    )
}
fn test_exec_07_eq_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_011() {
    let src = fs::read_to_string("./assets/07_eq_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_011_res())
}
fn test_parse_07_eq_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_011())
}
fn prog_07_eq_cmd_011() -> Prog {
    Prog(vec![Eq, Num(1), Num(-5)])
}
#[test]
fn test_exec_07_eq_cmd_011() {
    assert_eq!(
        prog_07_eq_cmd_011().exec(false),
        test_exec_07_eq_cmd_011_res()
    )
}
fn test_exec_07_eq_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_012() {
    let src = fs::read_to_string("./assets/07_eq_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_012_res())
}
fn test_parse_07_eq_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_012())
}
fn prog_07_eq_cmd_012() -> Prog {
    Prog(vec![Eq, Num(10), Num(-2147483646)])
}
#[test]
fn test_exec_07_eq_cmd_012() {
    assert_eq!(
        prog_07_eq_cmd_012().exec(false),
        test_exec_07_eq_cmd_012_res()
    )
}
fn test_exec_07_eq_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_013() {
    let src = fs::read_to_string("./assets/07_eq_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_013_res())
}
fn test_parse_07_eq_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_013())
}
fn prog_07_eq_cmd_013() -> Prog {
    Prog(vec![Eq, Num(-5), Num(2147483648)])
}
#[test]
fn test_exec_07_eq_cmd_013() {
    assert_eq!(
        prog_07_eq_cmd_013().exec(false),
        test_exec_07_eq_cmd_013_res()
    )
}
fn test_exec_07_eq_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_014() {
    let src = fs::read_to_string("./assets/07_eq_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_014_res())
}
fn test_parse_07_eq_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_014())
}
fn prog_07_eq_cmd_014() -> Prog {
    Prog(vec![Eq, Num(-1), Num(-1)])
}
#[test]
fn test_exec_07_eq_cmd_014() {
    assert_eq!(
        prog_07_eq_cmd_014().exec(false),
        test_exec_07_eq_cmd_014_res()
    )
}
fn test_exec_07_eq_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_07_eq_cmd_015() {
    let src = fs::read_to_string("./assets/07_eq_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_015_res())
}
fn test_parse_07_eq_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_015())
}
fn prog_07_eq_cmd_015() -> Prog {
    Prog(vec![Eq, Num(-2147483646), Num(2147483646)])
}
#[test]
fn test_exec_07_eq_cmd_015() {
    assert_eq!(
        prog_07_eq_cmd_015().exec(false),
        test_exec_07_eq_cmd_015_res()
    )
}
fn test_exec_07_eq_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_016() {
    let src = fs::read_to_string("./assets/07_eq_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_016_res())
}
fn test_parse_07_eq_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_016())
}
fn prog_07_eq_cmd_016() -> Prog {
    Prog(vec![Eq, Num(-10), Num(2)])
}
#[test]
fn test_exec_07_eq_cmd_016() {
    assert_eq!(
        prog_07_eq_cmd_016().exec(false),
        test_exec_07_eq_cmd_016_res()
    )
}
fn test_exec_07_eq_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_018() {
    let src = fs::read_to_string("./assets/07_eq_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_018_res())
}
fn test_parse_07_eq_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_018())
}
fn prog_07_eq_cmd_018() -> Prog {
    Prog(vec![Eq, Num(-3), Num(3)])
}
#[test]
fn test_exec_07_eq_cmd_018() {
    assert_eq!(
        prog_07_eq_cmd_018().exec(false),
        test_exec_07_eq_cmd_018_res()
    )
}
fn test_exec_07_eq_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_019() {
    let src = fs::read_to_string("./assets/07_eq_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_019_res())
}
fn test_parse_07_eq_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_019())
}
fn prog_07_eq_cmd_019() -> Prog {
    Prog(vec![Eq, Num(3)])
}
#[test]
fn test_exec_07_eq_cmd_019() {
    assert_eq!(
        prog_07_eq_cmd_019().exec(false),
        test_exec_07_eq_cmd_019_res()
    )
}
fn test_exec_07_eq_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_07_eq_cmd_020() {
    let src = fs::read_to_string("./assets/07_eq_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_020_res())
}
fn test_parse_07_eq_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_020())
}
fn prog_07_eq_cmd_020() -> Prog {
    Prog(vec![Eq])
}
#[test]
fn test_exec_07_eq_cmd_020() {
    assert_eq!(
        prog_07_eq_cmd_020().exec(false),
        test_exec_07_eq_cmd_020_res()
    )
}
fn test_exec_07_eq_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_08_lt_cmd_000() {
    let src = fs::read_to_string("./assets/08_lt_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_000_res())
}
fn test_parse_08_lt_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_000())
}
fn prog_08_lt_cmd_000() -> Prog {
    Prog(vec![Lt, Num(2147483646), Num(3)])
}
#[test]
fn test_exec_08_lt_cmd_000() {
    assert_eq!(
        prog_08_lt_cmd_000().exec(false),
        test_exec_08_lt_cmd_000_res()
    )
}
fn test_exec_08_lt_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_001() {
    let src = fs::read_to_string("./assets/08_lt_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_001_res())
}
fn test_parse_08_lt_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_001())
}
fn prog_08_lt_cmd_001() -> Prog {
    Prog(vec![Lt, Num(3), Num(-10)])
}
#[test]
fn test_exec_08_lt_cmd_001() {
    assert_eq!(
        prog_08_lt_cmd_001().exec(false),
        test_exec_08_lt_cmd_001_res()
    )
}
fn test_exec_08_lt_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_002() {
    let src = fs::read_to_string("./assets/08_lt_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_002_res())
}
fn test_parse_08_lt_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_002())
}
fn prog_08_lt_cmd_002() -> Prog {
    Prog(vec![Lt, Num(10), Num(0)])
}
#[test]
fn test_exec_08_lt_cmd_002() {
    assert_eq!(
        prog_08_lt_cmd_002().exec(false),
        test_exec_08_lt_cmd_002_res()
    )
}
fn test_exec_08_lt_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_003() {
    let src = fs::read_to_string("./assets/08_lt_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_003_res())
}
fn test_parse_08_lt_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_003())
}
fn prog_08_lt_cmd_003() -> Prog {
    Prog(vec![Lt, Num(2), Num(-3)])
}
#[test]
fn test_exec_08_lt_cmd_003() {
    assert_eq!(
        prog_08_lt_cmd_003().exec(false),
        test_exec_08_lt_cmd_003_res()
    )
}
fn test_exec_08_lt_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_004() {
    let src = fs::read_to_string("./assets/08_lt_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_004_res())
}
fn test_parse_08_lt_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_004())
}
fn prog_08_lt_cmd_004() -> Prog {
    Prog(vec![Lt, Num(1), Num(-5)])
}
#[test]
fn test_exec_08_lt_cmd_004() {
    assert_eq!(
        prog_08_lt_cmd_004().exec(false),
        test_exec_08_lt_cmd_004_res()
    )
}
fn test_exec_08_lt_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_005() {
    let src = fs::read_to_string("./assets/08_lt_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_005_res())
}
fn test_parse_08_lt_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_005())
}
fn prog_08_lt_cmd_005() -> Prog {
    Prog(vec![Lt, Num(-5), Num(-1)])
}
#[test]
fn test_exec_08_lt_cmd_005() {
    assert_eq!(
        prog_08_lt_cmd_005().exec(false),
        test_exec_08_lt_cmd_005_res()
    )
}
fn test_exec_08_lt_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_006() {
    let src = fs::read_to_string("./assets/08_lt_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_006_res())
}
fn test_parse_08_lt_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_006())
}
fn prog_08_lt_cmd_006() -> Prog {
    Prog(vec![Lt, Num(0), Num(-2147483648)])
}
#[test]
fn test_exec_08_lt_cmd_006() {
    assert_eq!(
        prog_08_lt_cmd_006().exec(false),
        test_exec_08_lt_cmd_006_res()
    )
}
fn test_exec_08_lt_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_007() {
    let src = fs::read_to_string("./assets/08_lt_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_007_res())
}
fn test_parse_08_lt_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_007())
}
fn prog_08_lt_cmd_007() -> Prog {
    Prog(vec![Lt, Num(-2147483647), Num(2)])
}
#[test]
fn test_exec_08_lt_cmd_007() {
    assert_eq!(
        prog_08_lt_cmd_007().exec(false),
        test_exec_08_lt_cmd_007_res()
    )
}
fn test_exec_08_lt_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_009() {
    let src = fs::read_to_string("./assets/08_lt_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_009_res())
}
fn test_parse_08_lt_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_009())
}
fn prog_08_lt_cmd_009() -> Prog {
    Prog(vec![Lt, Num(-1), Num(-2147483646)])
}
#[test]
fn test_exec_08_lt_cmd_009() {
    assert_eq!(
        prog_08_lt_cmd_009().exec(false),
        test_exec_08_lt_cmd_009_res()
    )
}
fn test_exec_08_lt_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_010() {
    let src = fs::read_to_string("./assets/08_lt_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_010_res())
}
fn test_parse_08_lt_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_010())
}
fn prog_08_lt_cmd_010() -> Prog {
    Prog(vec![Lt, Num(-2), Num(1)])
}
#[test]
fn test_exec_08_lt_cmd_010() {
    assert_eq!(
        prog_08_lt_cmd_010().exec(false),
        test_exec_08_lt_cmd_010_res()
    )
}
fn test_exec_08_lt_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_011() {
    let src = fs::read_to_string("./assets/08_lt_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_011_res())
}
fn test_parse_08_lt_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_011())
}
fn prog_08_lt_cmd_011() -> Prog {
    Prog(vec![Lt, Num(-10), Num(10)])
}
#[test]
fn test_exec_08_lt_cmd_011() {
    assert_eq!(
        prog_08_lt_cmd_011().exec(false),
        test_exec_08_lt_cmd_011_res()
    )
}
fn test_exec_08_lt_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_012() {
    let src = fs::read_to_string("./assets/08_lt_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_012_res())
}
fn test_parse_08_lt_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_012())
}
fn prog_08_lt_cmd_012() -> Prog {
    Prog(vec![Lt, Num(2147483648), Num(-2147483647)])
}
#[test]
fn test_exec_08_lt_cmd_012() {
    assert_eq!(
        prog_08_lt_cmd_012().exec(false),
        test_exec_08_lt_cmd_012_res()
    )
}
fn test_exec_08_lt_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_014() {
    let src = fs::read_to_string("./assets/08_lt_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_014_res())
}
fn test_parse_08_lt_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_014())
}
fn prog_08_lt_cmd_014() -> Prog {
    Prog(vec![Lt, Num(-2147483646), Num(2147483646)])
}
#[test]
fn test_exec_08_lt_cmd_014() {
    assert_eq!(
        prog_08_lt_cmd_014().exec(false),
        test_exec_08_lt_cmd_014_res()
    )
}
fn test_exec_08_lt_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_015() {
    let src = fs::read_to_string("./assets/08_lt_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_015_res())
}
fn test_parse_08_lt_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_015())
}
fn prog_08_lt_cmd_015() -> Prog {
    Prog(vec![Lt, Num(2147483647), Num(-2)])
}
#[test]
fn test_exec_08_lt_cmd_015() {
    assert_eq!(
        prog_08_lt_cmd_015().exec(false),
        test_exec_08_lt_cmd_015_res()
    )
}
fn test_exec_08_lt_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_017() {
    let src = fs::read_to_string("./assets/08_lt_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_017_res())
}
fn test_parse_08_lt_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_017())
}
fn prog_08_lt_cmd_017() -> Prog {
    Prog(vec![Lt, Num(-2147483648), Num(5)])
}
#[test]
fn test_exec_08_lt_cmd_017() {
    assert_eq!(
        prog_08_lt_cmd_017().exec(false),
        test_exec_08_lt_cmd_017_res()
    )
}
fn test_exec_08_lt_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_019() {
    let src = fs::read_to_string("./assets/08_lt_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_019_res())
}
fn test_parse_08_lt_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_019())
}
fn prog_08_lt_cmd_019() -> Prog {
    Prog(vec![Lt, Num(2147483647)])
}
#[test]
fn test_exec_08_lt_cmd_019() {
    assert_eq!(
        prog_08_lt_cmd_019().exec(false),
        test_exec_08_lt_cmd_019_res()
    )
}
fn test_exec_08_lt_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_08_lt_cmd_020() {
    let src = fs::read_to_string("./assets/08_lt_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_020_res())
}
fn test_parse_08_lt_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_020())
}
fn prog_08_lt_cmd_020() -> Prog {
    Prog(vec![Lt])
}
#[test]
fn test_exec_08_lt_cmd_020() {
    assert_eq!(
        prog_08_lt_cmd_020().exec(false),
        test_exec_08_lt_cmd_020_res()
    )
}
fn test_exec_08_lt_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_09_gt_cmd_001() {
    let src = fs::read_to_string("./assets/09_gt_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_001_res())
}
fn test_parse_09_gt_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_001())
}
fn prog_09_gt_cmd_001() -> Prog {
    Prog(vec![Gt, Num(-5), Num(10)])
}
#[test]
fn test_exec_09_gt_cmd_001() {
    assert_eq!(
        prog_09_gt_cmd_001().exec(false),
        test_exec_09_gt_cmd_001_res()
    )
}
fn test_exec_09_gt_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_002() {
    let src = fs::read_to_string("./assets/09_gt_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_002_res())
}
fn test_parse_09_gt_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_002())
}
fn prog_09_gt_cmd_002() -> Prog {
    Prog(vec![Gt, Num(-2147483647), Num(-10)])
}
#[test]
fn test_exec_09_gt_cmd_002() {
    assert_eq!(
        prog_09_gt_cmd_002().exec(false),
        test_exec_09_gt_cmd_002_res()
    )
}
fn test_exec_09_gt_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_003() {
    let src = fs::read_to_string("./assets/09_gt_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_003_res())
}
fn test_parse_09_gt_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_003())
}
fn prog_09_gt_cmd_003() -> Prog {
    Prog(vec![Gt, Num(-2147483648), Num(1)])
}
#[test]
fn test_exec_09_gt_cmd_003() {
    assert_eq!(
        prog_09_gt_cmd_003().exec(false),
        test_exec_09_gt_cmd_003_res()
    )
}
fn test_exec_09_gt_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_004() {
    let src = fs::read_to_string("./assets/09_gt_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_004_res())
}
fn test_parse_09_gt_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_004())
}
fn prog_09_gt_cmd_004() -> Prog {
    Prog(vec![Gt, Num(3), Num(-2147483648)])
}
#[test]
fn test_exec_09_gt_cmd_004() {
    assert_eq!(
        prog_09_gt_cmd_004().exec(false),
        test_exec_09_gt_cmd_004_res()
    )
}
fn test_exec_09_gt_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_09_gt_cmd_005() {
    let src = fs::read_to_string("./assets/09_gt_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_005_res())
}
fn test_parse_09_gt_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_005())
}
fn prog_09_gt_cmd_005() -> Prog {
    Prog(vec![Gt, Num(-1), Num(2147483647)])
}
#[test]
fn test_exec_09_gt_cmd_005() {
    assert_eq!(
        prog_09_gt_cmd_005().exec(false),
        test_exec_09_gt_cmd_005_res()
    )
}
fn test_exec_09_gt_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_006() {
    let src = fs::read_to_string("./assets/09_gt_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_006_res())
}
fn test_parse_09_gt_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_006())
}
fn prog_09_gt_cmd_006() -> Prog {
    Prog(vec![Gt, Num(-2), Num(3)])
}
#[test]
fn test_exec_09_gt_cmd_006() {
    assert_eq!(
        prog_09_gt_cmd_006().exec(false),
        test_exec_09_gt_cmd_006_res()
    )
}
fn test_exec_09_gt_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_007() {
    let src = fs::read_to_string("./assets/09_gt_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_007_res())
}
fn test_parse_09_gt_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_007())
}
fn prog_09_gt_cmd_007() -> Prog {
    Prog(vec![Gt, Num(-2147483646), Num(-2147483646)])
}
#[test]
fn test_exec_09_gt_cmd_007() {
    assert_eq!(
        prog_09_gt_cmd_007().exec(false),
        test_exec_09_gt_cmd_007_res()
    )
}
fn test_exec_09_gt_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_008() {
    let src = fs::read_to_string("./assets/09_gt_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_008_res())
}
fn test_parse_09_gt_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_008())
}
fn prog_09_gt_cmd_008() -> Prog {
    Prog(vec![Gt, Num(0), Num(0)])
}
#[test]
fn test_exec_09_gt_cmd_008() {
    assert_eq!(
        prog_09_gt_cmd_008().exec(false),
        test_exec_09_gt_cmd_008_res()
    )
}
fn test_exec_09_gt_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_009() {
    let src = fs::read_to_string("./assets/09_gt_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_009_res())
}
fn test_parse_09_gt_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_009())
}
fn prog_09_gt_cmd_009() -> Prog {
    Prog(vec![Gt, Num(2147483646), Num(2147483646)])
}
#[test]
fn test_exec_09_gt_cmd_009() {
    assert_eq!(
        prog_09_gt_cmd_009().exec(false),
        test_exec_09_gt_cmd_009_res()
    )
}
fn test_exec_09_gt_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_010() {
    let src = fs::read_to_string("./assets/09_gt_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_010_res())
}
fn test_parse_09_gt_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_010())
}
fn prog_09_gt_cmd_010() -> Prog {
    Prog(vec![Gt, Num(10), Num(2147483648)])
}
#[test]
fn test_exec_09_gt_cmd_010() {
    assert_eq!(
        prog_09_gt_cmd_010().exec(false),
        test_exec_09_gt_cmd_010_res()
    )
}
fn test_exec_09_gt_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_011() {
    let src = fs::read_to_string("./assets/09_gt_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_011_res())
}
fn test_parse_09_gt_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_011())
}
fn prog_09_gt_cmd_011() -> Prog {
    Prog(vec![Gt, Num(2147483648), Num(-2)])
}
#[test]
fn test_exec_09_gt_cmd_011() {
    assert_eq!(
        prog_09_gt_cmd_011().exec(false),
        test_exec_09_gt_cmd_011_res()
    )
}
fn test_exec_09_gt_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_09_gt_cmd_012() {
    let src = fs::read_to_string("./assets/09_gt_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_012_res())
}
fn test_parse_09_gt_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_012())
}
fn prog_09_gt_cmd_012() -> Prog {
    Prog(vec![Gt, Num(1), Num(-2147483647)])
}
#[test]
fn test_exec_09_gt_cmd_012() {
    assert_eq!(
        prog_09_gt_cmd_012().exec(false),
        test_exec_09_gt_cmd_012_res()
    )
}
fn test_exec_09_gt_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_09_gt_cmd_014() {
    let src = fs::read_to_string("./assets/09_gt_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_014_res())
}
fn test_parse_09_gt_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_014())
}
fn prog_09_gt_cmd_014() -> Prog {
    Prog(vec![Gt, Num(-3), Num(5)])
}
#[test]
fn test_exec_09_gt_cmd_014() {
    assert_eq!(
        prog_09_gt_cmd_014().exec(false),
        test_exec_09_gt_cmd_014_res()
    )
}
fn test_exec_09_gt_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_015() {
    let src = fs::read_to_string("./assets/09_gt_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_015_res())
}
fn test_parse_09_gt_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_015())
}
fn prog_09_gt_cmd_015() -> Prog {
    Prog(vec![Gt, Num(2), Num(-5)])
}
#[test]
fn test_exec_09_gt_cmd_015() {
    assert_eq!(
        prog_09_gt_cmd_015().exec(false),
        test_exec_09_gt_cmd_015_res()
    )
}
fn test_exec_09_gt_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_09_gt_cmd_017() {
    let src = fs::read_to_string("./assets/09_gt_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_017_res())
}
fn test_parse_09_gt_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_017())
}
fn prog_09_gt_cmd_017() -> Prog {
    Prog(vec![Gt, Num(5), Num(-1)])
}
#[test]
fn test_exec_09_gt_cmd_017() {
    assert_eq!(
        prog_09_gt_cmd_017().exec(false),
        test_exec_09_gt_cmd_017_res()
    )
}
fn test_exec_09_gt_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_09_gt_cmd_020() {
    let src = fs::read_to_string("./assets/09_gt_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_020_res())
}
fn test_parse_09_gt_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_020())
}
fn prog_09_gt_cmd_020() -> Prog {
    Prog(vec![Gt])
}
#[test]
fn test_exec_09_gt_cmd_020() {
    assert_eq!(
        prog_09_gt_cmd_020().exec(false),
        test_exec_09_gt_cmd_020_res()
    )
}
fn test_exec_09_gt_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_00() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_00_res())
}
fn test_parse_10_ifz_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_00())
}
fn prog_10_ifz_cmd_00() -> Prog {
    Prog(vec![Ifz, Num(-2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_00() {
    assert_eq!(
        prog_10_ifz_cmd_00().exec(false),
        test_exec_10_ifz_cmd_00_res()
    )
}
fn test_exec_10_ifz_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_10_ifz_cmd_01() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_01_res())
}
fn test_parse_10_ifz_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_01())
}
fn prog_10_ifz_cmd_01() -> Prog {
    Prog(vec![Ifz, Num(-2)])
}
#[test]
fn test_exec_10_ifz_cmd_01() {
    assert_eq!(
        prog_10_ifz_cmd_01().exec(false),
        test_exec_10_ifz_cmd_01_res()
    )
}
fn test_exec_10_ifz_cmd_01_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_02() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_02_res())
}
fn test_parse_10_ifz_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_02())
}
fn prog_10_ifz_cmd_02() -> Prog {
    Prog(vec![Ifz, Num(-1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_02() {
    assert_eq!(
        prog_10_ifz_cmd_02().exec(false),
        test_exec_10_ifz_cmd_02_res()
    )
}
fn test_exec_10_ifz_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_10_ifz_cmd_03() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_03_res())
}
fn test_parse_10_ifz_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_03())
}
fn prog_10_ifz_cmd_03() -> Prog {
    Prog(vec![Ifz, Num(-1)])
}
#[test]
fn test_exec_10_ifz_cmd_03() {
    assert_eq!(
        prog_10_ifz_cmd_03().exec(false),
        test_exec_10_ifz_cmd_03_res()
    )
}
fn test_exec_10_ifz_cmd_03_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_04() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_04_res())
}
fn test_parse_10_ifz_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_04())
}
fn prog_10_ifz_cmd_04() -> Prog {
    Prog(vec![Ifz, Num(0), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_04() {
    assert_eq!(
        prog_10_ifz_cmd_04().exec(false),
        test_exec_10_ifz_cmd_04_res()
    )
}
fn test_exec_10_ifz_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_10_ifz_cmd_05() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_05_res())
}
fn test_parse_10_ifz_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_05())
}
fn prog_10_ifz_cmd_05() -> Prog {
    Prog(vec![Ifz, Num(0)])
}
#[test]
fn test_exec_10_ifz_cmd_05() {
    assert_eq!(
        prog_10_ifz_cmd_05().exec(false),
        test_exec_10_ifz_cmd_05_res()
    )
}
fn test_exec_10_ifz_cmd_05_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_06() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_06_res())
}
fn test_parse_10_ifz_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_06())
}
fn prog_10_ifz_cmd_06() -> Prog {
    Prog(vec![Ifz, Num(1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_06() {
    assert_eq!(
        prog_10_ifz_cmd_06().exec(false),
        test_exec_10_ifz_cmd_06_res()
    )
}
fn test_exec_10_ifz_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_10_ifz_cmd_07() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_07_res())
}
fn test_parse_10_ifz_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_07())
}
fn prog_10_ifz_cmd_07() -> Prog {
    Prog(vec![Ifz, Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_07() {
    assert_eq!(
        prog_10_ifz_cmd_07().exec(false),
        test_exec_10_ifz_cmd_07_res()
    )
}
fn test_exec_10_ifz_cmd_07_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_08() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_08_res())
}
fn test_parse_10_ifz_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_08())
}
fn prog_10_ifz_cmd_08() -> Prog {
    Prog(vec![Ifz, Num(2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_08() {
    assert_eq!(
        prog_10_ifz_cmd_08().exec(false),
        test_exec_10_ifz_cmd_08_res()
    )
}
fn test_exec_10_ifz_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_10_ifz_cmd_09() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_09_res())
}
fn test_parse_10_ifz_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_09())
}
fn prog_10_ifz_cmd_09() -> Prog {
    Prog(vec![Ifz, Num(2)])
}
#[test]
fn test_exec_10_ifz_cmd_09() {
    assert_eq!(
        prog_10_ifz_cmd_09().exec(false),
        test_exec_10_ifz_cmd_09_res()
    )
}
fn test_exec_10_ifz_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_10() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_10_res())
}
fn test_parse_10_ifz_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_10())
}
fn prog_10_ifz_cmd_10() -> Prog {
    Prog(vec![Ifz])
}
#[test]
fn test_exec_10_ifz_cmd_10() {
    assert_eq!(
        prog_10_ifz_cmd_10().exec(false),
        test_exec_10_ifz_cmd_10_res()
    )
}
fn test_exec_10_ifz_cmd_10_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_11_dup_cmd_00() {
    let src = fs::read_to_string("./assets/11_dup_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_00_res())
}
fn test_parse_11_dup_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_00())
}
fn prog_11_dup_cmd_00() -> Prog {
    Prog(vec![Dup, Num(-5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_00() {
    assert_eq!(
        prog_11_dup_cmd_00().exec(false),
        test_exec_11_dup_cmd_00_res()
    )
}
fn test_exec_11_dup_cmd_00_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_11_dup_cmd_01() {
    let src = fs::read_to_string("./assets/11_dup_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_01_res())
}
fn test_parse_11_dup_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_01())
}
fn prog_11_dup_cmd_01() -> Prog {
    Prog(vec![Dup, Num(-4), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_01() {
    assert_eq!(
        prog_11_dup_cmd_01().exec(false),
        test_exec_11_dup_cmd_01_res()
    )
}
fn test_exec_11_dup_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_11_dup_cmd_02() {
    let src = fs::read_to_string("./assets/11_dup_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_02_res())
}
fn test_parse_11_dup_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_02())
}
fn prog_11_dup_cmd_02() -> Prog {
    Prog(vec![Dup, Num(-3), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_02() {
    assert_eq!(
        prog_11_dup_cmd_02().exec(false),
        test_exec_11_dup_cmd_02_res()
    )
}
fn test_exec_11_dup_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_11_dup_cmd_03() {
    let src = fs::read_to_string("./assets/11_dup_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_03_res())
}
fn test_parse_11_dup_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_03())
}
fn prog_11_dup_cmd_03() -> Prog {
    Prog(vec![Dup, Num(-2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_03() {
    assert_eq!(
        prog_11_dup_cmd_03().exec(false),
        test_exec_11_dup_cmd_03_res()
    )
}
fn test_exec_11_dup_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_11_dup_cmd_04() {
    let src = fs::read_to_string("./assets/11_dup_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_04_res())
}
fn test_parse_11_dup_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_04())
}
fn prog_11_dup_cmd_04() -> Prog {
    Prog(vec![Dup, Num(-1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_04() {
    assert_eq!(
        prog_11_dup_cmd_04().exec(false),
        test_exec_11_dup_cmd_04_res()
    )
}
fn test_exec_11_dup_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_11_dup_cmd_05() {
    let src = fs::read_to_string("./assets/11_dup_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_05_res())
}
fn test_parse_11_dup_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_05())
}
fn prog_11_dup_cmd_05() -> Prog {
    Prog(vec![Dup, Num(0), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_05() {
    assert_eq!(
        prog_11_dup_cmd_05().exec(false),
        test_exec_11_dup_cmd_05_res()
    )
}
fn test_exec_11_dup_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_11_dup_cmd_06() {
    let src = fs::read_to_string("./assets/11_dup_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_06_res())
}
fn test_parse_11_dup_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_06())
}
fn prog_11_dup_cmd_06() -> Prog {
    Prog(vec![Dup, Num(1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_06() {
    assert_eq!(
        prog_11_dup_cmd_06().exec(false),
        test_exec_11_dup_cmd_06_res()
    )
}
fn test_exec_11_dup_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_11_dup_cmd_07() {
    let src = fs::read_to_string("./assets/11_dup_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_07_res())
}
fn test_parse_11_dup_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_07())
}
fn prog_11_dup_cmd_07() -> Prog {
    Prog(vec![Dup, Num(2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_07() {
    assert_eq!(
        prog_11_dup_cmd_07().exec(false),
        test_exec_11_dup_cmd_07_res()
    )
}
fn test_exec_11_dup_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_11_dup_cmd_08() {
    let src = fs::read_to_string("./assets/11_dup_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_08_res())
}
fn test_parse_11_dup_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_08())
}
fn prog_11_dup_cmd_08() -> Prog {
    Prog(vec![Dup, Num(3), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_08() {
    assert_eq!(
        prog_11_dup_cmd_08().exec(false),
        test_exec_11_dup_cmd_08_res()
    )
}
fn test_exec_11_dup_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_11_dup_cmd_09() {
    let src = fs::read_to_string("./assets/11_dup_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_09_res())
}
fn test_parse_11_dup_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_09())
}
fn prog_11_dup_cmd_09() -> Prog {
    Prog(vec![Dup, Num(4), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_09() {
    assert_eq!(
        prog_11_dup_cmd_09().exec(false),
        test_exec_11_dup_cmd_09_res()
    )
}
fn test_exec_11_dup_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_11_dup_cmd_10() {
    let src = fs::read_to_string("./assets/11_dup_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_10_res())
}
fn test_parse_11_dup_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_10())
}
fn prog_11_dup_cmd_10() -> Prog {
    Prog(vec![Dup, Num(5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_10() {
    assert_eq!(
        prog_11_dup_cmd_10().exec(false),
        test_exec_11_dup_cmd_10_res()
    )
}
fn test_exec_11_dup_cmd_10_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_11_dup_cmd_11() {
    let src = fs::read_to_string("./assets/11_dup_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_11_res())
}
fn test_parse_11_dup_cmd_11_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_11())
}
fn prog_11_dup_cmd_11() -> Prog {
    Prog(vec![Dup])
}
#[test]
fn test_exec_11_dup_cmd_11() {
    assert_eq!(
        prog_11_dup_cmd_11().exec(false),
        test_exec_11_dup_cmd_11_res()
    )
}
fn test_exec_11_dup_cmd_11_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_00() {
    let src = fs::read_to_string("./assets/12_pop_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_00_res())
}
fn test_parse_12_pop_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_00())
}
fn prog_12_pop_cmd_00() -> Prog {
    Prog(vec![Pop, Num(5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_12_pop_cmd_00() {
    assert_eq!(
        prog_12_pop_cmd_00().exec(false),
        test_exec_12_pop_cmd_00_res()
    )
}
fn test_exec_12_pop_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_12_pop_cmd_01() {
    let src = fs::read_to_string("./assets/12_pop_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_01_res())
}
fn test_parse_12_pop_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_01())
}
fn prog_12_pop_cmd_01() -> Prog {
    Prog(vec![Pop, Pop, Num(5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_12_pop_cmd_01() {
    assert_eq!(
        prog_12_pop_cmd_01().exec(false),
        test_exec_12_pop_cmd_01_res()
    )
}
fn test_exec_12_pop_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_12_pop_cmd_02() {
    let src = fs::read_to_string("./assets/12_pop_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_02_res())
}
fn test_parse_12_pop_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_02())
}
fn prog_12_pop_cmd_02() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_02() {
    assert_eq!(
        prog_12_pop_cmd_02().exec(false),
        test_exec_12_pop_cmd_02_res()
    )
}
fn test_exec_12_pop_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_12_pop_cmd_03() {
    let src = fs::read_to_string("./assets/12_pop_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_03_res())
}
fn test_parse_12_pop_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_03())
}
fn prog_12_pop_cmd_03() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_03() {
    assert_eq!(
        prog_12_pop_cmd_03().exec(false),
        test_exec_12_pop_cmd_03_res()
    )
}
fn test_exec_12_pop_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_12_pop_cmd_04() {
    let src = fs::read_to_string("./assets/12_pop_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_04_res())
}
fn test_parse_12_pop_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_04())
}
fn prog_12_pop_cmd_04() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_04() {
    assert_eq!(
        prog_12_pop_cmd_04().exec(false),
        test_exec_12_pop_cmd_04_res()
    )
}
fn test_exec_12_pop_cmd_04_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_05() {
    let src = fs::read_to_string("./assets/12_pop_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_05_res())
}
fn test_parse_12_pop_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_05())
}
fn prog_12_pop_cmd_05() -> Prog {
    Prog(vec![
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_05() {
    assert_eq!(
        prog_12_pop_cmd_05().exec(false),
        test_exec_12_pop_cmd_05_res()
    )
}
fn test_exec_12_pop_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_12_pop_cmd_06() {
    let src = fs::read_to_string("./assets/12_pop_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_06_res())
}
fn test_parse_12_pop_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_06())
}
fn prog_12_pop_cmd_06() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_06() {
    assert_eq!(
        prog_12_pop_cmd_06().exec(false),
        test_exec_12_pop_cmd_06_res()
    )
}
fn test_exec_12_pop_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(-6)
}

#[test]
fn test_parse_12_pop_cmd_07() {
    let src = fs::read_to_string("./assets/12_pop_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_07_res())
}
fn test_parse_12_pop_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_07())
}
fn prog_12_pop_cmd_07() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_07() {
    assert_eq!(
        prog_12_pop_cmd_07().exec(false),
        test_exec_12_pop_cmd_07_res()
    )
}
fn test_exec_12_pop_cmd_07_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_08() {
    let src = fs::read_to_string("./assets/12_pop_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_08_res())
}
fn test_parse_12_pop_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_08())
}
fn prog_12_pop_cmd_08() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_08() {
    assert_eq!(
        prog_12_pop_cmd_08().exec(false),
        test_exec_12_pop_cmd_08_res()
    )
}
fn test_exec_12_pop_cmd_08_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_09() {
    let src = fs::read_to_string("./assets/12_pop_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_09_res())
}
fn test_parse_12_pop_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_09())
}
fn prog_12_pop_cmd_09() -> Prog {
    Prog(vec![Pop])
}
#[test]
fn test_exec_12_pop_cmd_09() {
    assert_eq!(
        prog_12_pop_cmd_09().exec(false),
        test_exec_12_pop_cmd_09_res()
    )
}
fn test_exec_12_pop_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_13_swap_cmd_00() {
    let src = fs::read_to_string("./assets/13_swap_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_00_res())
}
fn test_parse_13_swap_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_00())
}
fn prog_13_swap_cmd_00() -> Prog {
    Prog(vec![
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_00() {
    assert_eq!(
        prog_13_swap_cmd_00().exec(false),
        test_exec_13_swap_cmd_00_res()
    )
}
fn test_exec_13_swap_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(-8)
}

#[test]
fn test_parse_13_swap_cmd_01() {
    let src = fs::read_to_string("./assets/13_swap_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_01_res())
}
fn test_parse_13_swap_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_01())
}
fn prog_13_swap_cmd_01() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_01() {
    assert_eq!(
        prog_13_swap_cmd_01().exec(false),
        test_exec_13_swap_cmd_01_res()
    )
}
fn test_exec_13_swap_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_13_swap_cmd_02() {
    let src = fs::read_to_string("./assets/13_swap_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_02_res())
}
fn test_parse_13_swap_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_02())
}
fn prog_13_swap_cmd_02() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_02() {
    assert_eq!(
        prog_13_swap_cmd_02().exec(false),
        test_exec_13_swap_cmd_02_res()
    )
}
fn test_exec_13_swap_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-6)
}

#[test]
fn test_parse_13_swap_cmd_03() {
    let src = fs::read_to_string("./assets/13_swap_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_03_res())
}
fn test_parse_13_swap_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_03())
}
fn prog_13_swap_cmd_03() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_03() {
    assert_eq!(
        prog_13_swap_cmd_03().exec(false),
        test_exec_13_swap_cmd_03_res()
    )
}
fn test_exec_13_swap_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_13_swap_cmd_04() {
    let src = fs::read_to_string("./assets/13_swap_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_04_res())
}
fn test_parse_13_swap_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_04())
}
fn prog_13_swap_cmd_04() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_04() {
    assert_eq!(
        prog_13_swap_cmd_04().exec(false),
        test_exec_13_swap_cmd_04_res()
    )
}
fn test_exec_13_swap_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_13_swap_cmd_05() {
    let src = fs::read_to_string("./assets/13_swap_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_05_res())
}
fn test_parse_13_swap_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_05())
}
fn prog_13_swap_cmd_05() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_05() {
    assert_eq!(
        prog_13_swap_cmd_05().exec(false),
        test_exec_13_swap_cmd_05_res()
    )
}
fn test_exec_13_swap_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_13_swap_cmd_06() {
    let src = fs::read_to_string("./assets/13_swap_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_06_res())
}
fn test_parse_13_swap_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_06())
}
fn prog_13_swap_cmd_06() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_06() {
    assert_eq!(
        prog_13_swap_cmd_06().exec(false),
        test_exec_13_swap_cmd_06_res()
    )
}
fn test_exec_13_swap_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_13_swap_cmd_07() {
    let src = fs::read_to_string("./assets/13_swap_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_07_res())
}
fn test_parse_13_swap_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_07())
}
fn prog_13_swap_cmd_07() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_07() {
    assert_eq!(
        prog_13_swap_cmd_07().exec(false),
        test_exec_13_swap_cmd_07_res()
    )
}
fn test_exec_13_swap_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_13_swap_cmd_08() {
    let src = fs::read_to_string("./assets/13_swap_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_08_res())
}
fn test_parse_13_swap_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_08())
}
fn prog_13_swap_cmd_08() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_08() {
    assert_eq!(
        prog_13_swap_cmd_08().exec(false),
        test_exec_13_swap_cmd_08_res()
    )
}
fn test_exec_13_swap_cmd_08_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_13_swap_cmd_09() {
    let src = fs::read_to_string("./assets/13_swap_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_09_res())
}
fn test_parse_13_swap_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_09())
}
fn prog_13_swap_cmd_09() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_09() {
    assert_eq!(
        prog_13_swap_cmd_09().exec(false),
        test_exec_13_swap_cmd_09_res()
    )
}
fn test_exec_13_swap_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_13_swap_cmd_10() {
    let src = fs::read_to_string("./assets/13_swap_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_10_res())
}
fn test_parse_13_swap_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_10())
}
fn prog_13_swap_cmd_10() -> Prog {
    Prog(vec![Swap])
}
#[test]
fn test_exec_13_swap_cmd_10() {
    assert_eq!(
        prog_13_swap_cmd_10().exec(false),
        test_exec_13_swap_cmd_10_res()
    )
}
fn test_exec_13_swap_cmd_10_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_00() {
    let src = fs::read_to_string("./assets/14_rev_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_00_res())
}
fn test_parse_14_rev_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_00())
}
fn prog_14_rev_cmd_00() -> Prog {
    Prog(vec![
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_00() {
    assert_eq!(
        prog_14_rev_cmd_00().exec(false),
        test_exec_14_rev_cmd_00_res()
    )
}
fn test_exec_14_rev_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_01() {
    let src = fs::read_to_string("./assets/14_rev_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_01_res())
}
fn test_parse_14_rev_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_01())
}
fn prog_14_rev_cmd_01() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_01() {
    assert_eq!(
        prog_14_rev_cmd_01().exec(false),
        test_exec_14_rev_cmd_01_res()
    )
}
fn test_exec_14_rev_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_14_rev_cmd_02() {
    let src = fs::read_to_string("./assets/14_rev_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_02_res())
}
fn test_parse_14_rev_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_02())
}
fn prog_14_rev_cmd_02() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_02() {
    assert_eq!(
        prog_14_rev_cmd_02().exec(false),
        test_exec_14_rev_cmd_02_res()
    )
}
fn test_exec_14_rev_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_14_rev_cmd_03() {
    let src = fs::read_to_string("./assets/14_rev_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_03_res())
}
fn test_parse_14_rev_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_03())
}
fn prog_14_rev_cmd_03() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_03() {
    assert_eq!(
        prog_14_rev_cmd_03().exec(false),
        test_exec_14_rev_cmd_03_res()
    )
}
fn test_exec_14_rev_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-8)
}

#[test]
fn test_parse_14_rev_cmd_04() {
    let src = fs::read_to_string("./assets/14_rev_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_04_res())
}
fn test_parse_14_rev_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_04())
}
fn prog_14_rev_cmd_04() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_04() {
    assert_eq!(
        prog_14_rev_cmd_04().exec(false),
        test_exec_14_rev_cmd_04_res()
    )
}
fn test_exec_14_rev_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_05() {
    let src = fs::read_to_string("./assets/14_rev_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_05_res())
}
fn test_parse_14_rev_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_05())
}
fn prog_14_rev_cmd_05() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_05() {
    assert_eq!(
        prog_14_rev_cmd_05().exec(false),
        test_exec_14_rev_cmd_05_res()
    )
}
fn test_exec_14_rev_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_14_rev_cmd_06() {
    let src = fs::read_to_string("./assets/14_rev_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_06_res())
}
fn test_parse_14_rev_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_06())
}
fn prog_14_rev_cmd_06() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_06() {
    assert_eq!(
        prog_14_rev_cmd_06().exec(false),
        test_exec_14_rev_cmd_06_res()
    )
}
fn test_exec_14_rev_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_14_rev_cmd_07() {
    let src = fs::read_to_string("./assets/14_rev_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_07_res())
}
fn test_parse_14_rev_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_07())
}
fn prog_14_rev_cmd_07() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_07() {
    assert_eq!(
        prog_14_rev_cmd_07().exec(false),
        test_exec_14_rev_cmd_07_res()
    )
}
fn test_exec_14_rev_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(-6)
}

#[test]
fn test_parse_14_rev_cmd_08() {
    let src = fs::read_to_string("./assets/14_rev_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_08_res())
}
fn test_parse_14_rev_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_08())
}
fn prog_14_rev_cmd_08() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_08() {
    assert_eq!(
        prog_14_rev_cmd_08().exec(false),
        test_exec_14_rev_cmd_08_res()
    )
}
fn test_exec_14_rev_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_09() {
    let src = fs::read_to_string("./assets/14_rev_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_09_res())
}
fn test_parse_14_rev_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_09())
}
fn prog_14_rev_cmd_09() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_09() {
    assert_eq!(
        prog_14_rev_cmd_09().exec(false),
        test_exec_14_rev_cmd_09_res()
    )
}
fn test_exec_14_rev_cmd_09_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_14_rev_cmd_10() {
    let src = fs::read_to_string("./assets/14_rev_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_10_res())
}
fn test_parse_14_rev_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_10())
}
fn prog_14_rev_cmd_10() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_10() {
    assert_eq!(
        prog_14_rev_cmd_10().exec(false),
        test_exec_14_rev_cmd_10_res()
    )
}
fn test_exec_14_rev_cmd_10_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_14_rev_cmd_11() {
    let src = fs::read_to_string("./assets/14_rev_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_11_res())
}
fn test_parse_14_rev_cmd_11_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_11())
}
fn prog_14_rev_cmd_11() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_11() {
    assert_eq!(
        prog_14_rev_cmd_11().exec(false),
        test_exec_14_rev_cmd_11_res()
    )
}
fn test_exec_14_rev_cmd_11_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_14_rev_cmd_12() {
    let src = fs::read_to_string("./assets/14_rev_cmd_12.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_12_res())
}
fn test_parse_14_rev_cmd_12_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_12())
}
fn prog_14_rev_cmd_12() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_12() {
    assert_eq!(
        prog_14_rev_cmd_12().exec(false),
        test_exec_14_rev_cmd_12_res()
    )
}
fn test_exec_14_rev_cmd_12_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_13() {
    let src = fs::read_to_string("./assets/14_rev_cmd_13.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_13_res())
}
fn test_parse_14_rev_cmd_13_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_13())
}
fn prog_14_rev_cmd_13() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_13() {
    assert_eq!(
        prog_14_rev_cmd_13().exec(false),
        test_exec_14_rev_cmd_13_res()
    )
}
fn test_exec_14_rev_cmd_13_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_14_rev_cmd_14() {
    let src = fs::read_to_string("./assets/14_rev_cmd_14.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_14_res())
}
fn test_parse_14_rev_cmd_14_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_14())
}
fn prog_14_rev_cmd_14() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_14() {
    assert_eq!(
        prog_14_rev_cmd_14().exec(false),
        test_exec_14_rev_cmd_14_res()
    )
}
fn test_exec_14_rev_cmd_14_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_14_rev_cmd_15() {
    let src = fs::read_to_string("./assets/14_rev_cmd_15.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_15_res())
}
fn test_parse_14_rev_cmd_15_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_15())
}
fn prog_14_rev_cmd_15() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_15() {
    assert_eq!(
        prog_14_rev_cmd_15().exec(false),
        test_exec_14_rev_cmd_15_res()
    )
}
fn test_exec_14_rev_cmd_15_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_14_rev_cmd_16() {
    let src = fs::read_to_string("./assets/14_rev_cmd_16.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_16_res())
}
fn test_parse_14_rev_cmd_16_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_16())
}
fn prog_14_rev_cmd_16() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_16() {
    assert_eq!(
        prog_14_rev_cmd_16().exec(false),
        test_exec_14_rev_cmd_16_res()
    )
}
fn test_exec_14_rev_cmd_16_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_17() {
    let src = fs::read_to_string("./assets/14_rev_cmd_17.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_17_res())
}
fn test_parse_14_rev_cmd_17_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_17())
}
fn prog_14_rev_cmd_17() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_17() {
    assert_eq!(
        prog_14_rev_cmd_17().exec(false),
        test_exec_14_rev_cmd_17_res()
    )
}
fn test_exec_14_rev_cmd_17_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_18() {
    let src = fs::read_to_string("./assets/14_rev_cmd_18.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_18_res())
}
fn test_parse_14_rev_cmd_18_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_18())
}
fn prog_14_rev_cmd_18() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_18() {
    assert_eq!(
        prog_14_rev_cmd_18().exec(false),
        test_exec_14_rev_cmd_18_res()
    )
}
fn test_exec_14_rev_cmd_18_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_19() {
    let src = fs::read_to_string("./assets/14_rev_cmd_19.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_19_res())
}
fn test_parse_14_rev_cmd_19_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_19())
}
fn prog_14_rev_cmd_19() -> Prog {
    Prog(vec![Rev])
}
#[test]
fn test_exec_14_rev_cmd_19() {
    assert_eq!(
        prog_14_rev_cmd_19().exec(false),
        test_exec_14_rev_cmd_19_res()
    )
}
fn test_exec_14_rev_cmd_19_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_15_cmds_cmd_00() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_00_res())
}
fn test_parse_15_cmds_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_00())
}
fn prog_15_cmds_cmd_00() -> Prog {
    Prog(vec![Num(3), Num(2), Num(1), Cmds(vec![])])
}
#[test]
fn test_exec_15_cmds_cmd_00() {
    assert_eq!(
        prog_15_cmds_cmd_00().exec(false),
        test_exec_15_cmds_cmd_00_res()
    )
}
fn test_exec_15_cmds_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_15_cmds_cmd_01() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_01_res())
}
fn test_parse_15_cmds_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_01())
}
fn prog_15_cmds_cmd_01() -> Prog {
    Prog(vec![Num(3), Num(2), Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_15_cmds_cmd_01() {
    assert_eq!(
        prog_15_cmds_cmd_01().exec(false),
        test_exec_15_cmds_cmd_01_res()
    )
}
fn test_exec_15_cmds_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_15_cmds_cmd_02() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_02_res())
}
fn test_parse_15_cmds_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_02())
}
fn prog_15_cmds_cmd_02() -> Prog {
    Prog(vec![Num(3), Cmds(vec![Num(2), Num(1)])])
}
#[test]
fn test_exec_15_cmds_cmd_02() {
    assert_eq!(
        prog_15_cmds_cmd_02().exec(false),
        test_exec_15_cmds_cmd_02_res()
    )
}
fn test_exec_15_cmds_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_15_cmds_cmd_03() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_03_res())
}
fn test_parse_15_cmds_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_03())
}
fn prog_15_cmds_cmd_03() -> Prog {
    Prog(vec![Cmds(vec![Num(3), Num(2), Num(1)])])
}
#[test]
fn test_exec_15_cmds_cmd_03() {
    assert_eq!(
        prog_15_cmds_cmd_03().exec(false),
        test_exec_15_cmds_cmd_03_res()
    )
}
fn test_exec_15_cmds_cmd_03_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_15_cmds_cmd_04() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_04_res())
}
fn test_parse_15_cmds_cmd_04_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_15_cmds_cmd_05() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_05_res())
}
fn test_parse_15_cmds_cmd_05_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_15_cmds_cmd_06() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_06_res())
}
fn test_parse_15_cmds_cmd_06_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_15_cmds_cmd_07() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_07_res())
}
fn test_parse_15_cmds_cmd_07_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_15_cmds_cmd_08() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_08_res())
}
fn test_parse_15_cmds_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_08())
}
fn prog_15_cmds_cmd_08() -> Prog {
    Prog(vec![Ifz, Cmds(vec![]), Num(2), Num(3)])
}
#[test]
fn test_exec_15_cmds_cmd_08() {
    assert_eq!(
        prog_15_cmds_cmd_08().exec(false),
        test_exec_15_cmds_cmd_08_res()
    )
}
fn test_exec_15_cmds_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_15_cmds_cmd_09() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_09_res())
}
fn test_parse_15_cmds_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_09())
}
fn prog_15_cmds_cmd_09() -> Prog {
    Prog(vec![Cmds(vec![Mul, Num(2)])])
}
#[test]
fn test_exec_15_cmds_cmd_09() {
    assert_eq!(
        prog_15_cmds_cmd_09().exec(false),
        test_exec_15_cmds_cmd_09_res()
    )
}
fn test_exec_15_cmds_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_00() {
    let src = fs::read_to_string("./assets/16_exec_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_00_res())
}
fn test_parse_16_exec_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_00())
}
fn prog_16_exec_cmd_00() -> Prog {
    Prog(vec![Exec, Swap, Num(3), Cmds(vec![Mul, Num(4)])])
}
#[test]
fn test_exec_16_exec_cmd_00() {
    assert_eq!(
        prog_16_exec_cmd_00().exec(false),
        test_exec_16_exec_cmd_00_res()
    )
}
fn test_exec_16_exec_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_16_exec_cmd_01() {
    let src = fs::read_to_string("./assets/16_exec_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_01_res())
}
fn test_parse_16_exec_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_01())
}
fn prog_16_exec_cmd_01() -> Prog {
    Prog(vec![Exec, Cmds(vec![Mul, Num(2)]), Num(7)])
}
#[test]
fn test_exec_16_exec_cmd_01() {
    assert_eq!(
        prog_16_exec_cmd_01().exec(false),
        test_exec_16_exec_cmd_01_res()
    )
}
fn test_exec_16_exec_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(14)
}

#[test]
fn test_parse_16_exec_cmd_02() {
    let src = fs::read_to_string("./assets/16_exec_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_02_res())
}
fn test_parse_16_exec_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_02())
}
fn prog_16_exec_cmd_02() -> Prog {
    Prog(vec![Exec, Swap, Num(7), Cmds(vec![Sub, Num(0)])])
}
#[test]
fn test_exec_16_exec_cmd_02() {
    assert_eq!(
        prog_16_exec_cmd_02().exec(false),
        test_exec_16_exec_cmd_02_res()
    )
}
fn test_exec_16_exec_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-7)
}

#[test]
fn test_parse_16_exec_cmd_03() {
    let src = fs::read_to_string("./assets/16_exec_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_03_res())
}
fn test_parse_16_exec_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_03())
}
fn prog_16_exec_cmd_03() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Cmds(vec![Exec, Swap, Num(7)]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_03() {
    assert_eq!(
        prog_16_exec_cmd_03().exec(false),
        test_exec_16_exec_cmd_03_res()
    )
}
fn test_exec_16_exec_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-7)
}

#[test]
fn test_parse_16_exec_cmd_04() {
    let src = fs::read_to_string("./assets/16_exec_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_04_res())
}
fn test_parse_16_exec_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_04())
}
fn prog_16_exec_cmd_04() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Exec,
        Swap,
        Dup,
        Num(3),
        Cmds(vec![Mul, Dup, Num(0)]),
        Cmds(vec![Sub, Swap, Mul]),
        Num(-10),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_04() {
    assert_eq!(
        prog_16_exec_cmd_04().exec(false),
        test_exec_16_exec_cmd_04_res()
    )
}
fn test_exec_16_exec_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(42)
}

#[test]
fn test_parse_16_exec_cmd_05() {
    let src = fs::read_to_string("./assets/16_exec_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_05_res())
}
fn test_parse_16_exec_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_05())
}
fn prog_16_exec_cmd_05() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![Mul]),
        Swap,
        Cmds(vec![Add]),
        Lt,
        Swap,
        Num(3),
        Num(4),
        Num(5),
        Num(6),
    ])
}
#[test]
fn test_exec_16_exec_cmd_05() {
    assert_eq!(
        prog_16_exec_cmd_05().exec(false),
        test_exec_16_exec_cmd_05_res()
    )
}
fn test_exec_16_exec_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(30)
}

#[test]
fn test_parse_16_exec_cmd_06() {
    let src = fs::read_to_string("./assets/16_exec_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_06_res())
}
fn test_parse_16_exec_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_06())
}
fn prog_16_exec_cmd_06() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![Mul]),
        Swap,
        Cmds(vec![Add]),
        Lt,
        Swap,
        Num(4),
        Num(3),
        Num(5),
        Num(6),
    ])
}
#[test]
fn test_exec_16_exec_cmd_06() {
    assert_eq!(
        prog_16_exec_cmd_06().exec(false),
        test_exec_16_exec_cmd_06_res()
    )
}
fn test_exec_16_exec_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(11)
}

#[test]
fn test_parse_16_exec_cmd_07() {
    let src = fs::read_to_string("./assets/16_exec_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_07_res())
}
fn test_parse_16_exec_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_07())
}
fn prog_16_exec_cmd_07() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![]),
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Gt,
        Num(0),
        Dup,
        Num(0),
        Num(-7),
    ])
}
#[test]
fn test_exec_16_exec_cmd_07() {
    assert_eq!(
        prog_16_exec_cmd_07().exec(false),
        test_exec_16_exec_cmd_07_res()
    )
}
fn test_exec_16_exec_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_16_exec_cmd_08() {
    let src = fs::read_to_string("./assets/16_exec_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_08_res())
}
fn test_parse_16_exec_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_08())
}
fn prog_16_exec_cmd_08() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![]),
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Gt,
        Num(0),
        Dup,
        Num(0),
        Num(7),
    ])
}
#[test]
fn test_exec_16_exec_cmd_08() {
    assert_eq!(
        prog_16_exec_cmd_08().exec(false),
        test_exec_16_exec_cmd_08_res()
    )
}
fn test_exec_16_exec_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_16_exec_cmd_09() {
    let src = fs::read_to_string("./assets/16_exec_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_09_res())
}
fn test_parse_16_exec_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_09())
}
fn prog_16_exec_cmd_09() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Num(1),
        Cmds(vec![Sub, Swap, Mul, Num(2), Swap]),
        Num(10),
    ])
}
#[test]
fn test_exec_16_exec_cmd_09() {
    assert_eq!(
        prog_16_exec_cmd_09().exec(false),
        test_exec_16_exec_cmd_09_res()
    )
}
fn test_exec_16_exec_cmd_09_res() -> Result<i64, ProgExecError> {
    Ok(-19)
}

#[test]
fn test_parse_16_exec_cmd_10() {
    let src = fs::read_to_string("./assets/16_exec_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_10_res())
}
fn test_parse_16_exec_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_10())
}
fn prog_16_exec_cmd_10() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Num(3),
        Cmds(vec![Exec, Cmds(vec![Mul, Num(2)]), Num(5)]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_10() {
    assert_eq!(
        prog_16_exec_cmd_10().exec(false),
        test_exec_16_exec_cmd_10_res()
    )
}
fn test_exec_16_exec_cmd_10_res() -> Result<i64, ProgExecError> {
    Ok(10)
}

#[test]
fn test_parse_16_exec_cmd_11() {
    let src = fs::read_to_string("./assets/16_exec_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_11_res())
}
fn test_parse_16_exec_cmd_11_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_11())
}
fn prog_16_exec_cmd_11() -> Prog {
    Prog(vec![Exec, Cmds(vec![Exec, Cmds(vec![])]), Num(99)])
}
#[test]
fn test_exec_16_exec_cmd_11() {
    assert_eq!(
        prog_16_exec_cmd_11().exec(false),
        test_exec_16_exec_cmd_11_res()
    )
}
fn test_exec_16_exec_cmd_11_res() -> Result<i64, ProgExecError> {
    Ok(99)
}

#[test]
fn test_parse_16_exec_cmd_12() {
    let src = fs::read_to_string("./assets/16_exec_cmd_12.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_12_res())
}
fn test_parse_16_exec_cmd_12_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_12())
}
fn prog_16_exec_cmd_12() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Num(0),
        Cmds(vec![Mul]),
        Cmds(vec![Add]),
        Num(3),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_12() {
    assert_eq!(
        prog_16_exec_cmd_12().exec(false),
        test_exec_16_exec_cmd_12_res()
    )
}
fn test_exec_16_exec_cmd_12_res() -> Result<i64, ProgExecError> {
    Ok(6)
}

#[test]
fn test_parse_16_exec_cmd_13() {
    let src = fs::read_to_string("./assets/16_exec_cmd_13.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_13_res())
}
fn test_parse_16_exec_cmd_13_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_13())
}
fn prog_16_exec_cmd_13() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Num(0),
        Cmds(vec![Add, Num(3), Num(2)]),
        Num(4),
    ])
}
#[test]
fn test_exec_16_exec_cmd_13() {
    assert_eq!(
        prog_16_exec_cmd_13().exec(false),
        test_exec_16_exec_cmd_13_res()
    )
}
fn test_exec_16_exec_cmd_13_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_16_exec_cmd_14() {
    let src = fs::read_to_string("./assets/16_exec_cmd_14.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_14_res())
}
fn test_parse_16_exec_cmd_14_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_14())
}
fn prog_16_exec_cmd_14() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Lt,
        Num(6),
        Num(5),
        Cmds(vec![Add, Num(3), Num(2)]),
        Num(4),
    ])
}
#[test]
fn test_exec_16_exec_cmd_14() {
    assert_eq!(
        prog_16_exec_cmd_14().exec(false),
        test_exec_16_exec_cmd_14_res()
    )
}
fn test_exec_16_exec_cmd_14_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_16_exec_cmd_15() {
    let src = fs::read_to_string("./assets/16_exec_cmd_15.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_15_res())
}
fn test_parse_16_exec_cmd_15_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_15())
}
fn prog_16_exec_cmd_15() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Num(3),
        Swap,
        Cmds(vec![Mul, Num(2)]),
        Swap,
        Cmds(vec![Sub, Swap, Num(1)]),
        Cmds(vec![Exec, Swap, Exec, Swap]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_15() {
    assert_eq!(
        prog_16_exec_cmd_15().exec(false),
        test_exec_16_exec_cmd_15_res()
    )
}
fn test_exec_16_exec_cmd_15_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_16_exec_cmd_16() {
    let src = fs::read_to_string("./assets/16_exec_cmd_16.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_16_res())
}
fn test_parse_16_exec_cmd_16_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_16())
}
fn prog_16_exec_cmd_16() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(0),
    ])
}
#[test]
fn test_exec_16_exec_cmd_16() {
    assert_eq!(
        prog_16_exec_cmd_16().exec(false),
        test_exec_16_exec_cmd_16_res()
    )
}
fn test_exec_16_exec_cmd_16_res() -> Result<i64, ProgExecError> {
    Ok(-5)
}

#[test]
fn test_parse_16_exec_cmd_17() {
    let src = fs::read_to_string("./assets/16_exec_cmd_17.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_17_res())
}
fn test_parse_16_exec_cmd_17_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_17())
}
fn prog_16_exec_cmd_17() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(1),
    ])
}
#[test]
fn test_exec_16_exec_cmd_17() {
    assert_eq!(
        prog_16_exec_cmd_17().exec(false),
        test_exec_16_exec_cmd_17_res()
    )
}
fn test_exec_16_exec_cmd_17_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_16_exec_cmd_18() {
    let src = fs::read_to_string("./assets/16_exec_cmd_18.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_18_res())
}
fn test_parse_16_exec_cmd_18_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_18())
}
fn prog_16_exec_cmd_18() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_18() {
    assert_eq!(
        prog_16_exec_cmd_18().exec(false),
        test_exec_16_exec_cmd_18_res()
    )
}
fn test_exec_16_exec_cmd_18_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_16_exec_cmd_19() {
    let src = fs::read_to_string("./assets/16_exec_cmd_19.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_19_res())
}
fn test_parse_16_exec_cmd_19_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_19())
}
fn prog_16_exec_cmd_19() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_19() {
    assert_eq!(
        prog_16_exec_cmd_19().exec(false),
        test_exec_16_exec_cmd_19_res()
    )
}
fn test_exec_16_exec_cmd_19_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_16_exec_cmd_20() {
    let src = fs::read_to_string("./assets/16_exec_cmd_20.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_20_res())
}
fn test_parse_16_exec_cmd_20_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_20())
}
fn prog_16_exec_cmd_20() -> Prog {
    Prog(vec![Exec, Num(42)])
}
#[test]
fn test_exec_16_exec_cmd_20() {
    assert_eq!(
        prog_16_exec_cmd_20().exec(false),
        test_exec_16_exec_cmd_20_res()
    )
}
fn test_exec_16_exec_cmd_20_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_21() {
    let src = fs::read_to_string("./assets/16_exec_cmd_21.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_21_res())
}
fn test_parse_16_exec_cmd_21_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_21())
}
fn prog_16_exec_cmd_21() -> Prog {
    Prog(vec![
        Swap,
        Num(3),
        Cmds(vec![Exec, Cmds(vec![Mul, Num(2)]), Num(5)]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_21() {
    assert_eq!(
        prog_16_exec_cmd_21().exec(false),
        test_exec_16_exec_cmd_21_res()
    )
}
fn test_exec_16_exec_cmd_21_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_22() {
    let src = fs::read_to_string("./assets/16_exec_cmd_22.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_22_res())
}
fn test_parse_16_exec_cmd_22_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_22())
}
fn prog_16_exec_cmd_22() -> Prog {
    Prog(vec![Exec, Cmds(vec![Exec, Cmds(vec![])])])
}
#[test]
fn test_exec_16_exec_cmd_22() {
    assert_eq!(
        prog_16_exec_cmd_22().exec(false),
        test_exec_16_exec_cmd_22_res()
    )
}
fn test_exec_16_exec_cmd_22_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_23() {
    let src = fs::read_to_string("./assets/16_exec_cmd_23.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_23_res())
}
fn test_parse_16_exec_cmd_23_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_23())
}
fn prog_16_exec_cmd_23() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Num(1),
        Cmds(vec![Add, Num(3), Num(2)]),
        Num(4),
    ])
}
#[test]
fn test_exec_16_exec_cmd_23() {
    assert_eq!(
        prog_16_exec_cmd_23().exec(false),
        test_exec_16_exec_cmd_23_res()
    )
}
fn test_exec_16_exec_cmd_23_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_80_tak() {
    let src = fs::read_to_string("./assets/80_tak.bir").unwrap();
    assert_eq!(src.parse(), test_parse_80_tak_res())
}
fn test_parse_80_tak_res() -> Result<Prog, ProgParseError> {
    Ok(prog_80_tak())
}
fn prog_80_tak() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(24),
        Num(16),
        Num(8),
        Cmds(vec![
            Exec,
            Ifz,
            Sub,
            Num(1),
            Lt,
            Dup,
            Num(4),
            Dup,
            Num(2),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(4),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(1),
                Dup,
                Num(1),
            ]),
            Cmds(vec![Pop, Pop]),
        ]),
    ])
}
#[test]
fn test_exec_80_tak() {
    assert_eq!(prog_80_tak().exec(false), test_exec_80_tak_res())
}
fn test_exec_80_tak_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_81_tak() {
    let src = fs::read_to_string("./assets/81_tak.bir").unwrap();
    assert_eq!(src.parse(), test_parse_81_tak_res())
}
fn test_parse_81_tak_res() -> Result<Prog, ProgParseError> {
    Ok(prog_81_tak())
}
fn prog_81_tak() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(12),
        Num(8),
        Num(4),
        Cmds(vec![
            Exec,
            Ifz,
            Sub,
            Num(1),
            Lt,
            Dup,
            Num(4),
            Dup,
            Num(2),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(4),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(1),
                Dup,
                Num(1),
            ]),
            Cmds(vec![Pop, Pop]),
        ]),
    ])
}
#[test]
fn test_exec_81_tak() {
    assert_eq!(prog_81_tak().exec(false), test_exec_81_tak_res())
}
fn test_exec_81_tak_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_82_tak() {
    let src = fs::read_to_string("./assets/82_tak.bir").unwrap();
    assert_eq!(src.parse(), test_parse_82_tak_res())
}
fn test_parse_82_tak_res() -> Result<Prog, ProgParseError> {
    Ok(prog_82_tak())
}
fn prog_82_tak() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(24),
        Num(16),
        Num(8),
        Cmds(vec![
            Exec,
            Ifz,
            Sub,
            Num(1),
            Lt,
            Dup,
            Num(4),
            Dup,
            Num(2),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(4),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(1),
                Dup,
                Num(1),
            ]),
            Cmds(vec![Pop, Pop]),
        ]),
    ])
}
#[test]
fn test_exec_82_tak() {
    assert_eq!(prog_82_tak().exec(false), test_exec_82_tak_res())
}
fn test_exec_82_tak_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_50_fact() {
    let src = fs::read_to_string("./assets/50_fact.bir").unwrap();
    assert_eq!(src.parse(), test_parse_50_fact_res())
}
fn test_parse_50_fact_res() -> Result<Prog, ProgParseError> {
    Ok(prog_50_fact())
}
fn prog_50_fact() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Mul,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(5),
    ])
}
#[test]
fn test_exec_50_fact() {
    assert_eq!(prog_50_fact().exec(false), test_exec_50_fact_res())
}
fn test_exec_50_fact_res() -> Result<i64, ProgExecError> {
    Ok(120)
}

#[test]
fn test_parse_51_fact() {
    let src = fs::read_to_string("./assets/51_fact.bir").unwrap();
    assert_eq!(src.parse(), test_parse_51_fact_res())
}
fn test_parse_51_fact_res() -> Result<Prog, ProgParseError> {
    Ok(prog_51_fact())
}
fn prog_51_fact() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Mul,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(11),
    ])
}
#[test]
fn test_exec_51_fact() {
    assert_eq!(prog_51_fact().exec(false), test_exec_51_fact_res())
}
fn test_exec_51_fact_res() -> Result<i64, ProgExecError> {
    Ok(39916800)
}

#[test]
fn test_parse_52_fact() {
    let src = fs::read_to_string("./assets/52_fact.bir").unwrap();
    assert_eq!(src.parse(), test_parse_52_fact_res())
}
fn test_parse_52_fact_res() -> Result<Prog, ProgParseError> {
    Ok(prog_52_fact())
}
fn prog_52_fact() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Mul,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(20),
    ])
}
#[test]
fn test_exec_52_fact() {
    assert_eq!(prog_52_fact().exec(false), test_exec_52_fact_res())
}
fn test_exec_52_fact_res() -> Result<i64, ProgExecError> {
    Ok(2432902008176640000)
}

#[test]
fn test_parse_60_fib() {
    let src = fs::read_to_string("./assets/60_fib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_60_fib_res())
}
fn test_parse_60_fib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_60_fib())
}
fn prog_60_fib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Add,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(2),
                Swap,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(5),
    ])
}
#[test]
fn test_exec_60_fib() {
    assert_eq!(prog_60_fib().exec(false), test_exec_60_fib_res())
}
fn test_exec_60_fib_res() -> Result<i64, ProgExecError> {
    Ok(8)
}

#[test]
fn test_parse_61_fib() {
    let src = fs::read_to_string("./assets/61_fib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_61_fib_res())
}
fn test_parse_61_fib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_61_fib())
}
fn prog_61_fib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Add,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(2),
                Swap,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(15),
    ])
}
#[test]
fn test_exec_61_fib() {
    assert_eq!(prog_61_fib().exec(false), test_exec_61_fib_res())
}
fn test_exec_61_fib_res() -> Result<i64, ProgExecError> {
    Ok(987)
}

#[test]
fn test_parse_62_fib() {
    let src = fs::read_to_string("./assets/62_fib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_62_fib_res())
}
fn test_parse_62_fib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_62_fib())
}
fn prog_62_fib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Add,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(2),
                Swap,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(30),
    ])
}
#[test]
fn test_exec_62_fib() {
    assert_eq!(prog_62_fib().exec(false), test_exec_62_fib_res())
}
fn test_exec_62_fib_res() -> Result<i64, ProgExecError> {
    Ok(1346269)
}

#[test]
fn test_parse_70_tailfib() {
    let src = fs::read_to_string("./assets/70_tailfib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_70_tailfib_res())
}
fn test_parse_70_tailfib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_70_tailfib())
}
fn prog_70_tailfib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(0),
        Num(1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Dup,
            Num(4),
            Cmds(vec![Pop, Swap, Pop, Swap]),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Add,
                Dup,
                Num(4),
                Dup,
                Num(0),
                Dup,
                Num(1),
                Sub,
                Dup,
                Num(3),
                Num(1),
            ]),
        ]),
        Rev,
        Num(5),
    ])
}
#[test]
fn test_exec_70_tailfib() {
    assert_eq!(prog_70_tailfib().exec(false), test_exec_70_tailfib_res())
}
fn test_exec_70_tailfib_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_71_tailfib() {
    let src = fs::read_to_string("./assets/71_tailfib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_71_tailfib_res())
}
fn test_parse_71_tailfib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_71_tailfib())
}
fn prog_71_tailfib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(0),
        Num(1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Dup,
            Num(4),
            Cmds(vec![Pop, Swap, Pop, Swap]),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Add,
                Dup,
                Num(4),
                Dup,
                Num(0),
                Dup,
                Num(1),
                Sub,
                Dup,
                Num(3),
                Num(1),
            ]),
        ]),
        Rev,
        Num(50),
    ])
}
#[test]
fn test_exec_71_tailfib() {
    assert_eq!(prog_71_tailfib().exec(false), test_exec_71_tailfib_res())
}
fn test_exec_71_tailfib_res() -> Result<i64, ProgExecError> {
    Ok(12586269025)
}

#[test]
fn test_parse_72_tailfib() {
    let src = fs::read_to_string("./assets/72_tailfib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_72_tailfib_res())
}
fn test_parse_72_tailfib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_72_tailfib())
}
fn prog_72_tailfib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(0),
        Num(1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Dup,
            Num(4),
            Cmds(vec![Pop, Swap, Pop, Swap]),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Add,
                Dup,
                Num(4),
                Dup,
                Num(0),
                Dup,
                Num(1),
                Sub,
                Dup,
                Num(3),
                Num(1),
            ]),
        ]),
        Rev,
        Num(92),
    ])
}
#[test]
fn test_exec_72_tailfib() {
    assert_eq!(prog_72_tailfib().exec(false), test_exec_72_tailfib_res())
}
fn test_exec_72_tailfib_res() -> Result<i64, ProgExecError> {
    Ok(7540113804746346429)
}

#[test]
fn test_parse_15_cmds_cmd_10() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_10_res())
}
fn test_parse_15_cmds_cmd_10_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_15_cmds_cmd_11() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_11_res())
}
fn test_parse_15_cmds_cmd_11_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}
