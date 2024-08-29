
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Selectable, Insertable};

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
}