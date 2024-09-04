use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::files)]
pub struct UploadedFile {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
}
