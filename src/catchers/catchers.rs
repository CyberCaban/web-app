#[catch(404)]
pub fn not_found(req: &rocket::Request) -> String {
    let uri = req.uri();
    format!("Not Found at {}", uri)
}