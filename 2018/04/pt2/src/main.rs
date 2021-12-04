use std::str::FromStr;
use std::fs::read_to_string;
use std::collections::{BTreeMap, HashMap};
use std::ops::{Add, AddAssign};

extern crate regex;
use regex::Regex;

#[derive(Debug)]
enum Action {
    Wake,
    Sleep,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falls asleep" => Ok(Action::Sleep),
            "wakes up" => Ok(Action::Wake),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Log {
    Action(Action),
    NewGuard(i32),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Add<u32> for Timestamp {
    type Output = Timestamp;

    fn add(mut self, rhs: u32) -> Timestamp {
        self.minute += rhs;

        self.hour += self.minute / 60;
        self.minute = self.minute % 60;

        self.day += self.hour / 24;
        self.hour = self.hour % 24;

        self
    }
}
impl AddAssign<u32> for Timestamp {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

fn main() {
    //                           1,    2,    3,    4,    5,                6,      7
    let schema = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (?:Guard #(\d+) )?(begins shift|falls asleep|wakes up)").unwrap();

    let input = read_to_string("input").unwrap();

    let parsed_log: BTreeMap<_, _> = input.split_terminator('\n')
        .map(|l| {
            let captures = schema.captures(l).unwrap();

            let ts = Timestamp {
                year: captures[1].parse().unwrap(),
                month: captures[2].parse().unwrap(),
                day: captures[3].parse().unwrap(),
                hour: captures[4].parse().unwrap(),
                minute: captures[5].parse().unwrap(),
            };

            if captures.get(6).is_some() {
                (ts, Log::NewGuard(captures[6].parse().unwrap()))
            } else {
                (ts, Log::Action(captures[7].parse().unwrap()))
            }
        })
        .collect();

    let mut guard_sleep_times = HashMap::new();

    let mut guard = None;
    let mut sleep_time = None;

    parsed_log.iter()
        .for_each(
            |(ts, msg)| {
                match msg {
                    Log::NewGuard(new_guard) => {
                        guard = Some(*new_guard);
                    },
                    Log::Action(action) => match action {
                        Action::Sleep => {
                            sleep_time = Some(*ts);
                        }
                        Action::Wake => {
                            let sleep_time_c = sleep_time.take();

                            if let (Some(guard), Some(sleep_time)) = (guard, sleep_time_c) {
                                let sleep_times = guard_sleep_times.entry(guard)
                                    .or_insert_with(Vec::new);

                                sleep_times.push((sleep_time, ts));
                            }
                        }
                    }
                }
            }
        );

    let (&max_guard, max_minute_id, _max_minute_count) = guard_sleep_times.iter()
        .map(
            |(guard, sleep_times)| sleep_times.iter()
                .fold((guard, [0; 60]), |(guard, mut minutes), (sleep_time, &wake_time)| {
                    let mut time = *sleep_time;

                    while time < wake_time {
                        minutes[time.minute as usize] += 1;

                        time += 1;
                    }

                    (guard, minutes)
                })
        )
        .map(|(guard, minutes)| {
            let (key, value) = minutes.iter().enumerate()
                .max_by_key(|(_, m)| *m)
                .unwrap();

            (guard, key as i32, *value)
        })
        .max_by_key(|(_, _, m)| *m)
        .unwrap();

    // println!("{:?}", guard_sleep_times);

    println!("{} at {} -> {}", max_guard, max_minute_id, max_guard * max_minute_id);
}
