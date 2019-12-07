//extern crate chrono;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
//use self::chrono;
//use chrono::prelude::*;

use crate::schema::{posts, time_activity};
//use diesel::mysql::types::Datetime;
use diesel::sql_types::Timestamp;
//use self::chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

#[derive(Queryable,Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


#[derive(Queryable, Serialize)]
pub struct TimeActivity {
    pub id: i32,
    pub time_start: String,
    pub time_now: String
//pub time_now: Option<NaiveDateTime>,
//pub time_start: Option<NaiveDateTime>
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "time_activity"]
pub struct NewTime {
    //pub time_start: &'a str,
//    pub time_now: &'a str
//    pub time_now: Option<NaiveDateTime>,
//    pub time_start: Option<NaiveDateTime>

    pub time_now: String,
    pub time_start: String

}


#[derive(Queryable, Serialize,Deserialize)]
pub struct Projects {
    pub id: i32,
    pub name: Option<String>,
}


