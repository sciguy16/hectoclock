use chrono::{DateTime, Utc};
use std::fmt::{self, Display, Formatter};

const MICROS_IN_DAY: i64 = 86_400_000_000;
const MICROS_IN_HECTODAY: i64 = MICROS_IN_DAY / 100;
const MICROS_IN_MILLIDAY: i64 = MICROS_IN_HECTODAY / 10;
const MICROS_IN_MICRODAY: i64 = MICROS_IN_MILLIDAY / 1000;

#[derive(Copy, Clone, Debug)]
struct HectoTime {
    pub hectodays: i64,
    pub millidays: i64,
    pub microdays: i64,
}

impl Display for HectoTime {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}.{}.{:03}",
            self.hectodays, self.millidays, self.microdays,
        )
    }
}

fn chrono_to_hectotime(ts: DateTime<Utc>) -> HectoTime {
    let midnight = ts
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .timestamp_micros();
    let ts = ts.timestamp_micros();

    let micros = ts - midnight;
    let hectodays = micros.div_euclid(MICROS_IN_HECTODAY);
    let remainder = micros.rem_euclid(MICROS_IN_HECTODAY);
    let millidays = remainder.div_euclid(MICROS_IN_MILLIDAY);
    let remainder = remainder.rem_euclid(MICROS_IN_MILLIDAY);
    let microdays = remainder.div_euclid(MICROS_IN_MICRODAY);
    HectoTime {
        hectodays,
        millidays,
        microdays,
    }
}

fn main() {
    let now = Utc::now();
    let ht = chrono_to_hectotime(now);
    println!("time: {ht}");
}
