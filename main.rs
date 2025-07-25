use std::collections::HashMap;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("Serveur DNS  sur 127.0.0.1:8080");

    let mut dns_table = HashMap::new();
    dns_table.insert("rebecca.com", "93.184.116.34");
    dns_table.insert("rust-lang.org", "13.227.75.90");

    let mut buf = [0u8; 512];
    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        let request = String::from_utf8_lossy(&buf[..size]);
        println!("Reception de {}: {}", src, request);

        let response = match dns_table.get(request.trim()) {
            Some(ip) => format!("{} => {}", request.trim(), ip),
            None => format!("{} => Aucun resultat", request.trim()),
        };

        socket.send_to(response.as_bytes(), src)?;
    }
}