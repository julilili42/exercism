use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start
        .checked_add(Duration::seconds(1_000_000_000))
        .unwrap_or(start)
}

pub fn afterBad(start: DateTime) -> DateTime {
    if let Some(added) = start.checked_add(Duration::seconds(1_000_000_000)) {
        return added;
    } else {
        start
    }
}
