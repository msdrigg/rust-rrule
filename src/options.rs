use chrono::prelude::*;

#[derive(Debug)]
pub struct YearInfo {
    pub yearlen: usize,
    pub nextyearlen: usize,
    pub yearordinal: isize,
    pub yearweekday: usize,
    pub mmask: Vec<usize>,
    pub mrange: Vec<usize>,
    pub mdaymask: Vec<isize>,
    pub nmdaymask: Vec<isize>,
    pub wdaymask: Vec<usize>,
    pub wnomask: Option<Vec<usize>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Frequenzy {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Hourly = 4,
    Minutely = 5,
    Secondly = 6,
}

#[derive(Debug, Clone)]
pub struct ParsedOptions {
    pub freq: Frequenzy,
    pub interval: usize,
    pub count: Option<u32>,
    pub until: Option<DateTime<Utc>>,
    pub tzid: Option<String>,
    pub dtstart: DateTime<Utc>,
    pub wkst: usize,
    pub bysetpos: Vec<isize>,
    pub bymonth: Vec<usize>,
    pub bymonthday: Vec<isize>,
    pub bynmonthday: Vec<isize>,
    pub byyearday: Vec<isize>,
    pub byweekno: Vec<isize>,
    pub byweekday: Vec<usize>,
    pub byhour: Vec<usize>,
    pub byminute: Vec<usize>,
    pub bysecond: Vec<usize>,
    pub bynweekday: Vec<Vec<isize>>,
    pub byeaster: Option<isize>,
}