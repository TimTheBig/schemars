use crate::SchemaGenerator;
use crate::{json_schema, JsonSchema, Schema};
use alloc::borrow::Cow;
use crate::_alloc_prelude::*;
use geo_3d030::{Coord, Line, LineString, MultiPolygon, Point, Polygon, Triangle};

impl JsonSchema for Coord {
    fn schema_name() -> Cow<'static, str> {
        "Coord".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo-3d::Coord".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "description": "A coordinate in 3-dimensional space.",
            "required": ["x", "y"], // geo-3d will deserialize z as 0 when not present
            "properties": {
                "x": f64::json_schema(generator),
                "y": f64::json_schema(generator),
                "z": f64::json_schema(generator),
            },
        })
    }
}

impl JsonSchema for Point {
    fn schema_name() -> Cow<'static, str> {
        "Point".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo-3d::Point".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "description": "A point in 3-dimensional space.",
            "required": ["x", "y", "z"],
            "properties": {
                "x": f64::json_schema(generator),
                "y": f64::json_schema(generator),
                "z": f64::json_schema(generator),
            },
        })
    }
}

impl JsonSchema for Polygon {
    fn schema_name() -> Cow<'static, str> {
        "Polygon".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo-3d::Polygon".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "description": "A bounded three-dimensional area, with an outer boundary (exterior ring) and zero or more holes (interior rings).",
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
        "geo-3d::LineString".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "description": "An ordered collection of Coords, representing a path between locations.",
            "items": generator.subschema_for::<Coord>(),
            "minItems": 2,
        })
    }
}

impl JsonSchema for MultiPolygon {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "MultiPolygon".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo-3d::MultiPolygon".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "description": "A collection of Polygons.",
            "items": generator.subschema_for::<Polygon>(),
        })
    }
}
// "description":
impl JsonSchema for Line {
    fn schema_name() -> Cow<'static, str> {
        "Line".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo-3d::Line".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut schema = generator.subschema_for::<(Coord, Coord)>();
        schema.insert(
            "description".to_string(),
            serde_json::Value::String("A line segment made up of exactly two Coords.".to_string()),
        );

        schema
    }
}

impl JsonSchema for Triangle {
    fn schema_name() -> Cow<'static, str> {
        "Triangle".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "geo-3d::Triangle".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "description": "An ordered collection of Coords, representing a path between locations.",
            "items": generator.subschema_for::<Coord>(),
            "minItems": 3,
            "maxItems": 3,
        })
    }
}
