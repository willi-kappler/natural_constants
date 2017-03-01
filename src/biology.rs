//! Biology related constants
//!
//! Taken from http://bionumbers.hms.harvard.edu/search.aspx?task=searchbypop
//!
//! Every organism is it's own module, so we don't have overlapping values.
//!
//! TODO: Needs some more documentation and usage examples

mod generic {
    pub struct BioInfo {
        pub value: Option<f64>,
        pub value_min: Option<f64>,
        pub value_max: Option<f64>,
    }
}

pub mod human_homo_sapiens {
    pub struct BioInfo {
        pub value: Option<f64>,
        pub value_min: Option<f64>,
        pub value_max: Option<f64>,
    }

    /// [kb / min]
    pub const rate_dna_replication: BioInfo = BioInfo {
        value: Some(2.0),
        value_min: Some(0.0),
        value_max: Some(2.0),
    };

    /// [kb]
    pub const spacing_between_origins_dna_replication: BioInfo = BioInfo {
        value: Some(100.0),
        value_min: None,
        value_max: None,
    };

    /// [µm]
    pub const diameter_hek_293_cell: BioInfo = BioInfo {
        value: Some(13.0),
        value_min: None,
        value_max: None,
    };
}

pub mod unspecified {
    pub struct BioInfo {
        pub value: Option<f64>,
        pub value_min: Option<f64>,
        pub value_max: Option<f64>,
    }

    /// [nm]
    pub const excitation_maximum_of_rfp: BioInfo = BioInfo {
        value: Some(587.0),
        value_min: None,
        value_max: None,
    };
} /* unspecified */

pub mod bacteria_escherichia_coli {
    pub struct BioInfo {
        value: Option<f64>,
        value_min: Option<f64>,
        value_max: Option<f64>,
    }

    /// mM
    pub const affinity_of_allolactose_and_lacz: BioInfo = BioInfo {
        value: Some(1.9),
        value_min: None,
        value_max: None,
    };

    /// μM
    pub const affinity_of_allolactose_and_laci: BioInfo = BioInfo {
        value: Some(6.0),
        value_min: None,
        value_max: None,
    };

    /// Molecules / minute
    pub const velocity_of_allolactose_hydrolysis_by_lacz: BioInfo = BioInfo {
        value: Some(20_000.0),
        value_min: None,
        value_max: None,
    };

    /// µM
    pub const maximal_amount_of_lacy: BioInfo = BioInfo {
        value: Some(50.0),
        value_min: None,
        value_max: None,
    };

    /// µM
    pub const maximal_amount_of_lacz: BioInfo = BioInfo {
        value: Some(50.0),
        value_min: None,
        value_max: None,
    };

    /// µM
    pub const total_amount_of_laci_repressor: BioInfo = BioInfo {
        value: Some(0.01),
        value_min: None,
        value_max: None,
    };

    /// %
    pub const percent_of_cell_total_dry_weight_that_is_lipopolysaccharide: BioInfo = BioInfo {
        value: Some(3.4),
        value_min: None,
        value_max: None,
    };

    /// Unitless
    pub const number_of_proteins_in_50s_subunit: BioInfo = BioInfo {
        value: Some(36.0),
        value_min: None,
        value_max: None,
    };

    /// Unitless
    pub const number_of_proteins_in_30s_subunit: BioInfo = BioInfo {
        value: Some(22.0),
        value_min: None,
        value_max: None,
    };

    /// Unitless
    pub const number_of_protein_types_to_make_ribosome: BioInfo = BioInfo {
        value: Some(56.0),
        value_min: None,
        value_max: None,
    };

    /// Unitless
    pub const number_of_lipids_per_cell: BioInfo = BioInfo {
        value: Some(22000000.0),
        value_min: None,
        value_max: None,
    };

    /// Unitless
    pub const number_of_lipopolysaccharide_per_cell: BioInfo = BioInfo {
        value: Some(1430000.0),
        value_min: None,
        value_max: None,
    };

    /// Unitless
    pub const number_of_all_rna_per_cell: BioInfo = BioInfo {
        value: Some(255000.0),
        value_min: None,
        value_max: None,
    };

    /// %
    pub const volume_occupied_by_water: BioInfo = BioInfo {
        value: Some(70.0),
        value_min: None,
        value_max: None,
    };

    /// µm^2/sec
    pub const apparent_diffusion_constant_of_protein_along_dna_segments: BioInfo = BioInfo {
        value: Some(0.4),
        value_min: Some(0.38),
        value_max: Some(0.42),
    };

    /// Molecules / cell
    pub const reca_molecules_per_cell: BioInfo = BioInfo {
        value: Some(1000.0),
        value_min: Some(800.0),
        value_max: Some(1200.0),
    };

