use socketioxide::extract::SocketRef;
use tracing::info;




pub fn on_connect(socket: SocketRef) {
    info!("Client connected: {}", socket.id);
}