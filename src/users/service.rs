use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::users::models::{User, NewUser};
use diesel::result::Error;
use diesel::prelude::*;
use chrono::Utc;

pub async fn get_user_by_email(conn: &mut PgConnection, user_email: &str) -> Result<User, Error> {
    users
        .filter(email.eq(user_email))
        .first::<User>(conn)
}

pub async fn create_user(conn: &mut PgConnection, new_user: NewUser) -> Result<User, Error> {
    let now = Utc::now().naive_utc();

    // Insert and return the created user
    diesel::insert_into(users::table)
        .values((
            &new_user,
            created_at.eq(now),
            updated_at.eq(now),
        ))
        .get_result(conn)
}
