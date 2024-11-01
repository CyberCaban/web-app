use rocket::{fs::FileServer, futures::lock::Mutex, route, Build, Rocket};
use crate::database::Connection as Conn;

use super::{file_routes, routes, ws_routes, AuthorizationRoutes};

impl AuthorizationRoutes for Rocket<Build> {
    fn mount_auth_routes(self) -> Self {
        self.mount("/api", routes![
            routes::api_hello,
            routes::api_hello_post,
            routes::api_get_users,
            routes::api_register,
            file_routes::api_upload_file,
            file_routes::api_get_file,
            file_routes::api_delete_file,
            file_routes::api_get_files,
            routes::api_login,
            routes::api_logout,
            routes::toro,
            ws_routes::routes::test_ws,
            ws_routes::routes::stream_ws,
            ws_routes::routes::watch_ws
        ])
    }

    fn mount_static_files(self) -> Self {
        self.mount("/", FileServer::from("www/dist"))
    }

    fn manage_db(self) -> Self {
        self.manage(Conn::new())
    }

    fn manage_ws_users(self) -> Self {
        self.manage(ws_routes::WSPeers::new())
    }
}