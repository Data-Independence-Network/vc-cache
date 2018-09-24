use std::thread;
use tokio::net::{TcpStream, TcpListener};
use std::sync::Arc;
use tokio_codec::Framed;
use tokio::prelude::*;


pub struct App<T: 'static + Context + Send> {
//    pub _route_parser: RouteParser<T>,
    ///
    /// Generate context is common to all `App`s. It's the function that's called upon receiving a request
    /// that translates an acutal `Request` struct to your custom Context type. It should be noted that
    /// the context_generator should be as fast as possible as this is called with every request, including
    /// 404s.
    pub context_generator: fn(Request) -> T
}

impl<T: Context + Send> App<T> {
///
/// Starts the app with a thread pool optimized for small requests and quick timeouts. This
/// is done internally by spawning a separate thread for each reactor core. This is valuable
/// if all server endpoints are similar in their load, as work is divided evenly among threads.
/// As seanmonstar points out though, this is a very specific use case and might not be useful
/// for everyday work loads.alloc
///
/// See the discussion here for more information:
///
/// https://users.rust-lang.org/t/getting-tokio-to-match-actix-web-performance/18659/7
///
pub fn start_small_load_optimized(mut app: App<T>, host: &str, port: u16) {
    let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();
    let mut threads = Vec::new();
    app._route_parser.optimize();
    let arc_app = Arc::new(app);

    for _ in 0..num_cpus::get() {
        let arc_app = arc_app.clone();
        threads.push(thread::spawn(move || {
            let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();

            let server = future::lazy(move || {
                let listener = {
                    let builder = TcpBuilder::new_v4().unwrap();
                    #[cfg(not(windows))]
                        builder.reuse_address(true).unwrap();
                    #[cfg(not(windows))]
                        builder.reuse_port(true).unwrap();
                    builder.bind(addr).unwrap();
                    builder.listen(2048).unwrap()
                };
                let listener = TcpListener::from_std(listener, &tokio::reactor::Handle::current()).unwrap();

                listener.incoming().for_each(move |socket| {
                    process(Arc::clone(&arc_app), socket);
                    Ok(())
                })
                    .map_err(|err| eprintln!("accept error = {:?}", err))
            });

            runtime.spawn(server);
            runtime.run().unwrap();
        }));
    }

    println!("Server running on {}", addr);

    for thread in threads {
        thread.join().unwrap();
    }

    fn process<T: Context + Send>(app: Arc<App<T>>, socket: TcpStream) {
        let framed = Framed::new(socket, Http);
        let (tx, rx) = framed.split();

        let task = tx.send_all(rx.and_then(move |request: Request| {
            app.resolve(request)
        }))
            .then(|_| future::ok(()));

        // Spawn the task that handles the connection.
        tokio::spawn(task);
    }
}

/// Resolves a request, returning a future that is processable into a Response

fn resolve(&self, mut request: Request) -> impl Future<Item=Response, Error=io::Error> + Send {
    let matched_route = self._req_to_matched_route(&request);
    request.set_params(matched_route.params);

    let context = (self.context_generator)(request);
    let middleware = matched_route.middleware;
    let middleware_chain = MiddlewareChain::new(middleware);

    let context_future = middleware_chain.next(context);

    context_future
        .and_then(|context| {
            future::ok(context.get_response())
        })
}
}