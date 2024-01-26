#[derive(Debug)]
enum ServiceError {}

// Actor
// T is the connection type, e.g. TcpStream or UnixStream
// - TcpStream is a method of inter-process communication (IPC) using the network.
// - UnixStream is a method of IPC using Unix domain sockets, that knows it is local-only.
struct MyService<T: AsyncRead + AsyncWrite> {
    receiver: mpsc::Receiver<ServiceMessage>,
    connection: T,
}

enum ServiceMessage {
    Ping,
    Pong,
}

// Sender interface
// Can be cloned for shared access
#[derive(Clone)]
pub struct MyServiceHandler {
    sender: mpsc::Sender<ServiceMessage>,
}
