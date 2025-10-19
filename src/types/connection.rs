
struct SendBuf<B: Buf> {
    bufs: VecDeque<B>
}

impl<B: Buf> SendBuf<B> {
    fn new() -> Self {
        Self {
            bufs: VecDeque::new(),
        }
    }
    fn get_front(&self) -> Option<B> {
        if let Some(front) = self.bufs.front() {
            return Some(front.chunk());
        }
        None
    }
    fn advance_front(&self, sent: usize) { 
        let front_left = self
            .bufs
            .front()
            .expect("Cannot Advance Empty SendBuf")
            .remaining();

        debug_assert!(
            sent <= front_left,
            "advance_front: sent {} > remaining {}",
            sent,
            front_left
        );
        
        let front = self.bufs.front_mut().unwrap();
        front.advance(sent);

        if front.remaining() == 0 {
            self.bufs.pop_front();
        }

    }
    fn push_front(&mut self, buf: B) {
        self.bufs.push_front(buf);
    }
    fn push_back(&mut self, buf: B) {
        self.bufs.push_back(buf);
    }
}

enum ConnState {
    Closed,
    Upgrade(UpgradeParser),
    WebSocket(FrameTransducer),
    Error,
}
struct Connection<B: Buf> {
    state: ConnState,
    gen: u8,
    leftover: SendBuf<B>,
}

impl<B: Buf> Connection {
    fn new() -> Self {
        Self {
            Closed,
            0,
            SendBuf::new(),
        }
    }
    fn accept(&mut self) {
        match self.state {
            ConnState::Closed | ConnState::Error => {      
                self.state = Upgrade(UpgradeParser::new());
                self.gen.wrapping_add(1);
                self.leftover = SendBuf::new();
            }
            _ => panic!("Connection already active");
        };
    }
    fn close(&mut self) {
        

    }
}



