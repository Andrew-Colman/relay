/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use fixture_tests::Fixture;
use fnv::FnvHashMap;
use graphql_ir::{build, Program};
use graphql_syntax::parse_executable;
use graphql_transforms::validate_relay_directives;
use test_schema::get_test_schema;

pub fn transform_fixture(fixture: &Fixture) -> Result<String, String> {
    let source_location = SourceLocationKey::standalone(fixture.file_name);

    let mut sources = FnvHashMap::default();
    sources.insert(source_location, fixture.content);

    let schema = get_test_schema();
    let ast = parse_executable(fixture.content, source_location).unwrap();
    let ir_result = build(&schema, &ast.definitions);
    let ir = match ir_result {
        Ok(res) => res,
        Err(errors) => {
            let mut errs = errors
                .into_iter()
                .map(|err| err.print_with_sources(&sources))
                .collect::<Vec<_>>();
            errs.sort();
            return Err(errs.join("\n\n"));
        }
    };

    let program = Program::from_definitions(schema, ir);
    let validation_result = validate_relay_directives(&program);

    match validation_result {
        Ok(_) => Ok("OK".to_owned()),
        Err(errors) => {
            let mut errs = errors
                .into_iter()
                .map(|err| err.print_with_sources(&sources))
                .collect::<Vec<_>>();
            errs.sort();
            Err(errs.join("\n\n"))
        }
    }
}
