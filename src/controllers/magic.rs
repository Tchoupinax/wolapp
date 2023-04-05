use hyper::{Body, Request, Response};
use wake_on_lan;

#[derive(Debug, serde::Deserialize)]
struct MagicPacketRequest {
    address: String,
}

// 00:11:32:49:C2:FB
pub async fn magic_packet_handler(
    req: Request<Body>,
) -> Result<Response<Body>, routerify_json_response::Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let request: MagicPacketRequest = serde_json::from_slice(&body_bytes).unwrap();

    let address = request.address;

    let magic_packet = wake_on_lan::MagicPacket::new(&build_mac_address(address.into()));
    magic_packet.send()?;

    Ok(Response::new(Body::from("OK")))
}

pub fn build_mac_address(address: String) -> [u8; 6] {
    let mut bytes = [0; 6];
    let hex_strings: Vec<&str> = address.split(':').collect();

    for (i, hex_string) in hex_strings.iter().enumerate() {
        let byte = u8::from_str_radix(hex_string, 16).unwrap();
        bytes[i] = byte;
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_mac_address() {
        assert_eq!(
            build_mac_address("00:AA:53:29:C2:FB".into()),
            [0x00, 0xAA, 0x53, 0x29, 0xC2, 0xFB]
        );
    }
}
