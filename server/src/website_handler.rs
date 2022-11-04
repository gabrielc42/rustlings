use super::server::Handler;
use super::http:: {Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Testing...1,2,3...</h1>".to_string()))
    }
}
