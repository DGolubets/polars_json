#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde_json::Value;

#[polars_expr(output_type=String)]
fn json_strip_nulls(inputs: &[Series]) -> PolarsResult<Series> {
    let ca: &StringChunked = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|value: &str, output: &mut String| {
        if let Ok(mut value) = serde_json::from_str::<Value>(value) {
            strip_nulls(&mut value);
            unsafe {
                let vec = output.as_mut_vec();
                serde_json::to_writer(vec, &value).unwrap();
            };
        };
    });
    Ok(out.into_series())
}

fn strip_nulls(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for v in map.values_mut() {
                strip_nulls(v);
            }
            map.retain(|_, v| !v.is_null());
        },

        Value::Array(arr) => {
            for v in arr.iter_mut() {
                strip_nulls(v);
            }
            arr.retain(|v| !v.is_null());
        },

        _ => {},
    }
}
