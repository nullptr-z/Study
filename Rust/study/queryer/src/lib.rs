mod convert;
mod dialect;
mod fetcher;

use anyhow::{anyhow, Ok, Result};
use polars::prelude::DataFrame;
use std::{
  hash::Hasher,
  ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct DataSet(DataFrame);

// 让 DataSet 用起来和 DataFrame 一致
impl Deref for DataSet {
  type Target = DataFrame;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// 让 DataSet 用起来和 DataFrame 一致
impl DerefMut for DataSet {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl DataSet {
  // DataSet 转换成 csv
  pub fn to_csv(&self) -> Result<()> {
    let mut buf = Vec::new();
    let writer = CsvWriter::new(&mut buf);
    writer.finish(self)?;
    Ok(String::from_utf8(buf)?)
  }
}
