mod mount_routes;
mod routes;
mod file_routes;
mod ws_routes;
pub trait AuthorizationRoutes {
    fn mount_auth_routes(self) -> Self;
    fn mount_static_files(self) -> Self;
    fn manage_db(self) -> Self;
    fn manage_ws_users(self) -> Self;
}
