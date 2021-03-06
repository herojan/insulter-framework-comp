use actix_web::{App, HttpRequest, HttpResponse, Result, http::Method};

fn ok(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

fn created(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Created().header("Location", "/insults/1").finish())
}

fn updated(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}

fn insult(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!("Muppet")))
}

pub fn insult_mgmt() -> App {
    App::new()
        .prefix("/insults")
        .resource("", |r| {
            r.method(Method::GET).f(ok);
            r.method(Method::POST).f(created);
        })
        .resource("/{insult_id}", |r| {
            r.method(Method::GET).f(ok);
            r.method(Method::PUT).f(updated);
        })
}

pub fn insulter() -> App {
    App::new()
        .prefix("/insult")
        .resource("{name}", |r| {
            r.method(Method::GET).f(insult);
        })
}