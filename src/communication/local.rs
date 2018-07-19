use std::fs::remove_file;
use std::path::Path;

use tokio::prelude::*;
use tokio_core::reactor::Handle;
use tokio_uds::{UnixListener, UnixStream};

use settings::Settings;

/// Create a new unix listener.
/// In case a socket already exists it will be removed
pub fn get_unix_listener(settings: &Settings, handle: &Handle) -> UnixListener {
    let socket_path = get_socket_path(&settings);

    // Remove old socket
    if Path::new(&socket_path).exists() {
        remove_file(&socket_path).expect("Failed to remove old socket.");
        println!("Remove old socket.");
    }

    println!("Creating socket at {}", socket_path);

    UnixListener::bind(socket_path, handle).expect("Failed to create unix socket.")
}

/// Create a new unix stream.
/// This is used by clients and connects to the local daemon server socket.
pub fn get_unix_stream(settings: &Settings, handle: &Handle) -> UnixStream {
    let socket_path = get_socket_path(settings);
    println!("Connecting to socket at {}", socket_path);

    UnixStream::connect(&socket_path, handle).expect("Failed to connect to socket. Is the daemon running?")
}

/// Helper function to create the socket path used by clients and the daemon.
pub fn get_socket_path(settings: &Settings) -> String {
    let path = Path::new(settings.common.local_socket_dir.as_str())
        .join(format!("pueue_{}.sock", settings.common.group_id));

    path.as_path()
        .to_str()
        .expect("Unable to create socket path.")
        .to_string()
}