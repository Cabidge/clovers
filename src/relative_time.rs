use std::fmt::Display;

use chrono::{DateTime, Utc};

pub struct Relative(pub DateTime<Utc>);

impl Display for Relative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = Utc::now();
        let diff = now - self.0;

        let (unit, value) = if diff.num_weeks() > 3 {
            // If it's been more than 3 weeks, just show the date.
            return write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S %Z"));
        } else if diff.num_weeks() > 0 {
            ("week", diff.num_weeks()) 
        } else if diff.num_days() > 0 {
            ("day", diff.num_days())
        } else if diff.num_hours() > 0 {
            ("hour", diff.num_hours())
        } else if diff.num_minutes() > 0 {
            ("minute", diff.num_minutes())
        } else if diff.num_seconds() < 2 {
            // If it's been less than 2 seconds, just say "just now".
            return write!(f, "just now");
        } else {
            ("second", diff.num_seconds())
        };

        write!(f, "{} {}{} ago", value, unit, if value == 1 { "" } else { "s" })
    }
}
