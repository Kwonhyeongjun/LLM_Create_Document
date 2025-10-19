use bytes::{BufMut, BytesMut};

mod error; // UpgradeError 정의 있다고 가정
mod types; // UpgradeParser/Upgrade 트레잇 들어있는 파일

use error::upgrade_error;
use types::upgrade::{Upgrade, UpgradeParser};

fn main() {
    let buf = BytesMut::with_capacity(1024);
    let mut parser = UpgradeParser::new(buf);

    let req = fake_ws_request();

    match parser.accumulate(req) {
        Ok(Some(resp)) => {
            println!("--- Upgrade Response ---");
            println!("{}", String::from_utf8_lossy(&resp));
        }
        Ok(None) => {
            println!("Partial request, need more data");
        }
        Err(e) => {
            eprintln!("Upgrade failed: {:?}", e);
        }
    }
}

fn fake_ws_request() -> &'static [u8] {
    b"GET /ws HTTP/1.1\r\n\
    Host: localhost:8080\r\n\
    Connection: Upgrade\r\n\
    Upgrade: websocket\r\n\
    Origin: http://localhost\r\n\
    Sec-WebSocket-Version: 13\r\n\
    Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
    \r\n"
}
