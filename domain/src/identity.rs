// use anyhow::Result;
use std::fmt;

use async_trait::async_trait;

#[async_trait]
pub trait Identity: ToString + fmt::Display {}

pub struct UniqueId<T: Identity>(T);

impl<T: Identity> UniqueId<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T: Identity> fmt::Display for UniqueId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Identity for i32 {}
impl Identity for String {}
impl Identity for i64 {}
impl Identity for UniqueId<i32> {}
impl Identity for UniqueId<i64> {}
impl Identity for UniqueId<String> {}

pub type UniqueIdStr = UniqueId<String>;
pub type UniqueIdInt = UniqueId<i32>;
pub type UniqueIdInt64 = UniqueId<i64>;
