pub trait Server {
    fn listen(&self, host: String); 
}