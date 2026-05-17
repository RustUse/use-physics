use use_elasticity::{
    ElasticBar, ElasticMaterial, axial_deformation, elastic_energy_density, normal_stress,
};

fn main() {
    let Some(material) = ElasticMaterial::with_poisson_ratio(260.0, 0.3) else {
        panic!("expected valid ElasticMaterial");
    };
    let Some(bar) = ElasticBar::new(10.0, 2.0, 1_000.0) else {
        panic!("expected valid ElasticBar");
    };

    assert_eq!(normal_stress(100.0, 2.0), Some(50.0));
    assert_eq!(axial_deformation(100.0, 10.0, 2.0, 1_000.0), Some(0.5));
    assert_eq!(bar.deformation_under_force(100.0), Some(0.5));
    assert!(matches!(material.shear_modulus(), Some(value) if (value - 100.0).abs() < 1.0e-12));
    assert_eq!(elastic_energy_density(100.0, 0.01), Some(0.5));
}
