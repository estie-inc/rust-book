use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schedule {
    // 予定のID
    id: u64,
    // 勉強会の名前
    subject: String,
    // 開始時刻
    start: NaiveDateTime,
    // 終了時刻
    end: NaiveDateTime,
}
impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Calendar {
    schedules: Vec<Schedule>,
}

const SCHEDULE_FILE: &str = "schedule.json";

#[derive(Parser)]
#[clap()]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 予定の一覧表示
    List,
    /// 予定の追加
    Add {
        /// タイトル
        #[clap()]
        subject: String,
        /// 開始時刻
        #[clap()]
        start: NaiveDateTime,
        /// 終了時刻
        #[clap()]
        end: NaiveDateTime,
    },
    /// 予定の削除
    Delete {
        /// 予定のID
        #[clap()]
        id: u64,
    },
}

fn main() {
    let options = Cli::parse();

    match options.command {
        Commands::List => {
            let calendar = read_calendar();
            show_list(&calendar);
        }
        Commands::Add {
            subject,
            start,
            end,
        } => {
            let mut calendar = read_calendar();
            if add_schedule(&mut calendar, subject, start, end) {
                save_calendar(&calendar);
                println!("予定を追加しました。");
            } else {
                println!("エラー：予定が重複しています");
            }
        }
        Commands::Delete { id } => {
            let mut calendar = read_calendar();
            if delete_schedule(&mut calendar, id) {
                save_calendar(&calendar);
                println!("予定を削除しました。");
            } else {
                println!("エラー：IDが不正です");
            }
        }
    }
}

fn read_calendar() -> Calendar {
    let file = File::open(SCHEDULE_FILE).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn save_calendar(calendar: &Calendar) {
    let file = File::create(SCHEDULE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, calendar).unwrap();
}

fn show_list(calendar: &Calendar) {
    // 予定の表示
    println!("ID\tSTART\tEND\tSUBJECT");
    for schedule in &calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        );
    }
}

pub fn add_schedule(
    calendar: &mut Calendar,
    subject: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> bool {
    // 予定の作成
    let id = if calendar.schedules.is_empty() {
        0
    } else {
        calendar.schedules.last().unwrap().id + 1
    };
    let new_schedule = Schedule {
        id,
        subject,
        start,
        end,
    };

    // 予定の重複判定
    for schedule in &calendar.schedules {
        if schedule.intersects(&new_schedule) {
            return false;
        }
    }

    // 予定の追加
    calendar.schedules.push(new_schedule);

    true
}

pub fn delete_schedule(calendar: &mut Calendar, id: u64) -> bool {
    // 予定の削除
    for i in 0..calendar.schedules.len() {
        if calendar.schedules[i].id == id {
            calendar.schedules.remove(i);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    use rstest::rstest;

    #[rstest]
    #[case(18, 15, 18, 45, false)]
    #[case(18, 15, 19, 45, true)]
    #[case(18, 15, 20, 45, true)]
    #[case(19, 15, 19, 45, true)]
    #[case(19, 15, 20, 45, true)]
    #[case(20, 15, 20, 45, false)]
    fn test_schedule_intersects(
        #[case] h0: u32,
        #[case] m0: u32,
        #[case] h1: u32,
        #[case] m1: u32,
        #[case] should_intersect: bool,
    ) {
        let schedule = Schedule {
            id: 0,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, h0, m0, 0),
            end: naive_date_time(2024, 1, 1, h1, m1, 0),
        };
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };
        assert_eq!(should_intersect, schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_add_schedule() {
        let mut calendar = Calendar {
            schedules: vec![Schedule {
                id: 0,
                subject: "テスト予定".to_string(),
                start: naive_date_time(2023, 11, 19, 11, 22, 33),
                end: naive_date_time(2023, 11, 19, 22, 33, 44),
            }],
        };
        add_schedule(
            &mut calendar,
            "テスト予定2".to_string(),
            naive_date_time(2023, 12, 8, 9, 0, 0),
            naive_date_time(2023, 12, 8, 10, 30, 0),
        );
        let expected = Calendar {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "テスト予定".to_string(),
                    start: naive_date_time(2023, 11, 19, 11, 22, 33),
                    end: naive_date_time(2023, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1,
                    subject: "テスト予定2".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 10, 30, 0),
                },
            ],
        };
        assert_eq!(expected, calendar);
    }

    #[test]
    fn test_delete_schedule() {
        let mut calendar = Calendar {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "テスト予定".to_string(),
                    start: naive_date_time(2023, 11, 19, 11, 22, 33),
                    end: naive_date_time(2023, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1,
                    subject: "テスト予定2".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "追加できる予定".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };
        // 試しに0番の予定を削除してみる
        delete_schedule(&mut calendar, 0);
        // 削除後はこうなるはず
        let expected = Calendar {
            schedules: vec![
                Schedule {
                    id: 1,
                    subject: "テスト予定2".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "追加できる予定".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };
        assert_eq!(expected, calendar);
        // 次にID = 1の予定を削除してみる
        delete_schedule(&mut calendar, 1);
        // 削除後はこうなるはず
        let expected = Calendar {
            schedules: vec![Schedule {
                id: 2,
                subject: "追加できる予定".to_string(),
                start: naive_date_time(2023, 12, 15, 10, 0, 0),
                end: naive_date_time(2023, 12, 15, 11, 00, 0),
            }],
        };
        assert_eq!(expected, calendar);
        // 最後にID = 2の予定を削除してみる
        assert!(delete_schedule(&mut calendar, 2));
        // 削除後はこうなるはず
        let expected = Calendar { schedules: vec![] };
        assert_eq!(expected, calendar);
    }

    #[test]
    fn test_add_after_delete() {
        let mut calendar = Calendar {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "テスト予定".to_string(),
                    start: naive_date_time(2023, 11, 19, 11, 22, 33),
                    end: naive_date_time(2023, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1,
                    subject: "テスト予定2".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "テスト予定3".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };
        // 0番の予定を削除して新しい予定を追加する
        delete_schedule(&mut calendar, 0);
        add_schedule(
            &mut calendar,
            "テスト予定4".to_string(),
            naive_date_time(2023, 12, 22, 10, 0, 0),
            naive_date_time(2023, 12, 22, 11, 00, 0),
        );
        let expected = Calendar {
            schedules: vec![
                Schedule {
                    id: 1,
                    subject: "テスト予定2".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "テスト予定3".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
                Schedule {
                    id: 3,
                    subject: "テスト予定4".to_string(),
                    start: naive_date_time(2023, 12, 22, 10, 0, 0),
                    end: naive_date_time(2023, 12, 22, 11, 00, 0),
                },
            ],
        };
        assert_eq!(expected, calendar);
    }
}
