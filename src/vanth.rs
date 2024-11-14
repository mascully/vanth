#![allow(unused)]

use std::path::PathBuf;

pub struct Value {}

pub struct Ty {}

pub enum Primitive {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Path(PathBuf),
    Char(char),
    Byte(u8),
    ByteArray(Vec<u8>),
}
