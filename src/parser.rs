use crate::model;
use anyhow::{bail, Context};
use chrono::naive::NaiveDate;
use diesel::prelude::*;
use scan_fmt::scan_fmt;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Race {
    track: String,
    laps: Option<i32>,
    minutes: Option<i32>,
    entrants: Vec<Entrant>,
}

impl Race {
    pub fn insert_into(
        self,
        conn: &MysqlConnection,
        race_id: i32,
        date: NaiveDate,
    ) -> anyhow::Result<()> {
        conn.transaction(|| {
            let new_race = model::Race {
                id: race_id,
                date,
                track: self.track,
                laps: self.laps,
                minutes: self.minutes,
            };
            {
                use crate::schema::races::dsl::*;
                new_race.insert_into(races).execute(conn)?;
            }

            for entrant in self.entrants {
                let new_user = model::UserName { name: entrant.name };
                let user_id = new_user.get_or_insert(conn)?;
                let new_entrant = model::RaceEntrant {
                    race_id,
                    user_id,
                    position: Some(entrant.position),
                    vehicle: Some(entrant.vehicle),
                    time: entrant.time.map(|dur| dur.as_millis() as i32),
                    best_lap: entrant.best_lap.map(|dur| dur.as_millis() as i32),
                    lap: entrant.lap,
                    reason: entrant.reason.map(Into::into),
                    ping: entrant.ping,
                    fps: entrant.fps,
                    fps_locked: entrant.fps_locked,
                };
                {
                    use crate::schema::race_entrants::dsl::*;
                    new_entrant.insert_into(race_entrants).execute(conn)?;
                }
            }
            Ok(())
        })
    }
}

#[derive(Debug, Clone)]
struct Entrant {
    position: i32,
    name: String,
    vehicle: String,
    time: Option<Duration>,
    best_lap: Option<Duration>,
    lap: Option<i32>,
    reason: Option<Reason>,
    ping: Option<i32>,
    fps: Option<i32>,
    fps_locked: bool,
}

#[derive(Debug, Clone)]
enum Reason {
    Dns,
    Dnf,
    Dsq,
}

impl From<Reason> for model::Reason {
    fn from(reason: Reason) -> model::Reason {
        match reason {
            Reason::Dns => model::Reason::Dns,
            Reason::Dnf => model::Reason::Dnf,
            Reason::Dsq => model::Reason::Dsq,
        }
    }
}

