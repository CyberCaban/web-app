mod mount_catchers;
mod catchers;

pub trait Catcher {
    fn mount_catchers(self) -> Self;
}
