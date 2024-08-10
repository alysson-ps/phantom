use anyhow::Result;

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/php.pest"]
struct MyParser;

pub fn parser(source: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    pest::set_error_detail(true);
    MyParser::parse(Rule::program, source)
}
