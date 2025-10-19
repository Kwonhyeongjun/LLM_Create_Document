use crate::error::upgrade_error::UpgradeError;
use bytes::{Buf, BufMut, Bytes};
use httparse::Status;

pub struct UpgradeParser<B: BufMut + Buf> {
    buf: B,
}

pub trait Upgrade<B: BufMut + Buf> {
    const METHOD: &'static str;
    const PATH: &'static [&'static str];
    const VERSION: u8;
    const HEADER_REQUIRED: [(&'static str, Option<&'static str>); 6];
    const HEADER_OPTIONAL: [&'static str; 6];
    const WS_MAGIC: &'static str;

    fn new(buf: B) -> Self;
    fn accumulate(&mut self, chunk: &[u8]) -> Result<Option<Bytes>, UpgradeError>;
    fn check_header<'req>(
        req: &'req httparse::Request,
        name: &'req str,
        expected: Option<&'req str>,
    ) -> Result<&'req str, UpgradeError>;
    fn into_response<'h, 'req>(
        &self,
        req: httparse::Request<'h, 'req>,
        len: usize,
    ) -> Result<Bytes, UpgradeError>;
}

impl<B: BufMut + Buf> Upgrade<B> for UpgradeParser<B> {
    const METHOD: &'static str = "GET";
    const PATH: &'static [&'static str] = &["/ws"];
    const VERSION: u8 = 1;
    const HEADER_REQUIRED: [(&'static str, Option<&'static str>); 6] = [
        ("Host", None),
        ("Connection", Some("Upgrade")),
        ("Upgrade", Some("websocket")),
        ("Origin", None),
        ("Sec-WebSocket-Version", Some("13")),
        ("Sec-WebSocket-Key", None),
    ];
    const HEADER_OPTIONAL: [&'static str; 6] = [
        "Pragma",
        "Cache-Control",
        "User-Agent",
        "Accept-Encoding",
        "Accept-Language",
        "Sec-WebSocket-Extensions",
    ];
    const WS_MAGIC: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

    fn new(buf: B) -> Self {
        Self { buf }
    }

    fn accumulate(&mut self, chunk: &[u8]) -> Result<Option<Bytes>, UpgradeError> {
        self.buf.put_slice(chunk);
        let mut headers = [httparse::EMPTY_HEADER; 12];
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(self.buf.chunk());
        match res {
            Ok(Status::Complete(n)) => match self.into_response(req, n) {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            },
            Ok(Status::Partial) => Ok(None),
            Err(e) => Err(UpgradeError::ParseError(e).log(None)),
        }
    }

    fn check_header<'req>(
        req: &'req httparse::Request,
        name: &'req str,
        expected: Option<&'req str>,
    ) -> Result<&'req str, UpgradeError> {
        let value = req
            .headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| std::str::from_utf8(h.value).unwrap_or(""))
            .ok_or_else(|| UpgradeError::MissingHeader.log(Some(name)))?;

        if let Some(expected) = expected {
            if !value.eq_ignore_ascii_case(expected) {
                return Err(UpgradeError::InvalidValue.log(Some(name)));
            }
        }
        Ok(value)
    }

    fn into_response<'h, 'req>(
        &self,
        req: httparse::Request<'h, 'req>,
        len: usize,
    ) -> Result<Bytes, UpgradeError> {
        // 1. Method
        if req.method != Some(Self::METHOD) {
            return Err(UpgradeError::InvalidMethod.log(None));
        }

        // 2. Path
        if let Some(path) = req.path {
            if !Self::PATH.contains(&path) {
                return Err(UpgradeError::InvalidPath.log(Some(path)));
            }
        } else {
            return Err(UpgradeError::InvalidPath.log(None));
        }

        // 3. Version
        if req.version != Some(Self::VERSION) {
            return Err(UpgradeError::InvalidVersion.log(None));
        }

        // 4. Headers
        for (name, expected) in Self::HEADER_REQUIRED.iter() {
            Self::check_header(&req, name, *expected)?;
        }

        // 5. Body
        if self.buf.chunk().len() > len {
            return Err(UpgradeError::InvalidValue.log(Some("Body not allowed")));
        }

        // 6. Additional Header (Optional)

        // 7. Validate Sec-WebSocket-Key
        let key_header = Self::check_header(&req, "Sec-WebSocket-Key", None)?;

        use base64::{engine::general_purpose, Engine as _};
        use sha1::{Digest, Sha1};

        let mut sha1 = Sha1::new();
        sha1.update(key_header.as_bytes());
        sha1.update(Self::WS_MAGIC.as_bytes());

        let hashed = sha1.finalize();
        let accept_key = general_purpose::STANDARD.encode(hashed);

        // 8. Construct Response
        let mut buf = bytes::BytesMut::with_capacity(128);
        buf.extend_from_slice(b"HTTP/1.1 101 Switching Protocols\r\n");
        buf.extend_from_slice(b"Upgrade: websocket\r\n");
        buf.extend_from_slice(b"Connection: Upgrade\r\n");
        buf.extend_from_slice(b"Sec-WebSocket-Accept:");
        buf.extend_from_slice(accept_key.as_bytes());
        buf.extend_from_slice(b"\r\n\r\n");

        Ok(buf.freeze())
    }
}
