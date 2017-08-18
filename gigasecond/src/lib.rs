extern crate chrono;
use chrono::*;

extern crate time;
use time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    return start + Duration::seconds(1000000000);
}
