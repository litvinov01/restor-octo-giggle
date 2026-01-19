pub trait Commands {
    fn ack(&self, msg: &str);
}