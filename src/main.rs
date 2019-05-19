use actix_web::{fs, server, App, HttpRequest, Responder};

fn hello(req:&HttpRequest)->impl Responder{
    let to =
    req.match_info().get("name").unwrap_or("World");
    format!("Hello{}!",to)
}

fn main(){
    server::new(||{
        App::new().handler(
            "/",
            fs::StaticFiles::new(".").unwrap(),
        )
        })

        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}
