use chrono::{DateTime, TimeZone, Utc, Timelike};

type GuardId = u16;

#[derive(Debug)]
enum GuardEventType {
    ShiftBegin(GuardId),
    FallsAsleep,
    WakesUp
}

type TimeStamp = DateTime<Utc>;

#[derive(Debug)]
struct GuardEvent(TimeStamp, GuardEventType);

impl GuardEvent {
    fn from_str<T: std::borrow::Borrow<str>>(input: &T) -> Option<GuardEvent> {
        let mut parts = input.borrow().split("] ");

        let timestamp = {
            Utc.datetime_from_str(&parts.next()?[1..], "%F %H:%M").ok()?
        };

        let parts: Vec<_> = parts.next()?.split(' ').collect();

        match parts[0] {
            "Guard" =>
                Some(GuardEvent(timestamp, GuardEventType::ShiftBegin(parts[1][1..].parse().ok()?))),
            
            "falls" =>
                Some(GuardEvent(timestamp, GuardEventType::FallsAsleep)),

            "wakes" =>
                Some(GuardEvent(timestamp, GuardEventType::WakesUp)),

            _ => None
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
struct GuardState {
    total_sleep: u32,
    sleep_times: HashMap<u8, u16>
}

fn main() {
    let mut input = shared::input::read_stdin_lines()
        .expect("could not lock stdin")
        .iter()
        .filter_map(GuardEvent::from_str)
        .collect::<Vec<GuardEvent>>();
    
    input.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sleepy_guards: HashMap<GuardId, GuardState> = HashMap::new();
    let mut active_guard = None;
    let mut sleep_time = None;

    for event in input {
        let timestamp = event.0;
        let event = event.1;

        match event {
            GuardEventType::ShiftBegin(guard_id) => {
                active_guard = Some(guard_id);

                sleepy_guards.entry(guard_id).or_insert(GuardState {
                    total_sleep: 0,
                    sleep_times: HashMap::new()
                });
            }

            GuardEventType::FallsAsleep => {
                sleep_time = Some(timestamp);
            }

            GuardEventType::WakesUp => {
                let state = if let Some(guard_id) = active_guard {
                    sleepy_guards.get_mut(&guard_id).unwrap()
                } else {
                    // Waking up a non-existing person
                    continue;
                };

                let start = sleep_time.unwrap();
                let dur = timestamp - start;

                for i in 0..dur.num_minutes() as u32 {
                    let ctr = state.sleep_times
                        .entry(((start.minute() + i) % 60) as u8)
                        .or_insert(0);

                    *ctr += 1;
                }

                state.total_sleep += dur.num_minutes() as u32;
            }
        }
    }

    // Find guard asleep the longest
    let most_asleep = sleepy_guards
        .iter()
        .max_by(|a, b| a.1.total_sleep.cmp(&b.1.total_sleep)) // max by total sleep time
        .unwrap();

    // Get his most sleepy minute
    let minute = most_asleep.1.sleep_times
        .iter()
        .max_by(|a, b| a.1.cmp(b.1)) // max by minute
        .unwrap();

    println!("Part 1: Guard {} most asleep ({}) at minute {} ({} times)", most_asleep.0, most_asleep.1.total_sleep, minute.0, minute.1);
    println!(" => {}", *most_asleep.0 as u32 * *minute.0 as u32);

    let most_frequent = sleepy_guards
        .iter()
        .filter_map(|guard| {
            // Turn id:guard into (id, minute, count_asleep)
            let most = guard.1.sleep_times
                .iter()
                .max_by(|a, b| a.1.cmp(b.1))?; // Find most often asleep minute

            Some((guard.0, most.0, most.1))
        })
        .max_by(|a, b| a.2.cmp(&b.2)) // max by count
        .unwrap();
    
    println!("Part 2: Guard {} was most frequently asleep at minute {} ({} times)",
        most_frequent.0,
        most_frequent.1,
        most_frequent.2);

    println!(" => {}", *most_frequent.0 as u32 * *most_frequent.1 as u32);
}
