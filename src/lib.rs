#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

//! Library to experiment with ToDos

mod period;
pub use period::*;
mod task;
pub use task::*;

/// A done task occurence
#[derive(Debug)]
pub struct Done {
    /// ID of task that was done
    pub task_id: TaskId,
    /// Date of completion
    pub completed_on: Date,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_express_example_tasks() {
        // simple task
        let _simple_task = Task {
            id: String::from("task_id"),
            subject: String::from("simple task"),
            task_type: TaskType::Free,
            tags: vec![],
            repeat_mode: RepeatMode::Once,
            targets: None,
            due: None,
            defered: None,
        };

        // due task
        let _due_task = Task {
            id: String::from("task_id"),
            subject: String::from("due task"),
            task_type: TaskType::Free,
            tags: vec![],
            repeat_mode: RepeatMode::Once,
            defered: None,
            due: Some(String::from("some date")),
            targets: None,
        };

        // 100 push ups per day
        let _daily_push_ups = Task {
            id: String::from("task_id"),
            subject: String::from("let's do 100 push ups per day"),
            task_type: TaskType::Counted {
                target: 100,
                task: String::from("push ups"),
            },
            tags: vec![],
            repeat_mode: RepeatMode::RepeatAfterAndFail {
                period: Period::Daily,
            },
            targets: Some(vec![Streak {
                period: Period::Daily,
                streak_type: StreakType::TaskCount { target: 1 },
            }]),
            due: Some(String::from("today")),
            defered: None,
        };

        // once every two weeks, with at least one week between two occurences, repeat two weeks after last completion date
        let _bedsheets = Task {
            id: String::from("task_id"),
            subject: String::from("change the bedsheets"),
            task_type: TaskType::Free,
            tags: vec![],
            repeat_mode: RepeatMode::RepeatAfterCompletion {
                period: Period::Days { days: 14 },
            },
            due: Some(String::from("in two weeks")),
            defered: Some(String::from("in one week")),
            targets: None,
        };

        // three times per week, can miss one every two weeks
        let _workout = Task {
            id: String::from("task_id"),
            subject: String::from("workout"),
            task_type: TaskType::Free,
            tags: vec![],
            repeat_mode: RepeatMode::RepeatAfterCompletion {
                period: Period::OnDaysOfWeek {
                    days: DaysOfTheWeek {
                        monday: true,
                        wednesday: true,
                        friday: true,
                        ..Default::default()
                    },
                },
            },
            due: None,
            defered: None,
            targets: Some(vec![Streak {
                period: Period::Days { days: 14 },
                streak_type: StreakType::TaskCount { target: 5 },
            }]),
        };

        // 10000 per day, 400000 per month
        let _walkalot = Task {
            id: String::from("task_id"),
            subject: String::from("10000 steps per day, even more per month!"),
            task_type: TaskType::Counted {
                target: 10000,
                task: String::from("steps"),
            },
            tags: vec![],
            repeat_mode: RepeatMode::RepeatAfterAndFail {
                period: Period::Daily,
            },
            due: None,
            defered: None,
            targets: Some(vec![
                Streak {
                    period: Period::Daily,
                    streak_type: StreakType::TaskCount { target: 1 },
                },
                Streak {
                    period: Period::Monthly { day: 1 },
                    streak_type: StreakType::CountedSum { target: 400000 },
                },
            ]),
        };

        // every day, allow one missed day per week
        let _learn_spanish = Task {
            id: String::from("task_id"),
            subject: String::from("Learn spanish"),
            task_type: TaskType::Counted {
                target: 10,
                task: String::from("minutes"),
            },
            tags: vec![],
            repeat_mode: RepeatMode::RepeatAfterAndFail {
                period: Period::Daily,
            },
            due: None,
            defered: None,
            targets: Some(vec![Streak {
                period: Period::Weekly { day: Day::Monday },
                streak_type: StreakType::TaskCount { target: 6 },
            }]),
        };
    }
}
