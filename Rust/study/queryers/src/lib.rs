mod dialect;
use std::ops::{Deref, DerefMut};

use anyhow::{anyhow, Ok, Result};
pub use dialect::*;
mod loader;
use loader::detect_content;
use polars::{
    io::SerWriter,
    prelude::{CsvWriter, DataFrame, IntoLazy},
};
use sqlparser::parser::Parser;
use tracing::info;

use crate::{convert::Sql, fetcher::retrieve_data};
mod convert;
mod fetcher;

#[derive(Debug)]
pub struct DataSet(DataFrame);

// 让 DataSet 用起来和 DataFrame一样
impl Deref for DataSet {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    // DataSet 转换成 csv
    pub fn to_csv(&self) -> Result<String> {
        let mut buf = Vec::new();
        let writer = CsvWriter::new(&mut buf);
        writer.finish(self)?;
        Ok(String::from_utf8(buf)?)
    }
}

// 从 from中获取数据,从 where中过滤,最后返回需要选取的列
pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("目前只支持单条SQL"));
    };

    let sql = &ast[0];

    // 整个 SQL AST 转换成我们定义的 Sql 结构的细节都埋藏在 try_into() 中
    // 我们只需关注数据结构的使用，怎么转换可以之后需要的时候才关注，这是
    // 关注点分离，是我们控制软件复杂度的法宝。

    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    info!("retrieving·检索 data·数据 from·从 source·源: {}", source);

    // 从 source读取一个 DataSet
    // detect_content，怎么 detect 不重要，重要的是它能根据内容返回 DataSet
    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX));
    }

    Ok(DataSet((filtered.select(selection)).collect()?))
}
