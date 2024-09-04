use rocket::{Build, Rocket};

use super::{catchers, Catcher};

impl Catcher for Rocket<Build> {
    fn mount_catchers(self) -> Self {
        self.register("/", catchers![catchers::not_found])
    }
}
