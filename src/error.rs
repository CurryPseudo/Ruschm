use std::fmt;
use std::{fmt::Debug, ops::Deref, ops::DerefMut};
use thiserror::Error;

use fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Located<T> {
    pub data: T,
    pub location: Option<[u32; 2]>,
}

pub trait ToLocated {
    fn locate(self, location: Option<[u32; 2]>) -> Located<Self>
    where
        Self: Sized,
    {
        Located::<Self> {
            data: self,
            location,
        }
    }

    fn no_locate(self) -> Located<Self>
    where
        Self: Sized,
    {
        Located::<Self> {
            data: self,
            location: None,
        }
    }
}
impl<T> Located<T> {
    pub fn extract_data(self) -> T {
        self.data
    }
}

impl<T> From<T> for Located<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            location: None,
        }
    }
}

impl<T: PartialEq> PartialEq for Located<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Display> Display for Located<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<T> Deref for Located<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> DerefMut for Located<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Error, Debug, PartialEq)]
#[error("Error at location {}:{}", .0[0], .0[1])]
pub struct SchemeErrorLocation(pub [u32; 2]);

#[cfg(test)]
pub(crate) fn convert_located<T>(datas: Vec<T>) -> Vec<Located<T>> {
    datas.into_iter().map(Located::from).collect()
}
