use wgpu_types::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureViewDimension,
};

#[test]
fn texture_binding_view_dimension_survives_descriptor_mapping() {
    for dimension in [
        None,
        Some(TextureViewDimension::D2Array),
        Some(TextureViewDimension::Cube),
    ] {
        let descriptor = TextureDescriptor {
            label: "source",
            size: Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING,
            view_formats: Vec::<TextureFormat>::new(),
            texture_binding_view_dimension: dimension,
        };

        let relabeled = descriptor.map_label(|_| "mapped");
        assert_eq!(relabeled.label, "mapped");
        assert_eq!(relabeled.texture_binding_view_dimension, dimension);

        let remapped = descriptor.map_label_and_view_formats(|_| (), |formats| formats.clone());
        assert_eq!(remapped.texture_binding_view_dimension, dimension);
    }
}

#[cfg(feature = "serde")]
#[test]
fn texture_binding_view_dimension_serde_is_backward_compatible() {
    use serde_json::{json, Value};

    for (field, expected) in [
        (None, None),
        (Some(Value::Null), None),
        (Some(json!("2d-array")), Some(TextureViewDimension::D2Array)),
        (Some(json!("cube")), Some(TextureViewDimension::Cube)),
    ] {
        let mut legacy = json!({
            "label": "legacy",
            "size": { "width": 4, "height": 4, "depth_or_array_layers": 6 },
            "mip_level_count": 1,
            "sample_count": 1,
            "dimension": "2d",
            "format": "rgba8unorm",
            "usage": "TEXTURE_BINDING",
            "view_formats": [],
        });
        if let Some(value) = &field {
            legacy["texture_binding_view_dimension"] = value.clone();
        }
        let descriptor: TextureDescriptor<String, Vec<TextureFormat>> =
            serde_json::from_value(legacy).expect("valid texture descriptor");
        assert_eq!(descriptor.texture_binding_view_dimension, expected);

        let encoded = serde_json::to_value(&descriptor).expect("serializable texture descriptor");
        assert_eq!(
            encoded["texture_binding_view_dimension"],
            field.unwrap_or(Value::Null)
        );
        let round_trip: TextureDescriptor<String, Vec<TextureFormat>> =
            serde_json::from_value(encoded).expect("valid serialized texture descriptor");
        assert_eq!(round_trip, descriptor);
    }
}