pub fn parse_race(text: &str) -> anyhow::Result<Race> {
    let document = Document::from(text);

    let score_table = document
        .find(Name("table").and(Attr("id", "scoreTable")))
        .next()
        .context("score table not found in race result")?;

    let mut rows = score_table.find(Name("tr"));

    let mut heading = || {
        rows.next()
            .and_then(|tr| tr.find(Name("th")).next())
            .and_then(|th| th.first_child())
            .and_then(|text| text.as_text())
            .map(|s| s.trim())
            .context("heading not found")
    };

    let track = heading()?.to_string();
    let mut race = Race {
        track,
        laps: None,
        minutes: None,
        entrants: Vec::new(),
    };

    let mode_str = heading()?;
    if let Ok(laps) = scan_fmt!(mode_str, "Laps: {}", i32) {
        race.laps = Some(laps);
    } else if let Ok(minutes) = scan_fmt!(mode_str, "Minutes: {}", i32) {
        race.minutes = Some(minutes);
    } else if mode_str == "Rally" {
        //no-op
    } else if mode_str == "Point-to-Point" {
        //no-op
    } else {
        bail!("unrecognized mode string {:?}", mode_str);
    };

    // parse heading
    let heading = rows.next().context("missing heading")?;
    enum Field {
        FpsLocked,
        Pos,
        Name,
        Vehicle,
        Time,
        BestLap,
        Lap,
        Ping,
        Fps,
        Ignored,
    }
    let fields: Vec<Field> = heading
        .find(Name("th"))
        .map(|node| match node.text().trim() {
            "" => Field::FpsLocked,
            "Pos" => Field::Pos,
            "Name" => Field::Name,
            "Vehicle" => Field::Vehicle,
            "Time" => Field::Time,
            "BestLap" => Field::BestLap,
            "Lap" => Field::Lap,
            "PING" => Field::Ping,
            "FPS" => Field::Fps,
            _ => Field::Ignored,
        })
        .collect();

    let mut leader_time = Duration::from_millis(0);

    race.entrants = rows
        .map::<anyhow::Result<Entrant>, _>(|row| {
            let mut fps_locked = false;
            let mut position = None;
            let mut name = None;
            let mut vehicle = None;
            let mut reason = None;
            let mut time = None;
            let mut best_lap = None;
            let mut lap = None;
            let mut fps = None;
            let mut ping = None;

            for (td, field) in row.find(Name("td")).zip(&fields) {
                let cell = td
                    .first_child()
                    .and_then(|text| text.as_text())
                    .unwrap_or("")
                    .trim();

                match field {
                    Field::FpsLocked => {
                        fps_locked = cell == "*";
                    }
                    Field::Pos => {
                        position = Some(scan_fmt!(cell, "{}", i32)?);
                    }
                    Field::Name => {
                        name = Some(cell.to_string());
                    }
                    Field::Vehicle => {
                        vehicle = Some(cell.to_string());
                    }
                    Field::Time => match cell {
                        "DNS" => {
                            reason = Some(Reason::Dns);
                        }
                        "DNF" => {
                            reason = Some(Reason::Dnf);
                        }
                        "DSQ" => {
                            reason = Some(Reason::Dsq);
                        }
                        "" => {}
                        time_str if time_str.starts_with("+") => {
                            let offset = parse_time(&time_str[1..])?;
                            time = Some(leader_time + offset);
                        }
                        time_str => {
                            leader_time = parse_time(time_str)?;
                            time = Some(leader_time);
                        }
                    },
                    Field::BestLap => {
                        if cell != "" {
                            best_lap = Some(parse_time(cell)?);
                        }
                    }
                    Field::Lap => {
                        if cell != "-" {
                            lap = Some(scan_fmt!(cell, "{}", i32)?);
                        }
                    }
                    Field::Ping => {
                        ping = Some(scan_fmt!(cell, "{}", i32)?);
                    }
                    Field::Fps => {
                        fps = Some(scan_fmt!(cell, "{}", i32)?);
                    }
                    Field::Ignored => {}
                }
            }
            Ok(Entrant {
                position: position.context("position required")?,
                name: name.context("entrant name required")?,
                vehicle: vehicle.context("vehicle name required")?,
                time,
                best_lap,
                lap,
                reason,
                ping,
                fps,
                fps_locked,
            })
        })
        .collect::<Result<_, _>>()?;

    Ok(race)
}

fn parse_time(time_str: &str) -> anyhow::Result<Duration> {
    if let Ok((minutes, seconds, millis)) = scan_fmt!(time_str, "{}:{}.{}", u64, u64, u64) {
        Ok(Duration::from_millis(
            millis + 1000 * (seconds + 60 * minutes),
        ))
    } else if let Ok((hours, minutes, seconds, millis)) =
        scan_fmt!(time_str, "{}:{}:{}.{}", u64, u64, u64, u64)
    {
        Ok(Duration::from_millis(
            millis + 1000 * (seconds + 60 * (minutes + 60 * hours)),
        ))
    } else {
        bail!("unrecognized time string {:?}", time_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_26741() {
        let race = parse_race(include_str!("samples/26741.html")).unwrap();
        let expected = include!("samples/26741.rs");
        assert_eq!(race, expected)
    }

    #[test]
    fn parse_events() {
        use std::io::Read;

        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
            &include_bytes!("samples/EventResult.tar.gz")[..],
        ));

        let mut contents = String::new();
        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            if !entry.header().entry_type().is_file() {
                continue;
            }

            eprintln!("{:?}", entry.path().unwrap());
            contents.clear();
            entry.read_to_string(&mut contents).unwrap();
            let _parsed = parse_race(&contents).unwrap();
        }
    }
}
