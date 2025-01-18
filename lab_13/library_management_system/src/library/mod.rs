//mod.rs is like the entry point for library module
// declaring sub-modules
pub mod books;
pub mod media;

pub trait LibraryItem {
    fn title(&self) -> &str;
    fn check_out(&mut self);
    fn check_in(&mut self);
    fn is_available(&self) -> bool;
}
