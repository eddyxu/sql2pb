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

use std::fs;
use std::path::Path;

use clap::{App, Arg};
use prost::Message;

use sql2pb::sql2pb;

fn main() {
    let matches = App::new("sql2pb")
        .author("Lei Xu <eddyxu@gmail.com>")
        .about("Convert SQL AST into protobuf")
        .arg(
            Arg::with_name("in")
                .help("input file")
                .required(true)
                .value_name("SQL_FILE"),
        )
        .arg(
            Arg::with_name("out")
                .help("output file")
                .value_name("OUT_FILE"),
        )
        .arg(
            Arg::with_name("dialect")
                .help("SQL dialect")
                .short("d")
                .long("--dialect")
                .default_value("bigquery")
                .value_name("ansi|bigquery|postgres|mysql"),
        )
        .get_matches();

    let sql_file = matches.value_of("in").unwrap();
    let ast_file = matches.value_of("out").unwrap();
    let sql = fs::read_to_string(sql_file).unwrap();
    let stmt = sql2pb(sql.as_str()).unwrap();

    let stmt = &stmt[0];
    let mut buf: Vec<u8> = vec![];
    stmt.encode(&mut buf).unwrap();

    fs::write(Path::new(ast_file), buf.as_slice()).unwrap();
}
