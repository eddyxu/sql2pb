// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod data_type;
mod expr;
mod value;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/sql2pb.rs"));
}

use std::error::Error;
use std::fmt;

use sqlparser::ast::*;
use sqlparser::dialect::BigQueryDialect;
use sqlparser::parser::Parser;

#[derive(Debug, Clone)]
pub struct ConvertError(String);

impl Error for ConvertError {}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Conversion error: {}", self.0)
    }
}

impl ConvertError {
    fn new(msg: String) -> ConvertError {
        ConvertError(msg)
    }
}

type Result<T> = std::result::Result<T, ConvertError>;

/// A Trait that convert a struct into Protobuf.
///
trait IntoPb<P> {
    /// Convert a sqlparser::ast Node into Protobuf.
    fn into_pb(&self) -> Result<P>;
}

/// Compile a SQL script into list of statements in Protobuf.
pub fn sql2pb(sql: &str) -> Result<Vec<pb::Statement>> {
    let dialect = &BigQueryDialect {};
    let statements = match Parser::parse_sql(dialect, String::from(sql)) {
        Ok(s) => Ok(s),
        Err(e) => Err(ConvertError::new(format!("{}", e))),
    }?;
    return statements.into_iter().map(|s| s.into_pb()).collect();
}

impl IntoPb<pb::Statement> for Statement {
    fn into_pb(&self) -> Result<pb::Statement> {
        match self {
            Statement::CreateFunction {
                name,
                temporary,
                or_replace,
                if_not_exists,
                args,
                returns,
                expr,
            } => Ok(pb::Statement {
                s: Some(pb::statement::S::CreateFunction(
                    pb::statement::CreateFunction {
                        name: name.to_string(),
                        temporary: *temporary,
                        or_replace: *or_replace,
                        if_not_exists: *if_not_exists,
                        returns: match returns {
                            Some(dt) => Some(dt.into_pb()?),
                            None => None,
                        },
                        expr: Some(expr.into_pb()?),
                        args: args.into_pb()?,
                    },
                )),
            }),
            _ => Err(ConvertError::new("unimplemented".to_string())),
        }
    }
}
