use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::schema::users;
//use crate::models::Projects;
extern crate chrono;
use self::chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};
use argon2::{self, Config};

#[derive(Debug, Queryable, Serialize,Deserialize)]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Insertable,Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime

}

//Json POST
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonUser {
    pub email: String,
    pub password: String
}

impl Users {
    pub fn get(conn: &MysqlConnection, user_id:i32) -> Users
    {
        users::table.find(user_id).first(conn).unwrap()

        //users::table.order(users::id.desc()).load::<Users>(conn).unwrap()


    }

    pub fn add(conn: &MysqlConnection, mut user: JsonUser) -> Option<Users>
    {


        let user_exist:i64 = users::table.filter(users::email.eq(user.email.clone())).count().first(conn).unwrap();

        println!("user_count: {:?}", user_exist);


        if user_exist < 1
        {


            let password = b"password";
            let salt = b"randomsalt";
            let config = Config::default();
            let hash = argon2::hash_encoded(password, salt, &config).unwrap();
            let matches = argon2::verify_encoded(&hash, password).unwrap();
            println!("{:?}", hash);

            user.password = argon2::hash_encoded(user.password.as_bytes(), salt, &config).unwrap();
            //user.created_at = Utc::now().naive_utc();
            //user.updated_at = Utc::now().naive_utc();
            let newuser = NewUser { email: user.email, password: user.password, created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc()};

            //println!("{:?}", user);
            diesel::insert_into(users::table)
                .values(&newuser)
                .execute(conn)
                .expect("Error creating new user");

        Some(users::table.order(users::id.desc()).first(conn).unwrap())
        
        } else {
            //User Exist
            None
        }


    }
    pub fn auth(conn: &MysqlConnection, mut user: JsonUser) -> Option<Users>
    {

        let res: QueryResult<Users> = users::table
            .filter(users::email.eq(user.email))
            .order(users::id)
            .first(conn);

        match res {
            Ok(res2) => {

               // Some(res)
                //println!("{:?}", Some(res))
                
                let verify:bool = argon2::verify_encoded(&res2.password, user.password.as_bytes()).unwrap();
                if verify == true {
                    Some(res2)
                } else {
                    None
                } 
        
            },Err(_) => {
                None
        }
        }


            //let verify:bool = argon2::verify_encoded(&res.password, user.password.as_bytes()).unwrap();

        //println!("{:?}", verify);

        //x


    }

}
