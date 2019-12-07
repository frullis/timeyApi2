use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::schema::time_activity;

extern crate chrono;
use self::chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use rocket_contrib::json::JsonValue;

#[derive(Debug, Queryable, Serialize)]
//#[table_name = "time_activity"]
pub struct TimeActivity {
    pub id: i32,
    pub time_start: String,
    pub duration:Option<i32>,
    pub time_stop: Option<String>,
    pub project_id: Option<i32>,
    pub agent: Option<String>
    //pub time_local: String,
    //pub time_now: String
}

//Second struct so we can fetch some custom data from other query. Yes this seem retarded..
#[derive(Queryable)]
//#[table_name = "time_activity"]
pub struct Activity {
    pub time_start: String,
    pub time_now: String
}


#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "time_activity"]
pub struct NewTime {
    pub time_start: String,
    pub duration: i32,
    pub agent: Option<String>,
    pub project_id: Option<i32>
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "time_activity"]
pub struct JsonNewTime {
    pub project_id: Option<i32>,
    pub agent: Option<String>
}

#[derive(AsChangeset)]
#[table_name = "time_activity"]
pub struct UpdateTime {
    pub time_stop: String,
    pub duration: i32

}

#[derive(FromForm, Debug)]
pub struct FindDate {
    start_date: Option<String>,
    end_date: Option<String>,
    group: Option<String>
}


impl TimeActivity {


    pub fn start(conn: &MysqlConnection, time: JsonNewTime) -> TimeActivity
    {
        let utc_now = Utc::now();

        println!("Error: {:?}", time);

        let time_insert = NewTime { 
            time_start: utc_now.format("%Y-%m-%d %H:%M:%S").to_string(), 
            duration: 0, 
            agent: time.agent, 
            project_id: time.project_id};
                    
        
        diesel::insert_into(time_activity::table)
            .values(&time_insert)
            .execute(conn)
            .expect("Error creating new hero");

        time_activity::table.order(time_activity::id.desc()).first(conn).unwrap()



    }


    pub fn stop(conn: &MysqlConnection, id:i32) -> TimeActivity
    {
        let utc_now = Utc::now().naive_utc(); //Utc::now();
        let x: TimeActivity = time_activity::table.find(id).first(conn).unwrap();


        let test = NaiveDateTime::parse_from_str(&format!("{}",x.time_start), "%Y-%m-%d %H:%M:%S").unwrap(); //NaiveDateTime::parse_from_str(&x.time_start,"%Y-%m-%d %H:%M:%S").unwrap();

        let duration = (utc_now-test).to_std().unwrap().as_secs() as i32;

        //println!("{:?}", b);

        //println!("{:?} {}", NaiveDateTime::parse_from_str(&x.time_start,"%Y-%m-%d %H:%M:%S").unwrap());


        let time_update = UpdateTime { duration: duration, time_stop: utc_now.format("%Y-%m-%d %H:%M:%S").to_string()};

        diesel::update(time_activity::table.find(id))
   //         .set(time_activity::columns::time_stop.eq(utc_now.format("%Y-%m-%d %H:%M:%S").to_string()))//,time_activity::colums::duration.eq(duration))
 //           .set(time_activity::colums::duration.eq(duration))
            .set(time_update)
            .execute(conn).is_ok();

            time_activity::table.find(id).first(conn).unwrap()


        //x

        //updated_row

    }

    pub fn get(conn: &MysqlConnection, start_date:String, end_date:String) -> Vec<TimeActivity>
    //pub fn get(conn: &MysqlConnection, params: FindDate) -> Vec<TimeActivity>
    {   


        let x2 = time_activity::table
            .filter(time_activity::time_start.gt(start_date.clone()))
            .filter(time_activity::time_start.lt(end_date.clone()));



            let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&x2);
            println!("{:?}", sql);
            let result = x2.load::<TimeActivity>(conn).unwrap();
 //           println!("{:?}", result);
            result

        //updated_row

    }   

    pub fn get_groupby(conn: &MysqlConnection, start_date:String, end_date:String, group:String) -> Vec<TimeActivity>
    {
        let mut mygroup =time_activity::project_id;
        if group == "project"
        {
            mygroup = time_activity::project_id;
        }

        let x2 = time_activity::table
                .filter(time_activity::time_start.gt(start_date))
                .filter(time_activity::time_start.lt(end_date))
                .group_by(time_activity::project_id);

        let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&x2);
        println!("{:?}", sql);
        let result = x2.load::<TimeActivity>(conn).unwrap();

        result

    }


    pub fn add(conn: &MysqlConnection, timeactivity: NewTime) -> TimeActivity
    {
            diesel::insert_into(time_activity::table)
            .values(&timeactivity)
            .execute(conn)
            .expect("Error creating new hero");
        
        time_activity::table.order(time_activity::id.desc()).first(conn).unwrap()

    }

    //pub fn list(conn: &MysqlConnection) -> Vec<TimeActivity>
    //{
    
        //select time_start as y,(select time_now from time_activity order by id desc limit 1) as x, TIMESTAMPDIFF(SECOND,y,x) from time_activity group by time_start;
        //diesel::select("select time_start as y,(select time_now from time_activity order by id desc limit 1) as x, TIMESTAMPDIFF(SECOND,y,x) from time_activity group by time_start;");
        //use diesel::dsl::sql_query;
        //let data:i64 = sql_query("select count(*) from time_activity").load::<TimeActivity>(conn).unwrap();

        //data
    
        
        //time_activity::table.order(time_activity::id.desc()).load::<TimeActivity>(conn).unwrap()

    //}


    pub fn delete(conn: &MysqlConnection, id: i32) -> usize
    {
        diesel::delete(time_activity::table.filter(time_activity::id.eq(&id))).execute(conn).unwrap()


    }





/*
        pub fn list2(conn: &MysqlConnection) -> JsonValue //Vec<TimeActivity>
        {
            use diesel::expression::sql_literal::sql;

            println!("what");
            let query:Vec<Activity> = sql("
                                select time_start ,time_now FROM time_activity WHERE time_now IN (select MAX(time_now) from time_activity group by time_start)")
    
                
           // let query:Vec<TimeActivity> = sql("select 'hej','hej','hej'")
                                .load(conn)
                //.first(conn)
                //.get_result(conn)
    .expect("Error executing raw SQL");

            //let b = query;
            for _x in query
            {
                let time1 = NaiveDateTime::parse_from_str(&_x.time_now, "%Y-%m-%d %H:%M:%S").unwrap();
                let time2 = NaiveDateTime::parse_from_str(&_x.time_start, "%Y-%m-%d %H:%M:%S").unwrap();
                let diff = (time1 - time2).to_std().unwrap().as_secs();
                println!("time_start{:#?}", _x.time_start);
                println!("time_now{:#?}", _x.time_now);
                println!("difference: {:?}", diff);
            }

                json!({
        "activity": "error",
        "reason": "Resource was not found."
    })


        }
*/


}
