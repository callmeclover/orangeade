#[macro_use] extern crate rocket;
use rocket::http::ContentType;

mod libfunc;
use libfunc::*;


#[get("/plsgrab?<link>")]
async fn getter(link: &str) -> (ContentType, String) {
    // TODO: Find a way to make <link> able to be an absolute URL
    let source = ureq::get(&link.to_string()).call().expect("error occurred. check if this is a valid url.").into_string().ok().unwrap();
    (ContentType::HTML, replace_resources(&source, link).await.as_str().to_owned())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![getter])
}

/*
#[tokio::main]
async fn main() {
    let hostname: &str = "https://rust-lang.org";
    
    let request = ureq::get(hostname).call().expect("error occurred. check if this is a valid url.").into_string().ok();
    let source = &request.unwrap();

    // TODO: see that <link> and <script> in the source? we need to replace it's href (and src) to an absolute link (via resource injection and replacement)
    // see libfunc's replace_resources for this implementation
    let mod_source = replace_resources(source, hostname).await;
    println!("{}", mod_source)
}
 */