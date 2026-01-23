pub mod protocols;
pub mod transport_interface;
pub mod transport_server;
pub mod bind_transaport;
pub mod server_config;
pub mod protocol_factory;
pub mod server;
pub mod client_interface;
pub mod clients;
pub mod producers;
pub mod message;
pub mod registration_server;

pub use transport_server::TransportServer;
pub use transport_interface::MessageConsumer;
pub use protocols::TcpTransport;
pub use server_config::ServerConfig;
pub use server::Server;
pub use client_interface::Client;
pub use clients::TcpClient;
pub use producers::{
    Producer,
    ProducerPool,
    get_producer_pool,
    init_producer_pool,
    ClientFactory,
    add_tcp_producer,
    add_tcp_producer_with_timeout,
    forward_to_producer,
    forward_to_all_producers,
};

// Re-export bind_transport functions
pub use bind_transaport::{bind_transport, bind_transport_with_address, bind_transport_with_config};
pub use message::EventMessage;
pub use registration_server::{start_registration_server, start_registration_server_default};