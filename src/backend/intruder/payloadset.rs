use std::path::{Path, PathBuf};
use polars::{prelude::*, sql::SQLContext};

pub fn read_from_file(file: PathBuf){
  todo!("need to see how to deal with it in chunks")
}

fn seperator_identifier(){
  todo!("find the seperator here")
}

enum Column{
  None,
  Single,
  Multiple(usize)
}

fn get_cols(df: &DataFrame) -> Column{
  let cols = df.width();
  match cols{
    0 => Column::None,
    1 => Column::Single,
    _ => Column::Multiple(cols)
  }
}

// default no headers
pub fn read_csv(path: impl AsRef<Path>, has_headers: bool, separator: u8) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(has_headers)
        .with_parse_options(CsvParseOptions{separator, ..Default::default()})
        .try_into_reader_with_file_path(Some(path.as_ref().into()))?
        .finish()
}

// TODO: add logger later
pub fn run_sql_query(file: impl AsRef<Path>, query: &str, has_headers: bool, seperator: u8) -> Result<DataFrame, String>{
  let df = read_csv(file, has_headers, seperator).map_err(|e| format!("failed to load csv file: {e}"))?;

  let mut ctx = SQLContext::new();
  ctx.register("payload ith file", df.lazy());

  let plan = ctx.execute(query).map_err(|e| format!("SQL Syntax Error: {e}"))?;
  plan.collect().map_err(|e| format!("Query execution failed: {e}"))
}
