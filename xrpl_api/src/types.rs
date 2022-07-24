pub trait Request {
    type Response;

    fn method(&self) -> String;
}
