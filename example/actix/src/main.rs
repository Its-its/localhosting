// Untested. Copied from the src/command/test.rs file.
// Used just to show what specificially the program is used for.

use actix_web::{
	App,
	HttpRequest,
	HttpResponse,
	HttpServer,
	guard,
	http::header,
	web
};


const HOST_URL: &str = "example.com";
const HOST_URL_2: &str = "this.example.com";

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
	HttpServer::new(move || {
		App::new()

		// Host
		.service(
			web::scope("")
			.guard(guard::fn_guard(
				move |req| {
					(|| -> Option<bool> {
						let host = req.headers().get(header::HOST)?;
						Some(host == HOST_URL)
					})()
					.unwrap_or_default()
				}
			))
			.route("*", web::get().to(|req: HttpRequest| {
				let host = req.headers()
					.get(header::HOST)
					.unwrap()
					.to_str()
					.unwrap();

				println!("Loaded {}", host);

				HttpResponse::Ok().body(format!("Viewing Host {:?}", host))
			}))
		)

		// Host 2
		.service(
			web::scope("")
			.guard(guard::fn_guard(
				move |req| {
					(|| -> Option<bool> {
						let host = req.headers().get(header::HOST)?;
						Some(host == HOST_URL_2)
					})()
					.unwrap_or_default()
				}
			))
			.route("*", web::get().to(|req: HttpRequest| {
				let host = req.headers()
					.get(header::HOST)
					.unwrap()
					.to_str()
					.unwrap();

				println!("Loaded {}", host);

				HttpResponse::Ok().body(format!("Viewing Host {:?}", host))
			}))
		)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await?;

	Ok(())
}