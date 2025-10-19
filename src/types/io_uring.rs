// --- 1. userdata ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoUringOp {
    Accept = 13,
    Close = 19,
    PollAdd = 6,
    ProvideBuffers = 31,
    Recv = 27,
    RemoveBuffers = 32,
    Send = 26,
    Unknown = 255,
}

impl TryFrom<u8> for IoUringOp {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            6 => Ok(IOUringOp::PollAdd),
            13 => Ok(IOUringOp::Accept),
            19 => Ok(IOUringOp::Close),
            26 => Ok(IOUringOp::Send),
            27 => Ok(IOUringOp::Recv),
            31 => Ok(IOUringOp::ProvideBuffers),
            32 => Ok(IOUringOp::RemoveBuffers),
            255 => Ok(IOUringOp::Unknown),
            _ => Err(()),
        }
    }
}

pub trait UserDataExt {
    fn pack(fd: Option<i32>, gen: u16, shard: u8, event: IoUringOp) -> Self;
    fn opcode(&self) -> IoUringOp;
    fn shard(&self) -> u8;
    fn gen(&self) -> u16;
    fn fd(&self) -> Option<u32>;
}

impl UserDataExt for u64 {
    #[inline]
    fn pack(fd: Option<i32>, gen: u16, shard: u8, event: IoUringOp) -> u64 {
        let fd_u32 = fd.map(|v| v as u32).unwrap_or(u32::MAX);
        (event as u64 & 0xFF)
            | ((shard as u64 & 0xFF) << 8)
            | ((gen as u64 & 0xFFFF) << 16)
            | ((fd_u32 as u64 & 0xFFFF_FFFF) << 32)
    }

    fn opcode(&self) -> IoUringOp {
        let raw = (*self & 0xFF) as u8;
        IOUringOp::try_from(raw).unwrap_or(IOUringOp::Unknown)
    }

    #[inline]
    fn shard(&self) -> u8 {
        let shard = ((*self >> 8) & 0xFF) as u8;
        shard
    }

    #[inline]
    fn gen(&self) -> u16 {
        let gen = ((*self >> 16) & 0xFFFF) as u16;
        gen
    }

    fn fd(&self) -> Option<u32> {
        let fd = ((*self >> 32) & 0xFFFF_FFFF) as u32;
        if fd == u32::MAX {
            None
        } else {
            Some(fd as u32)
        }
    }
}
