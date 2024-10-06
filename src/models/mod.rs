use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::files)]
pub struct UploadedFile {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub private: bool,
}
