use chrono::{Duration, TimeZone};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};

struct ImportantEvent {
    what: String,
    when: DateTime<Local>,
}
trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        let now = Local::now();
        let difference  = self.when - now;
        difference < Duration::zero()
    }
}


fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.from_local_datetime(&NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
                .and_hms_milli_opt(23, 59, 59, 99).unwrap()).unwrap(),
    };
    
    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::now() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::now() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}

