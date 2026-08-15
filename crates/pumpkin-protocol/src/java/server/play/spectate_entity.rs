use pumpkin_data::packet::serverbound::PLAY_SPECTATE_ENTITY;
use pumpkin_macros::java_packet;
use pumpkin_util::version::JavaMinecraftVersion;

use crate::{
    ServerPacket,
    codec::var_int::VarInt,
    ser::{NetworkReadExt, ReadingError},
};

#[java_packet(PLAY_SPECTATE_ENTITY)]
pub struct SSpectateEntity {
    /// Zero clears the current target; otherwise this is the entity ID plus one.
    pub entity_id: VarInt,
}

impl<'a> ServerPacket<'a> for SSpectateEntity {
    fn read(bytebuf: &mut &'a [u8], _version: &JavaMinecraftVersion) -> Result<Self, ReadingError> {
        Ok(Self {
            entity_id: bytebuf.get_var_int()?,
        })
    }
}
