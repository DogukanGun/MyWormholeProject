use anchor_lang::{prelude::Pubkey, AnchorDeserialize, AnchorSerialize};
use std::io;

const PAYLOAD_ID_SIGN: u8 = 0;
const PAYLOAD_ID_MESSAGE: u8 = 1;

pub const SIGN_MAX_LENGTH: usize = 512;

#[derive(Clone)]
pub enum SignMessage {
    Sign { program_id: Pubkey },
    SignMessage { message: Vec<u8> },
}

impl AnchorSerialize for SignMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            SignMessage::Sign { program_id } => {
                PAYLOAD_ID_SIGN.serialize(writer)?;
                program_id.serialize(writer)
            }
            SignMessage::SignMessage { message } => {
                PAYLOAD_ID_SIGN.serialize(writer)?;
                message.serialize(writer)
            }
        }
    }
}

impl AnchorDeserialize for SignMessage {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        match buf[0] {
            PAYLOAD_ID_SIGN => Ok(SignMessage::Sign {
                program_id: Pubkey::try_from(&buf[1..33]).unwrap(),
            }),
            PAYLOAD_ID_MESSAGE => {
                let length = {
                    let mut out = [0u8; 2];
                    out.copy_from_slice(&buf[1..3]);
                    u16::from_be_bytes(out) as usize
                };
                if length > SIGN_MAX_LENGTH {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("message exceeds {SIGN_MAX_LENGTH} bytes"),
                    ))
                } else {
                    Ok(SignMessage::SignMessage {
                        message: buf[3..(3 + length)].to_vec(),
                    })
                }
            },
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid payload ID",
            )),
        }
    }
}