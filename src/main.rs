use chrono::{DateTime, Utc};
use std::fmt::{self, Display, Formatter};

const SECONDS_IN_DAY: i64 = 86400;
const SECONDS_IN_HECTODAY: i64 = SECONDS_IN_DAY / 100;
const SECONDS_IN_MILLIDAY: f64 = (SECONDS_IN_HECTODAY as f64) / 10.0;

#[derive(Copy, Clone, Debug)]
struct HectoTime {
    pub hectodays: i64,
    pub millidays: f64,
}

impl Display for HectoTime {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}.{:.02}", self.hectodays, self.millidays)
    }
}

fn chrono_to_hectotime(ts: DateTime<Utc>) -> HectoTime {
    let midnight = ts.date_naive().and_hms_opt(0, 0, 0).unwrap().timestamp();
    let ts = ts.timestamp();

    let seconds = ts - midnight;
    let hectodays = seconds.div_euclid(SECONDS_IN_HECTODAY);
    let remainder = seconds.rem_euclid(SECONDS_IN_HECTODAY);
    let millidays = (remainder as f64) / SECONDS_IN_MILLIDAY;
    HectoTime {
        hectodays,
        millidays,
    }
}

fn main() {
    let now = Utc::now();
    let ht = chrono_to_hectotime(now);
    println!("time: {ht}");
}
