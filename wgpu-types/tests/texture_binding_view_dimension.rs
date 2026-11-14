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
