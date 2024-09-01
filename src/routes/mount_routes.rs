use rocket::{fs::FileServer, Build, Rocket};
use crate::database::Connection as Conn;

use super::{routes, AuthorizationRoutes};

impl AuthorizationRoutes for Rocket<Build> {
    fn mount_auth_routes(self) -> Self {
        self.mount("/api", routes![
            routes::api_hello,
            routes::api_hello_post,
            routes::api_get_users,
            routes::api_register,
            routes::api_upload_file,
            routes::toro
        ])
    }

    fn mount_static_files(self) -> Self {
        self.mount("/", FileServer::from("www/dist"))
    }

    fn manage_db(self) -> Self {
        self.manage(Conn::new())
    }
}