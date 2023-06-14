use time::PrimitiveDateTime as DateTime;

const GIGASECOND: time::Duration = time::Duration::seconds(1000000000);

// Returns a DateTime one billion seconds after start.
pub fn after(start: &DateTime) -> DateTime {
    start.clone().saturating_add(GIGASECOND)
}
