use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

pub const LEDS: usize = 226;

pub struct UstripeSource {
    sock: UdpSocket,
    dest: SocketAddr,
    priority: u8
}

impl UstripeSource {
    pub fn new<A: ToSocketAddrs>(dest: A, priority: u8) -> UstripeSource {
        let mut dests = dest.to_socket_addrs().unwrap();
        match dests.next() {
            Some(dest) =>
                UstripeSource {
                    sock: UdpSocket::bind("0.0.0.0:0").unwrap(),
                    dest: dest,
                    priority: priority
                },
            None => panic!("No address")
        }
    }
    
    pub fn send(&self, pixels: &[[u8; 3]]) {
        let mut pkt = Vec::with_capacity(4 + pixels.len());
        // Header
        pkt.append(&mut vec![
            self.priority,
            0,  // CMD_SET_PIXEL_COLORS
            0x02, 0xA6  // Pixel data length
        ]);

        // Copy pixel data
        for pixel in pixels {
            pkt.extend(pixel);
        }
        
        // Send
        self.sock.send_to(&pkt[..], self.dest).unwrap();
    }
}
