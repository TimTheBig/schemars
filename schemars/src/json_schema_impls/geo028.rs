use crate::SchemaGenerator;
use crate::{json_schema, JsonSchema, Schema};
use alloc::borrow::Cow;
use geo028::{Coord, LineString, MultiPolygon, Polygon};

impl JsonSchema for Coord {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "Coord".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::Coord".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
        })
    }
}

impl JsonSchema for Polygon {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "Polygon".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::Polygon".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "required": ["exterior", "interiors"],
        })
    }
}

impl JsonSchema for LineString {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "LineString".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::LineString".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
        })
    }
}

impl JsonSchema for MultiPolygon {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "MultiPolygon".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::MultiPolygon".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
        })
    }
}
