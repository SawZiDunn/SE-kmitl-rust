// the folder containing mod.rs
mod library;

// use library::{books::Book, media::AudioBook, LibraryItem};

fn main() {
    let mut book = library::Book::new("The Rust Programming Language");
    let mut audiobook = AudioBook::new("Rust for Beginners");

    print_item_status(&book);
    book.check_out();
    print_item_status(&book);

    print_item_status(&audiobook);
    audiobook.check_out();
    print_item_status(&audiobook);
}

// &dyn LibraryItem is a trait-object.

// It's like we are saying that types like Book and AudoBook that implements
// LibraryItem traits can be accepted

// We can pass both types to the function.

// dyn stands for dynamic dispatch meaning Rust will dynamically decide at runtime which
// method implementation to call, based on the actual type of the value.

// Rust generally favors static dispatch, where method calls are determined at compile-time.
// However, in situations like trait objects, dynamic dispatch is used.

// If we implement using a generic type like this, this will trigger static dispatch which means method calls are determined
// at compile-time rather than run time

// fn print_item_status<T: LibraryItem>(item: &T) {
fn print_item_status(item: &dyn LibraryItem) {
    println!(
        "{} is {}",
        item.title(),
        if item.is_available() {
            "available"
        } else {
            "not available"
        }
    );
}
