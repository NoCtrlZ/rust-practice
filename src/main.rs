use actix_web::{server, App, HttpRequest, Responder};

fn hello(req:&HttpRequest)->impl Responder{
    let to =
    req.match_info().get("name").unwrap_or("World");
    format!("Hello{}!",to)
    }

    fn main(){
        server::new(||{
            App::new().resource("/",|r|r.f(hello)).resource("/{name}",|r|r.f(hello))
            })
            .bind("localhost:3000")
            .expect("Cannotbindtoport3000")
             .run();}
