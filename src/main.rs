#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    println!("wow");
    rocket::build().mount("/api/test", routes![hello_world])
}

#[get("/helloworld")]
async fn hello_world() -> String {
    return "Hello World....".to_string();
}
