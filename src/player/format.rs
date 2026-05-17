use std::time::Duration;

pub fn format_duration(d: Duration) -> String {
    let mins = d.as_secs() / 60;
    let secs = d.as_secs() % 60;
    format!("{:02}:{:02}", mins, secs)
}
