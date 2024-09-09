use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fs::NamedFile;
use rocket::tokio::io::AsyncReadExt;
use rocket::{form::Form, fs::TempFile, http::CookieJar, serde::json::Json, State};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::errors::LoginError;
use crate::schema::users::dsl::*;
use crate::{database::Connection, errors::ApiError, models::User};

#[derive(FromForm, Debug)]
pub struct UploadRequest<'r> {
    pub file: TempFile<'r>,
    pub filename: &'r str,
    pub is_private: bool,
}

#[post("/file/create", data = "<form>")]
pub async fn api_upload_file(
    form: Form<UploadRequest<'_>>,
    db: &State<Connection>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Value>, Value> {
    use crate::models::UploadedFile as File;
    use crate::schema::{files, users};

    if form.file.content_type().is_none() {
        return Err(ApiError::new("InvalidFileType", "Invalid file type").to_json());
    }
    let uploader_id = cookies.get("token");
    if uploader_id.is_none() {
        return Err(ApiError::new("Unauthorized", "Unauthorized").to_json());
    }
    let uploader_id = Uuid::parse_str(uploader_id.unwrap().value_trimmed()).unwrap();

    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return Err(ApiError::from_error(&e).to_json()),
    };
    match users
        .filter(users::id.eq(uploader_id))
        .first::<User>(&mut *conn)
    {
        Err(_) => return Err(ApiError::new("UserNotFound", LoginError::UserNotFound).to_json()),
        Ok(_) => (),
    }

    let old_name = match form.filename {
        "" => "unnamed",
        name => name,
    };
    let file_ext = match form.file.content_type() {
        None => "",
        Some(mime) => {
            let ext = mime.extension();
            match ext {
                None => "",
                Some(ext) => ext.as_str(),
            }
        }
    };
    let file_id = Uuid::new_v4();
    let file_name = format!("{}-{}.{}", file_id, old_name, file_ext);

    let new_file = File {
        id: uuid::Uuid::new_v4(),
        name: file_name.clone(),
        user_id: uploader_id,
        private: form.is_private,
    };

    if let Err(e) = diesel::insert_into(files::table)
        .values(&new_file)
        .execute(&mut *conn)
    {
        return Err(ApiError::from_error(&e).to_json());
    }

    let mut file = form.file.open().await.unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    let file_path = if form.is_private {
        format!("tmp/{}/{}", new_file.user_id, file_name)
    } else {
        format!("tmp/{}", file_name)
    };
    if new_file.private {
        std::fs::create_dir_all(format!("tmp/{}", new_file.user_id)).unwrap();
    }
    std::fs::write(file_path, buf).unwrap();

    Ok(Json("File uploaded".into()))
}

#[get("/file/<file_name>")]
pub async fn api_get_file(
    file_name: &str,
    db: &State<Connection>,
    cookies: &CookieJar<'_>,
) -> Option<NamedFile> {
    use crate::models::UploadedFile as File;
    use crate::schema::files::dsl::*;

    let uploader_id = cookies.get("token");
    if uploader_id.is_none() {
        let path = std::path::PathBuf::from("tmp").join(file_name);
        return NamedFile::open(path).await.ok();
    }
    let uploader_id = Uuid::parse_str(uploader_id.unwrap().value_trimmed()).unwrap();
    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return None;
        },
    };
    
    match files
    .filter(name.eq(file_name))
    .filter(user_id.eq(uploader_id))
    .first::<File>(&mut *conn)
    {
        Ok(f) => {
            let path = if f.private {
                format!("tmp/{}/{}", uploader_id, file_name)
            } else {
                format!("tmp/{}", file_name)
            };
            NamedFile::open(path).await.ok()
        },
        Err(e) => {
            println!("Uploader ID: {}", uploader_id);
            println!("File name: {}", file_name);
            eprintln!("Failed to get file: {}", e);
            None
        },
    }
}

#[delete("/file/<file_name>")]
pub fn api_delete_file(file_name: &str, db: &State<Connection>, cookies: &CookieJar<'_>) -> Value {
    use crate::models::UploadedFile as File;
    use crate::schema::{files as files_schema, files::dsl::files};

    let uploader_id = cookies.get("token");
    if uploader_id.is_none() {
        return ApiError::new("Unauthorized", "Unauthorized").to_json();
    }
    let uploader_id = Uuid::parse_str(uploader_id.unwrap().value_trimmed()).unwrap();

    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return ApiError::from_error(&e).to_json(),
    };
    match files
        .filter(files_schema::name.eq(file_name))
        .filter(files_schema::user_id.eq(uploader_id))
        .first::<File>(&mut *conn)
    {
        Ok(f) => {
            if let Err(e) = diesel::delete(files_schema::table.filter(files_schema::id.eq(f.id)))
                .execute(&mut *conn)
            {
                return ApiError::from_error(&e).to_json();
            }
             
            let path = if f.private {
                format!("tmp/{}/{}", uploader_id, file_name)
            } else {
                format!("tmp/{}", file_name)
            };
            std::fs::remove_file(path).unwrap();
            json!("File deleted")
        }
        Err(_) => json!("File not found"),
    }
}

#[get("/files")]
pub fn api_get_files(db: &State<Connection>, cookies: &CookieJar<'_>) -> Value {
    use crate::models::UploadedFile as File;
    use crate::schema::files as files_schema;
    let uploader_id = cookies.get("token");
    if uploader_id.is_none() {
        return ApiError::new("Unauthorized", "Unauthorized").to_json();
    }
    let uploader_id = Uuid::parse_str(uploader_id.unwrap().value_trimmed()).unwrap();
    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return ApiError::from_error(&e).to_json(),
    };
    match files_schema::table
        .filter(files_schema::user_id.eq(uploader_id))
        .load::<File>(&mut *conn)
    {
        Ok(files) => json!(files.into_iter().map(|f| f.name).collect::<Vec<String>>()),
        Err(e) => ApiError::from_error(&e).to_json(),
    }
}
