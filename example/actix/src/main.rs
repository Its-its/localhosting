// Copied from the src/command/test.rs file.
// Used just to show what specificially the program is used for.
// You can add website URLS to your 'cargo run' command to register specific domains.
// Otherwise it will use example.com, sub.example.com, other.example.com

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
	let args = std::env::args().skip(1);

	let addr = "127.0.0.1:8080";

	let mut websites = args.collect::<Vec<_>>();

	if websites.is_empty() {
		websites.push(String::from("example.com"));
		websites.push(String::from("sub.example.com"));
		websites.push(String::from("other.example.com"));
	}

	for website_name in &websites {
		println!("Registering Website URL: {}", website_name);
	}

	println!("Original address available to connect to also: {}", addr);

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