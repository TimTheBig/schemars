use crate::SchemaGenerator;
use crate::{json_schema, JsonSchema, Schema};
use alloc::borrow::Cow;
use crate::_alloc_prelude::*;
use geo028::{Coord, LineString, MultiPolygon, Polygon};

impl JsonSchema for Coord {
    fn schema_name() -> Cow<'static, str> {
        "Coord".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::Coord".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "required": ["x", "y"],
            "properties": {
                "x": f64::json_schema(generator),
                "y": f64::json_schema(generator),
            },
        })
    }
}

impl JsonSchema for Polygon {
    fn schema_name() -> Cow<'static, str> {
        "Polygon".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::Polygon".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "required": ["exterior", "interiors"],
            "properties": {
                "exterior": LineString::json_schema(generator),
                "interiors": <Vec<LineString>>::json_schema(generator),
            },
        })
    }
}

impl JsonSchema for LineString {
    fn schema_name() -> Cow<'static, str> {
        "LineString".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::LineString".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "items": generator.subschema_for::<Coord>(),
        })
    }
}

impl JsonSchema for MultiPolygon {
    fn schema_name() -> Cow<'static, str> {
        "MultiPolygon".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo::MultiPolygon".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "items": generator.subschema_for::<Polygon>(),
        })
    }
}
