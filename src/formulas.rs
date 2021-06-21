use std::time::Duration;

pub fn gross_wpm(chars: usize, time: Duration) -> usize {
    ((chars as f32 / 5.1f32) / (time.as_secs_f32() / 60f32)).round() as usize
}
