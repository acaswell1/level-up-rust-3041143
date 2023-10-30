use chrono::prelude::*;

fn weeks_between(a: &str, b: &str) -> i32 {
    match NaiveDate::parse_from_str(a, "%Y-%m-%d") {
        Ok(birth_a) => match NaiveDate::parse_from_str(b, "%Y-%m-%d") {
            Ok(birth_b) => (birth_b - birth_a).num_weeks().try_into().unwrap_or_default(),
            Err(_) => { println!("Unpredicted: 1 Error"); -1 },
        },
        Err(_) => { println!("Unpredicted: 2 Error"); -1 },
    }
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    println!("{}", n_weeks);
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
