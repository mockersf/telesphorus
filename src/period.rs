//! Types to describe a period

/// A period of time
#[derive(Debug, Copy, Clone)]
pub enum Period {
    /// A number of hours
    Hours {
        /// The number of hours
        hours: i8,
    },
    /// A number of days
    Days {
        /// THe number of days
        days: i8,
    },
    /// Given days in a week
    OnDaysOfWeek {
        /// Days of the week
        days: DaysOfTheWeek,
    },
    /// A period of one hour
    Hourly,
    /// A period of 24 hours
    Daily,
    /// A period of seven days
    Weekly {
        /// Day of the week to start on
        day: Day,
    },
    /// A period of one calendar month
    Monthly {
        /// Day of the month to start on
        day: u16,
    },
    /// A period of one calendar year
    Yearly {
        /// Day of the year to start on
        day: u16,
    },
}

/// Days of the week
#[derive(Debug, Copy, Clone, Default)]
pub struct DaysOfTheWeek {
    /// Monday
    pub monday: bool,
    /// Tuesday
    pub tuesday: bool,
    /// Wednesday
    pub wednesday: bool,
    /// Thursday
    pub thursday: bool,
    /// Friday
    pub friday: bool,
    /// Saturday
    pub saturday: bool,
    /// Sunday
    pub sunday: bool,
}

/// Day of the week
#[derive(Debug, Copy, Clone)]
pub enum Day {
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
    /// Sunday
    Sunday,
}