    /// µmol / g
    pub const imp_pool_size: BioInfo = BioInfo {
        value: Some(0.38),
        value_min: Some(0.37),
        value_max: Some(0.39),
    };

    /// µmol / g
    pub const carbamoyl_aspartate_pool_size: BioInfo = BioInfo {
        value: Some(0.84),
        value_min: Some(0.56),
        value_max: Some(1.12),
    };

    /// µmol / g
    pub const valine_pool_size: BioInfo = BioInfo {
        value: Some(2.41),
        value_min: Some(2.14),
        value_max: Some(2.68),
    };

    /// µmol / g
    pub const tyrosine_pool_size: BioInfo = BioInfo {
        value: Some(0.41),
        value_min: Some(0.16),
        value_max: Some(0.66),
    };

    /// µmol / g
    pub const threonine_pool_size: BioInfo = BioInfo {
        value: Some(1.34),
        value_min: Some(1.50),
        value_max: Some(1.18),
    };

    /// µmol / g
    pub const proline_pool_size: BioInfo = BioInfo {
        value: Some(1.1),
        value_min: Some(0.95),
        value_max: Some(1.25),
    };

    /// µmol / g
    pub const methionine_pool_size: BioInfo = BioInfo {
        value: Some(0.29),
        value_min: Some(0.22),
        value_max: Some(0.36),
    };

    /// µmol / g
    pub const aspartate_pool_size: BioInfo = BioInfo {
        value: Some(6.45),
        value_min: Some(9.9),
        value_max: Some(2.91),
    };

    /// µmol / g
    pub const asparagine_pool_size: BioInfo = BioInfo {
        value: Some(2.02),
        value_min: Some(1.56),
        value_max: Some(2.48),
    };

    /// µmol / g
    pub const alanine_pool_size: BioInfo = BioInfo {
        value: Some(6.81),
        value_min: Some(5.102),
        value_max: Some(8.51),
    };

    /// µmol / g
    pub const glutamate_pool_size: BioInfo = BioInfo {
        value: Some(100.55),
        value_min: Some(83.001),
        value_max: Some(118.09),
    };

    /// µmol / g
    pub const glutamine_pool_size: BioInfo = BioInfo {
        value: Some(3.92),
        value_min: Some(3.75),
        value_max: Some(4.09),
    };

    /// fg
    pub const mass_in_excess_of_displaced_buffer: BioInfo = BioInfo {
        value: Some(110.0),
        value_min: Some(80.0),
        value_max: Some(140.0),
    };

    /// nm
    pub const outer_membrane_thickness: BioInfo = BioInfo {
        value: Some(13.0),
        value_min: Some(12.0),
        value_max: Some(14.0),
    };

    /// µm^3
    pub const cell_total_volume: BioInfo = BioInfo {
        value: Some(1.1),
        value_min: Some(0.44),
        value_max: Some(1.79),
    };

    /// [µm]
    pub const concentration_pyruvate: BioInfo = BioInfo {
        value: Some(390.0),
        value_min: None,
        value_max: None,
    };

    /// [g / liter]
    pub const ratio_cell_dry_weight: BioInfo = BioInfo {
        value: Some(0.36),
        value_min: None,
        value_max: None,
    };
} /* bacteria_escherichia_coli */

pub mod chinese_hamster_ovary {
    pub struct BioInfo {
        pub value: Option<f64>,
        pub value_min: Option<f64>,
        pub value_max: Option<f64>,
    }

    /// [µm]
    pub const diameter_cell: BioInfo = BioInfo {
        value: None,
        value_min: Some(14.02),
        value_max: Some(15.21),
    };

} /* chinese_hamster_ovary */

pub mod budding_yeast_saccharomyces_cerevisiae {
    pub struct BioInfo {
        pub value: Option<f64>,
        pub value_min: Option<f64>,
        pub value_max: Option<f64>,
    }

    /// [pg / cell]
    pub const mass_protein_diploid_cell: BioInfo = BioInfo {
        value: Some(8.0),
        value_min: None,
        value_max: None,
    };
} /* budding_yeast_saccharomyces_cerevisiae  */

pub mod green_algae_chlorella_vulgaris {
    pub struct BioInfo {
        pub value: Option<f64>,
        pub value_min: Option<f64>,
        pub value_max: Option<f64>,
    }

    /// [µm]
    pub const size_diameter_green_algea: BioInfo = BioInfo {
        value: Some(3.0),
        value_min: Some(2.0),
        value_max: Some(4.0),
    };
} /* green_algae_chlorella_vulgaris */

