use use_elasticity::{ElasticBar, ElasticMaterial, axial_deformation, normal_stress};

#[test]
fn elasticity_helpers_cover_basic_usage() {
    let Some(material) = ElasticMaterial::with_poisson_ratio(260.0, 0.3) else {
        panic!("expected valid ElasticMaterial");
    };
    let Some(bar) = ElasticBar::new(10.0, 2.0, 1_000.0) else {
        panic!("expected valid ElasticBar");
    };

    assert_eq!(normal_stress(100.0, 2.0), Some(50.0));
    assert_eq!(axial_deformation(100.0, 10.0, 2.0, 1_000.0), Some(0.5));
    assert_eq!(bar.deformation_under_force(100.0), Some(0.5));
    assert!(
        matches!(material.bulk_modulus(), Some(value) if (value - 216.666_666_666_666_69).abs() < 1.0e-12)
    );
}
