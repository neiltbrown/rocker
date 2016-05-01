pub struct Request {
    pub method: Method,
    pub path: String,
}

pub enum Method {
    Get,
    Post
}

impl Request {
    pub fn build(&self) -> String {
        match self.method {
            Method::Get => String::from("GET http:/containers/json HTTP/1.0\r\n\r\n"),
            Method::Post => String::from(""),
        }
        //return format!("GET http:/containers/json HTTP/1.0\r\n\r\n")
    }
}
