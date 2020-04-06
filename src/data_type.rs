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

use crate::pb;
use crate::IntoPb;
use crate::Result;

use sqlparser::ast::DataType;

impl IntoPb<pb::DataType> for DataType {
    fn into_pb(&self) -> Result<pb::DataType> {
        use pb::data_type as dt;
        Ok(pb::DataType {
            data_type: Some(match self {
                DataType::Char(s) => dt::DataType::Char(dt::Char {
                    length: s.unwrap_or(0),
                }),
                DataType::Varchar(s) => dt::DataType::Varchar(dt::Varchar {
                    length: s.unwrap_or(0),
                }),
                DataType::Uuid => dt::DataType::Uuid(dt::Uuid {}),
                DataType::Clob(s) => dt::DataType::Clob(dt::Clob { length: *s }),
                DataType::Binary(s) => dt::DataType::Binary(dt::Binary { length: *s }),
                DataType::Varbinary(s) => dt::DataType::Varbinary(dt::Varbinary { length: *s }),
                DataType::Blob(s) => dt::DataType::Blob(dt::Blob { length: *s }),
                DataType::Decimal(p, s) => dt::DataType::Decimal(dt::Decimal {
                    precision: p.unwrap_or(0),
                    scale: s.unwrap_or(0),
                }),
                DataType::Float(s) => dt::DataType::Float(dt::Float {
                    length: s.unwrap_or(8),
                }),
                DataType::SmallInt => dt::DataType::Smallint(dt::SmallInt {}),
                DataType::Int => dt::DataType::Int(dt::Int {}),
                DataType::BigInt => dt::DataType::Bigint(dt::BigInt {}),
                DataType::Real => dt::DataType::Real(dt::Real {}),
                DataType::Double => dt::DataType::Double(dt::Double {}),
                DataType::Boolean => dt::DataType::Boolean(dt::Boolean {}),
                DataType::Date => dt::DataType::Date(dt::Date {}),
                DataType::Time => dt::DataType::Time(dt::Time {}),
                DataType::Timestamp => dt::DataType::Timestamp(dt::Timestamp {}),
                DataType::Interval => dt::DataType::Interval(dt::Interval {}),
                DataType::Regclass => dt::DataType::Regclass(dt::Regclass {}),
                DataType::Text => dt::DataType::Text(dt::Text {}),
                DataType::Bytea => dt::DataType::Bytea(dt::Bytea {}),
                DataType::Custom(name) => dt::DataType::Custom(dt::Custom {
                    name: name.0.into_pb()?,
                }),
                DataType::Array(dt) => dt::DataType::Array(Box::new(dt::Array {
                    data_type: Some(Box::new(dt.into_pb()?)),
                })),
            }),
        })
    }
}

#[cfg(test)]
mod tests {}
