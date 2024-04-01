// issue: https://www.reddit.com/r/rust/comments/17djgjd/working_through_programming_rust_2nd_edition_got/
// For Chrono made breaking changes, this code needs ti be updated according to https://github.com/ProgrammingRust/examples/blob/master/actix-gcd/Cargo.toml

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    // 闭包声明新App
    let server = HttpServer::new(|| App::new().route("/", web::get().to(get_index)));

    println!("Serving on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    // 原始字符串 r##
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}
