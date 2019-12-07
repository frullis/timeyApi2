use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::schema::projects;
//use crate::models::Projects;
extern crate chrono;
use self::chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};


#[derive(Debug, Queryable, Serialize,Deserialize)]
pub struct Projects {
    pub id: i32,
    pub name: Option<String>,
    pub created_at: Option<NaiveDateTime> 
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct JsonProject {
    pub name: Option<String>,
    //pub agent: Option<String>
}

#[derive(Debug, Insertable,Serialize, Deserialize)]
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub created_at: NaiveDateTime

}



impl Projects {
    pub fn list(conn: &MysqlConnection) -> Vec<Projects>
    {
            /*diesel::insert_into(projects::table)
            .values(&timeactivity)
            .execute(conn)
            .expect("Error creating new hero");
*/
//        projects::table.order(projects::id.desc()).first(conn).unwrap()
        projects::table.order(projects::id.desc()).load::<Projects>(conn).unwrap()


    }
    pub fn add(conn: &MysqlConnection, jsondata: JsonProject) -> Option<Projects>
    {
        
        let project_exist:i64 = projects::table.filter(projects::name.eq(jsondata.name.clone())).count().first(conn).unwrap();

        if project_exist < 1 {

        let insert_values = NewProject { name: jsondata.name.unwrap(), created_at: Utc::now().naive_utc() };

            diesel::insert_into(projects::table)
            .values(&insert_values)
            .execute(conn)
            .expect("Error creating new hero");

        Some(projects::table.order(projects::id.desc()).first(conn).unwrap())

        } else {
            
            None
        }

    }


}
