#[derive(Debug)]
pub enum RaknetError {
    SetRaknetRawSocketError,
    NotListen,
    BindAdressError,
    ConnectionClosed,
    NotSupportVersion,
    IncorrectReply,
    PacketParseError,
    SocketError,
    IncorrectReliability,
    IncorrectPacketID,
    ReadPacketBufferError,
    PacketSizeExceedMTU,
    PacketHeaderError,
    ConnectionTimeout,
}

pub type Result<T> = std::result::Result<T, RaknetError>;
