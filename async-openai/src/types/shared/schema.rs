//! Helpers for schema implementations that cannot use the derive directly.

// The component name is explicit so callers in any type module can reuse this
// helper without exposing that module's internal path in the OpenAPI document.
macro_rules! schema {
    ($ty:ty, $name:literal, $body:expr, [$($dependency:ty),* $(,)?]) => {
        impl utoipa::PartialSchema for $ty {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
                $body.into()
            }
        }
        impl utoipa::ToSchema for $ty {
            fn name() -> std::borrow::Cow<'static, str> {
                $name.into()
            }
            fn schemas(
                schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>,
            ) {
                $(<$dependency as utoipa::ToSchema>::schemas(schemas);)*
                // Some implementations only contain inline primitive schemas.
                let _ = schemas;
            }
        }
    };
}

pub(crate) use schema;

/// OpenAI's numeric limits apply to each bias, not to the map itself.
/// This schema metadata does not change the public Rust types or Serde behavior.
pub(crate) fn logit_bias() -> utoipa::openapi::schema::Object {
    use utoipa::openapi::schema::{ObjectBuilder, SchemaType, Type};

    ObjectBuilder::new()
        .schema_type(SchemaType::from_iter([Type::Object, Type::Null]))
        // schema_with replaces the whole property, including its description.
        .description(Some(
            "Map token IDs to integer bias values from -100 to 100. The bias is added \
             to the logits before sampling. Values near zero slightly change token \
             likelihood; -100 and 100 should result in a ban or exclusive selection.",
        ))
        .property_names(Some(ObjectBuilder::new().schema_type(Type::String)))
        .additional_properties(Some(
            ObjectBuilder::new()
                .schema_type(Type::Integer)
                .minimum(Some(-100))
                .maximum(Some(100)),
        ))
        .build()
}

#[cfg(test)]
mod tests {
    struct Example;

    crate::types::shared::schema::schema!(
        Example,
        "example.CustomName",
        <String as utoipa::PartialSchema>::schema(),
        []
    );

    #[test]
    fn component_name_is_independent_of_the_rust_type_and_module() {
        assert_eq!(<Example as utoipa::ToSchema>::name(), "example.CustomName");
        assert_eq!(
            serde_json::to_value(<Example as utoipa::PartialSchema>::schema()).unwrap(),
            serde_json::json!({"type": "string"})
        );
    }
}
