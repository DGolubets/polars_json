import polars as pl

from polars_json import json_strip_nulls


def test_json_strip_nulls():
    df = pl.DataFrame(
        {
            "json": [
                '{"key": "value", "null_key": null}',
                '[{"key": "value", "null_key": null}, {"key": "value2", "null_key": null}]',
                '"string"',
                "null",
                None,
            ],
        }
    )
    result = df.with_columns(json_stripped=json_strip_nulls("json"))

    expected_df = pl.DataFrame(
        {
            "json": [
                '{"key": "value", "null_key": null}',
                '[{"key": "value", "null_key": null}, {"key": "value2", "null_key": null}]',
                '"string"',
                "null",
                None,
            ],
            "json_stripped": [
                '{"key":"value"}',
                '[{"key":"value"},{"key":"value2"}]',
                '"string"',
                "null",
                None,
            ],
        }
    )

    assert result.equals(expected_df)
