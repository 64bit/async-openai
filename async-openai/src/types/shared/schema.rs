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
