use std::io;
use std::thread;
use tokio::net::{TcpStream, TcpListener};
use std::sync::Arc;
use tokio_codec::Framed;
use tokio::prelude::*;

use super::response::Response;
use super::request::Request;
use super::http::Http;
use super::codes;

use super::super::logic::cache_manager::CacheManager;


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

    #[inline]
    fn get_response(&self, request: &Request) -> Response {
        let mut response = Response::new();

        if request.method() != "PUT" {
            response.body_bytes(&[codes::RESPONSE_INVALID_REQUEST]);
            return response;
        }

        let path = request.path().as_ref();
        let request_body = request.raw_body();


        let data = match path {
            codes::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS => {
                if wrongRequestLength8(request) {
                    response.body_bytes(&[codes::RESPONSE_INVALID_REQUEST]);
                    return response;
                }
                let locationId: u32 = getFirstUInt(request_body);
                let categoryId: u32 = getSecondUInt(request_body);
                let data = CacheManager::get_todays_location_category_poll_rankings(locationId, categoryId);
                data
            },
            _ => {
                response.body_bytes(&[codes::RESPONSE_INVALID_REQUEST]);
                return response;
            }
        };

        response.body_vec(data);

        response
    }

    #[inline]
    fn wrongRequestLength8(requestBody: &[u8]) -> boolean {
        requestBody.len() != 8
    }

    #[inline]
    fn getFirstUInt(requestBody: &[u8]) -> u32 {
        let mut firstUInt: u32 = requestBody[0] << 24 as u32;
        firstUInt = firstUInt + requestBody[1] << 16;
        firstUInt = firstUInt + requestBody[2] << 8;
        firstUInt = firstUInt + requestBody[3];
    }

    #[inline]
    fn getSecondUInt(requestBody: &[u8]) -> u32 {
        let mut firstUInt: u32 = requestBody[4] << 24 as u32;
        firstUInt = firstUInt + requestBody[5] << 16;
        firstUInt = firstUInt + requestBody[6] << 8;
        firstUInt = firstUInt + requestBody[7];
    }


    /// Resolves a request, returning a future that is processable into a Response

    fn resolve(&self, mut request: Request) -> impl Future<Item=Response, Error=io::Error> + Send {
        let response = self.get_response(&request);
//        request.set_params(matched_route.params);

//        let context = (self.context_generator)(request);
//        let return_value = Box::new(future::ok(context));

//        return_value
//            .and_then(|context| {
//
                future::ok(response)
//            })
    }
}