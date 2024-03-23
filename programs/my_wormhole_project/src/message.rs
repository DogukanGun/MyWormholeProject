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
    fn deserialize_reader<R: io::prelude::Read>(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0; 3]; // Assuming the message contains at least 3 bytes
        reader.read_exact(&mut buf)?;

        match buf[0] {
            PAYLOAD_ID_SIGN => {
                // Deserialize Sign message
                let mut program_id_buf = [0; 32];
                reader.read_exact(&mut program_id_buf)?;
                let program_id = Pubkey::try_from(&program_id_buf[..]).unwrap();
                Ok(SignMessage::Sign { program_id })
            }
            PAYLOAD_ID_MESSAGE => {
                // Deserialize Message message
                let mut length_buf = [0; 2];
                reader.read_exact(&mut length_buf)?;
                let length = u16::from_be_bytes(length_buf) as usize;

                if length > SIGN_MAX_LENGTH {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("message exceeds {} bytes", SIGN_MAX_LENGTH),
                    ));
                }

                let mut message_data = vec![0; length];
                reader.read_exact(&mut message_data)?;
                Ok(SignMessage::SignMessage {
                    message: message_data,
                })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid payload ID",
            )),
        }
    }
}
