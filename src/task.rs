use crate::period::*;

/// A task to do
#[derive(Debug)]
pub struct Task {
    /// ID of the task
    pub id: TaskId,
    /// Subject of the task
    pub subject: String,
    /// Type of a task, for helpful tracking
    pub task_type: TaskType,
    /// Tags of the task
    pub tags: Vec<String>,
    /// How is this task to be repeated
    pub repeat_mode: RepeatMode,
    /// Targets of the task, for streaks tracking
    pub targets: Option<Vec<Streak>>,
    /// Due date of the task, when should this task be done
    pub due: Option<Date>,
    /// Defered date of the task, when should this task be started
    pub defered: Option<Date>,
}

/// A task type, used for improved tracking
#[derive(Debug)]
pub enum TaskType {
    /// A free task, that can be either done or not
    Free,
    /// A task that can be counted
    Counted {
        /// Number of time to do the task
        target: u32,
        /// Task to do
        task: String,
    },
}

/// How is a task to be repeated
#[derive(Debug, Clone, Copy)]
pub enum RepeatMode {
    /// It should be done only once
    Once,
    /// It should be repeated every specified period
    RepeatAfter {
        /// Period to use to repeat the task
        period: Period,
    },
    /// It should be repeated every specified period, and previous task occurence should be failed
    RepeatAfterAndFail {
        /// Period to use to repeat the task
        period: Period,
    },
    /// It should be repeated every specified period, based on the completion date
    RepeatAfterCompletion {
        /// Period to use to repeat the task
        period: Period,
    },
}

/// A streak of tasks done
#[derive(Debug, Copy, Clone)]
pub struct Streak {
    /// Period over which to check for this streak
    pub period: Period,
    /// Type of streak to check
    pub streak_type: StreakType,
}

/// How streak is computed
#[derive(Debug, Copy, Clone)]
pub enum StreakType {
    /// Counting tasks done during period
    TaskCount {
        /// Target number of tasks done during period
        target: u8,
    },
    /// Summing counts of tasks done during period
    CountedSum {
        /// Target sum of counted tasks done during period
        target: u32,
    },
}

/// Placeholder for a task ID
pub type TaskId = String;

/// Placeholder for a date
pub type Date = String;
