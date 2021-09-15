use actix_service::ServiceFactory;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, dev::{MessageBody, ServiceRequest, ServiceResponse}, guard, http::header, rt::System, web};
use anyhow::Result;

use crate::{Bridge, Connection, HostFile, HostItem, NetSH};

pub fn process(addr_or_host: &str, netsh: NetSH, hosts: HostFile) -> Result<()> {
	println!("Starting HTTP Server");

	// Manage Command "file.exe test 127.0.0.1:8080"
	let (host_items, bridge) = if addr_or_host.contains(':') {
		let connect_to = addr_or_host.parse::<Connection>()?;

		// Find Bridge from Connection Address and Port (ex: 127.0.0.1:8080)
		if let Some(bridge) = netsh.get_by_connection(connect_to) {
			let items = hosts.find_by_address(bridge.listen_to.address)
				.into_iter()
				.cloned()
				.collect::<Vec<_>>();

			(items, bridge.clone())
		} else {
			println!("Unable to find address.");
			return Ok(());
		}
	}

	// Manage Command "file.exe test example.com"
	else if let Some(host) = hosts.find_item_by_host(addr_or_host) {
		if let Some(bridge) = netsh.get_by_connection((host.address, 80).into()) {
			(vec![host.clone()], bridge.clone())
		} else {
			println!("Unable to find NetSH Bridge.");
			return Ok(());
		}
	} else {
		println!("Unable to find host.");
		return Ok(());
	};

	if host_items.is_empty() {
		println!("Unable to find Host(s).");
		return Ok(());
	}

	// Explain
	println!("Starting Webserver on Host(s)");
	println!("Using IP {}. Ensure it's not being used.", bridge.connect_to);
	for item in &host_items {
		println!("Listening on http://{}", item.host);
	}
	println!("You should now be able to use the Host URL to connect.");

	System::new("HTTP")
	.block_on(async {
		init(host_items, bridge).await
	})?;

	Ok(())
}

async fn init(host_items: Vec<HostItem>, bridge: Bridge) -> Result<()> {
	HttpServer::new(move || {
		let mut app = App::new();

		for item in &host_items {
			app = create_host_guard(app, item.host.clone());
		}

		app
	})
	.bind(bridge.connect_to.to_string())?
	.run()
	.await?;

	Ok(())
}

fn create_host_guard<A, B>(app: App<A, B>, host_url: String) -> App<A, B>
where
	B: MessageBody,
	A: ServiceFactory<
		Request = ServiceRequest,
		Response = ServiceResponse<B>,
		Error = actix_web::Error,
		InitError = (),
		Config = ()
	>
{
	app.service(
		web::scope("")
		.guard(guard::fn_guard(
			move |req| {
				(|| -> Option<bool> {
					let host = req.headers().get(header::HOST)?;
					Some(host == &host_url)
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
}