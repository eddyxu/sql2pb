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
use crate::Result;

use sqlparser::ast::*;

impl IntoPb<pb::Ident> for Ident {
    fn into_pb(&self) -> Result<pb::Ident> {
        Ok(pb::Ident {
            value: self.value.to_string(),
            quote_style: match self.quote_style {
                Some(s) => format!("{}", s),
                None => String::from(""),
            },
        })
    }
}

impl IntoPb<Vec<pb::Ident>> for Vec<Ident> {
    fn into_pb(&self) -> Result<Vec<pb::Ident>> {
        self.iter().map(|x| x.into_pb()).collect()
    }
}

/// Convert an vector of sql Expr to protobuf
impl IntoPb<Vec<pb::Expr>> for Vec<Expr> {
    fn into_pb(&self) -> Result<Vec<pb::Expr>> {
        self.into_iter().map(|e| e.into_pb()).collect()
    }
}

impl IntoPb<pb::Expr> for Expr {
    fn into_pb(&self) -> Result<pb::Expr> {
        match self {
            Expr::Identifier(Ident { value, quote_style }) => Ok(pb::Expr {
                expr: Some(pb::expr::Expr::Identifier(pb::expr::Identifier {
                    ident: Some(pb::Ident {
                        value: String::from(value),
                        quote_style: match quote_style {
                            Some(c) => format!("{}", c),
                            None => String::from(""),
                        },
                    }),
                })),
            }),
            Expr::Parameter {
                name,
                data_type,
                default: _,
            } => Ok(pb::Expr {
                expr: Some(pb::expr::Expr::Parameter(pb::expr::Parameter {
                    name: match name {
                        Some(id) => Some(id.into_pb()?),
                        None => None,
                    },
                    data_type: format!("{}", data_type),
                    default: None,
                })),
            }),
            Expr::Function(Function {
                name,
                args,
                over: _,
                distinct,
            }) => {
                let mut pb_args = vec![];
                for a in args {
                    pb_args.push(a.into_pb()?);
                }
                Ok(pb::Expr {
                    expr: Some(pb::expr::Expr::Function(pb::expr::Function {
                        name: name.to_string(),
                        args: pb_args,
                        distinct: *distinct,
                    })),
                })
            }
            Expr::UnaryOp { op, expr } => {
                use pb::expr::unary_op::Operator;
                Ok(pb::Expr {
                    expr: Some(pb::expr::Expr::UnaryOp(Box::new(pb::expr::UnaryOp {
                        op: match op {
                            UnaryOperator::Plus => Operator::Plus,
                            UnaryOperator::Minus => Operator::Minus,
                            UnaryOperator::Not => Operator::Not,
                        } as i32,
                        expr: Some(Box::new(expr.into_pb()?)),
                    }))),
                })
            }
            Expr::BinaryOp { left, op, right } => {
                use pb::expr::binary_op::Operator;
                Ok(pb::Expr {
                    expr: Some(pb::expr::Expr::BinaryOp(Box::new(pb::expr::BinaryOp {
                        left: Some(Box::new(left.into_pb()?)),
                        op: match op {
                            BinaryOperator::Plus => Operator::Plus,
                            BinaryOperator::Minus => Operator::Minus,
                            BinaryOperator::Multiply => Operator::Multiply,
                            BinaryOperator::Divide => Operator::Divide,
                            BinaryOperator::Modulus => Operator::Modulus,
                            BinaryOperator::Gt => Operator::Gt,
                            BinaryOperator::Lt => Operator::Lt,
                            BinaryOperator::GtEq => Operator::GtEq,
                            BinaryOperator::LtEq => Operator::LtEq,
                            BinaryOperator::Eq => Operator::Eq,
                            BinaryOperator::NotEq => Operator::NotEq,
                            BinaryOperator::And => Operator::And,
                            BinaryOperator::Or => Operator::Or,
                            BinaryOperator::Like => Operator::Like,
                            BinaryOperator::NotLike => Operator::NotLike,
                        } as i32,
                        right: Some(Box::new(right.into_pb()?)),
                    }))),
                })
            }
            Expr::Cast { expr, data_type } => Ok(pb::Expr {
                expr: Some(pb::expr::Expr::Cast(Box::new(pb::expr::Cast {
                    expr: Some(Box::new(expr.into_pb()?)),
                    data_type: format!("{}", data_type),
                }))),
            }),
            Expr::Case {
                operand,
                conditions,
                results,
                else_result,
            } => Ok(pb::Expr {
                expr: Some(pb::expr::Expr::Case(Box::new(pb::expr::Case {
                    operand: match operand {
                        Some(e) => Some(Box::new(e.into_pb()?)),
                        None => None,
                    },
                    conditions: conditions.into_pb()?,
                    results: results.into_pb()?,
                    else_result: match else_result {
                        Some(e) => Some(Box::new(e.into_pb()?)),
                        None => None,
                    },
                }))),
            }),
            Expr::Value(v) => Ok(pb::Expr {
                expr: Some(pb::expr::Expr::Value(
                    pb::expr::ValueExpr{
                        value: Some(v.into_pb()?),
                    })),
            }),
            Expr::Nested(e) => Ok(pb::Expr {
                expr: Some(pb::expr::Expr::Nested(Box::new(pb::expr::Nested {
                    expr: Some(Box::new(e.into_pb()?)),
                }))),
            }),
            Expr::Between {
                expr,
                negated,
                low,
                high,
            } => Ok(pb::Expr {
                expr: Some(pb::expr::Expr::Between(Box::new(pb::expr::Between {
                    expr: Some(Box::new(expr.into_pb()?)),
                    negated: *negated,
                    low: Some(Box::new(low.into_pb()?)),
                    high: Some(Box::new(high.into_pb()?)),
                }))),
            }),
            _ => Err(ConvertError::new(format!("Dont supported expr {:?}", self))),
        }
    }
}
