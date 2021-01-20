// Automatically generated rust module for 'netpong.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Paddle {
    pub y: f32,
}

impl<'a> MessageRead<'a> for Paddle {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.y = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Paddle {
    fn get_size(&self) -> usize {
        0
        + if self.y == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.y != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.y))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl<'a> MessageRead<'a> for Ball {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = r.read_float(bytes)?,
                Ok(21) => msg.y = r.read_float(bytes)?,
                Ok(29) => msg.vx = r.read_float(bytes)?,
                Ok(37) => msg.vy = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Ball {
    fn get_size(&self) -> usize {
        0
        + if self.x == 0f32 { 0 } else { 1 + 4 }
        + if self.y == 0f32 { 0 } else { 1 + 4 }
        + if self.vx == 0f32 { 0 } else { 1 + 4 }
        + if self.vy == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.x != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.x))?; }
        if self.y != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.y))?; }
        if self.vx != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.vx))?; }
        if self.vy != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.vy))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update {
    pub UpdateType: netpong::mod_Update::OneOfUpdateType,
}

impl<'a> MessageRead<'a> for Update {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.UpdateType = netpong::mod_Update::OneOfUpdateType::paddle(r.read_message::<netpong::Paddle>(bytes)?),
                Ok(18) => msg.UpdateType = netpong::mod_Update::OneOfUpdateType::ball(r.read_message::<netpong::Ball>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Update {
    fn get_size(&self) -> usize {
        0
        + match self.UpdateType {
            netpong::mod_Update::OneOfUpdateType::paddle(ref m) => 1 + sizeof_len((m).get_size()),
            netpong::mod_Update::OneOfUpdateType::ball(ref m) => 1 + sizeof_len((m).get_size()),
            netpong::mod_Update::OneOfUpdateType::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.UpdateType {            netpong::mod_Update::OneOfUpdateType::paddle(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            netpong::mod_Update::OneOfUpdateType::ball(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            netpong::mod_Update::OneOfUpdateType::None => {},
    }        Ok(())
    }
}

pub mod mod_Update {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfUpdateType {
    paddle(netpong::Paddle),
    ball(netpong::Ball),
    None,
}

impl Default for OneOfUpdateType {
    fn default() -> Self {
        OneOfUpdateType::None
    }
}

}

