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


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
	let mut args = std::env::args().skip(1);

	let port = args.next().unwrap_or_else(|| panic!("Expected a port to run the website on.\nPlease Use the following command: localhosting.exe 8080 example.com this.example.com"));
	let addr = format!("127.0.0.1:{}", port);

	let websites = args.collect::<Vec<_>>();

	if websites.is_empty() {
		panic!("Expected website urls.\nPlease Use the following command: localhosting.exe <port> example.com this.example.com")
	}

	for website_name in &websites {
		println!("Registering Website URL: {}", website_name);
	}


	HttpServer::new(move || {
		let mut app = App::new();

		for website_name in websites.clone() {
			app = app.service(
				web::scope("")
				.guard(guard::fn_guard(
					move |req| {
						(|| -> Option<bool> {
							Some(req.headers().get(header::HOST)? == &website_name)
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
			);
		}

		app.service(
			web::scope("")
			.route("*", web::get().to(|req: HttpRequest| {
				let host = req.headers()
					.get(header::HOST)
					.unwrap()
					.to_str()
					.unwrap();

				println!("Loaded {}", host);

				HttpResponse::Ok().body(format!("Viewing Default Host {:?}", host))
			}))
		)
	})
	.workers(1)
	.bind(addr)?
	.run()
	.await?;

	Ok(())
}