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
use crate::ConvertError;
use crate::IntoPb;

use sqlparser::ast::Value;

impl IntoPb<pb::Value> for Value {
    fn into_pb(&self) -> Result<pb::Value, ConvertError> {
        match self {
            Value::Number(v) => Ok(pb::Value {
                v: Some(pb::value::V::Number(v.to_string())),
            }),
            Value::SingleQuotedString(v) => Ok(pb::Value {
                v: Some(pb::value::V::SingleQuotedString(v.to_string())),
            }),
            Value::Null => Ok(pb::Value {
                v: Some(pb::value::V::Null(true)),
            }),
            _ => Err(ConvertError::new(format!("not supported value {}", self))),
        }
    }
}
