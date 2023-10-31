use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let months = ["Jan", "Feb", "Mar", "Apr", "May",
        "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    
    let delimiters = ['/', '-', '.', ' ', '\\'];
    let date_arr: Vec<&str> = text
        .split(delimiters.as_slice())
        .collect();

    if date_arr.len() != 3 { return None; } 

    let mut year: Option<i32> = None;
    let mut month: Option<u32> = None;
    let mut day: Option<u32> = None;
    
    for part in date_arr {
        if part.len() >= 3 {
            match months.iter().position(|&r| r == part) {
                Some(m_val) => month = Some(m_val as u32 + 1),
                None => { year =  match part.parse() {
                    Ok(yr) => Some(yr),
                    Err(_) => None,
                } } 
            }
        } else if month.is_none() && year.is_some() {
            month = match part.parse() {
                Ok(mth) => Some(mth),
                Err(_) => None,
            }
        } else if day.is_none() {
            day = match part.parse() {
                Ok(d) => Some(d),
                Err(_) => None,
            }
        }
    }

    if day.is_none() || month.is_none() || year.is_none() {
        None
    } else {
        NaiveDate::from_ymd_opt(year.unwrap_or_default(), 
            month.unwrap_or_default(),
            day.unwrap_or_default())
    }

}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        Some(NaiveDate::from_ymd(2010, 12, 11))
    )
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        Some(NaiveDate::from_ymd(1999, 3, 2))
    )
}

#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        Some(NaiveDate::from_ymd(2021, 3, 1))
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        Some(NaiveDate::from_ymd(2021, 4, 5))
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
