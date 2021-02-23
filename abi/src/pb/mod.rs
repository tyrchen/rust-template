use prost::Message;
use prost_helper::{prost_into_vec, vec_try_into_prost};

mod abi_impl;
pub mod gen;

use gen::*;

// Generate `From` trait for prost messages
prost_into_vec!((RequestPing, 64), (ResponsePong, 64));

// Generate `TryFrom` trait for Vec<u8>
vec_try_into_prost!(ResponsePong);
