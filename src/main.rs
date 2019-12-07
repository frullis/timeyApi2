#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

//extern crate chrono;
//use self::chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

//use rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};
use timeyAPI::db;
use timeyAPI::models::timeactivity::{TimeActivity, JsonNewTime};
use timeyAPI::models::projects::{Projects, JsonProject};
use timeyAPI::models::users::{Users, JsonUser};
use timeyAPI::error::Error as ApiError;


#[catch(400)]
fn badrequest_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "400 Bad Request"
    })  
}


#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}



#[post("/user/auth", data = "<userdata>", format = "json")]
fn user_auth(connection: db::Connection, userdata: Json<JsonUser>) -> Result<Json<Users>,ApiError> {
    let insert = JsonUser { ..userdata.into_inner() };
    match Users::auth(&connection, insert)
    {
        None => Err(ApiError::AuthError),
        Some(user) => Ok(Json(user))
    }
}

#[post("/user", data = "<userdata>", format = "json")]
fn user_add(connection: db::Connection, userdata: Json<JsonUser>) -> Result<Json<Users>,ApiError> {
    let insert = JsonUser { ..userdata.into_inner() };
    match Users::add(&connection, insert) {
        None => Err(ApiError::UserExist),
        Some(user) => Ok(Json(user))
    }
}

#[get("/user/<user_id>", format = "json")]
fn user_find(connection: db::Connection, user_id:i32) -> Json<Users> {

    Json(Users::get(&connection, user_id))

}


    //#[post("/activity", data = "<timeactivity>", format = "json")]
//fn create_activity(connection: db::Connection, timeactivity: Json<NewTime>) -> Json<TimeActivity> {
#[post("/activity/start", data = "<timedata>", format = "json")]
fn start_activity(connection: db::Connection, timedata: Json<JsonNewTime>) -> Json<TimeActivity> {    
    
    println!("XxX{:?}", timedata);
    let insert = JsonNewTime { ..timedata.into_inner() };
    Json(TimeActivity::start(&connection, insert))
}
#[put("/activity/stop/<id>", format = "json")]
fn stop_activity(connection: db::Connection, id:i32) -> Json<TimeActivity> {    
    //let insert = NewTime { ..timeactivity.into_inner() };
    Json(TimeActivity::stop(&connection, id))
}

#[delete("/activity/<id>", format = "json")]
fn delete_activity(connection: db::Connection, id:i32) -> JsonValue {

    if TimeActivity::delete(&connection, id) == 1
    {
        json!({
        "status": "ok",
        "message": "Timeactivity have been deleted"
    })

    } else {
    json!({
        "status": "error",
        "message": "no such timeactivity id"
    })


    }

}


use rocket::request::Form;

#[derive(FromForm, Debug)]
pub struct FindDate {
    start_date: Option<String>,
    end_date: Option<String>,
    group: Option<String>
}

#[get("/activity?<params..>", format = "json")]
fn list_activity(connection: db::Connection, params: Form<FindDate>) -> /*JsonValue {*/ Result<Json<Vec<TimeActivity>>, ApiError> {
        println!("found you:{:?}", Some(&params.start_date));
        //let mut start_date="".to_string();
        let mut start_date = String::from("");
        let mut end_date="".to_string();
        let mut group="".to_string();
        println!("{:?}", params);

        match &params.start_date {
            Some(x) => start_date=x.to_string(),
            None => { return Err(ApiError::ParamStartDateMissing) }
        }

        match &params.end_date {
            Some(x) => end_date=x.to_string(),
            None => { return Err(ApiError::ParamEndDateMissing) }
        }

        match &params.group {
            Some(x) => group=x.to_string(),
            None => {return Err(ApiError::ParamEndDateMissing)},
        }

        //println!("{:?}",)

        //let params = FindDate { start_date: start_date, end_date: end_date, group: Some(group)};

        if group == ""{
        Ok(Json(TimeActivity::get(&connection, start_date.to_string(), end_date.to_string())))
        } else {
            Ok(Json(TimeActivity::get_groupby(&connection, start_date.to_string(), end_date.to_string(), group.to_string() )))
}

}



#[get("/projects", format = "json")]
fn get_projects(connection: db::Connection) -> Json<Vec<Projects>> {

    Json(Projects::list(&connection))

}
#[post("/projects", data = "<projectdata>", format = "json")]
fn project_add(connection: db::Connection, projectdata: Json<JsonProject>) -> Result<Json<Projects>,ApiError> {
    match &projectdata.name {
    Some(x) => {},
    None => { return Err(ApiError::ProjectExist)}
    }

    let insert = JsonProject { ..projectdata.into_inner() };
    match Projects::add(&connection, insert) {
        None => Err(ApiError::ProjectExist),
        Some(project) => Ok(Json(project))
    }
}


//#[get("/config", format = "json")]
/*
#[get("/<name>/<age>", format = "json")]
fn hello2(name: String, age: u8) -> JsonValue {
            json!({
            "status": "error",
            "reason": "ID exists. Try put."
})
}*/




fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/1.0", routes![project_add, get_projects, delete_activity, start_activity, stop_activity, list_activity, user_add,user_auth, user_find])
        .register(catchers![not_found, badrequest_found])
        .launch();
    //rocket::ignite().mount("/bajs", routes![hello2]).register(catchers![not_found]).launch();
}

