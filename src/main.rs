extern crate iron;
extern crate mount;
extern crate staticfile;

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("resources/public/")));

    println!("Server running at http://localhost:3000/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
