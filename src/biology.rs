//! Biology related constants

// http://bionumbers.hms.harvard.edu/search.aspx?task=searchbypop

// Bacteria Escherichia coli
/// µm^3
pub const e_coli_cell_total_volume: f64 = 1.1;
/// µm^3
pub const e_coli_cell_total_volume_min: f64 = 0.44;
/// µm^3
pub const e_coli_cell_total_volume_max: f64 = 1.79;
/// nm
pub const e_coli_outer_membrane_thickness: f64 = 13.0;
/// nm
pub const e_coli_outer_membrane_thickness_min: f64 = 12.0;
/// nm
pub const e_coli_outer_membrane_thickness_max: f64 = 14.0;
/// %
pub const e_coli_volume_occupied_by_water: f64 = 70.0;
/// Unitless
pub const e_coli_number_of_all_rna_per_cell: f64 = 255000.0;
/// Unitless
pub const e_coli_number_of_lipopolysaccharide_per_cell: f64 = 1430000.0;
/// Unitless
pub const e_coli_number_of_lipids_per_cell: f64 = 22000000.0;
/// Unitless
pub const e_coli_number_of_protein_types_to_make_ribosome: f64 = 56.0;
/// Unitless
pub const e_coli_number_of_proteins_in_30s_subunit: f64 = 22.0;
/// Unitless
pub const e_coli_number_of_proteins_in_50s_subunit: f64 = 36.0;
/// fg
pub const e_coli_mass_in_excess_of_displaced_buffer: f64 = 110.0;
/// fg
pub const e_coli_mass_in_excess_of_displaced_buffer_min: f64 = 80.0;
/// fg
pub const e_coli_mass_in_excess_of_displaced_buffer_max: f64 = 140.0;
/// µM
pub const e_coli_total_amount_of_laci_repressor: f64 = 0.01;
/// µM
pub const e_coli_maximal_amount_of_lacz: f64 = 50.0;
/// µM
pub const e_coli_maximal_amount_of_lacy: f64 = 50.0;
/// Molecules / minute
pub const e_coli_velocity_of_allolactose_hydrolysis_by_lacz: f64 = 20_000.0;
/// μM
pub const e_coli_affinity_of_allolactose_and_laci: f64 = 6.0;
/// mM
pub const e_coli_affinity_of_allolactose_and_lacz: f64 = 1.9;
/// µmol / g
pub const e_coli_glutamine_pool_size: f64 = 3.92;
/// µmol / g
pub const e_coli_glutamine_pool_size_min: f64 = 3.75;
/// µmol / g
pub const e_coli_glutamine_pool_size_max: f64 = 4.09;
/// µmol / g
pub const e_coli_glutamate_pool_size: f64 = 100.55;
/// µmol / g
pub const e_coli_glutamate_pool_size_min: f64 = 83.001;
/// µmol / g
pub const e_coli_glutamate_pool_size_max: f64 = 118.09;
/// µmol / g
pub const e_coli_alanine_pool_size: f64 = 6.81;
/// µmol / g
pub const e_coli_alanine_pool_size_min: f64 = 5.102;
/// µmol / g
pub const e_coli_alanine_pool_size_max: f64 = 8.51;
/// µmol / g
pub const e_coli_asparagine_pool_size: f64 = 2.02;
/// µmol / g
pub const e_coli_asparagine_pool_size_min: f64 = 1.56;
/// µmol / g
pub const e_coli_asparagine_pool_size_max: f64 = 2.48;
/// µmol / g
pub const e_coli_aspartate_pool_size: f64 = 6.45;
/// µmol / g
pub const e_coli_aspartate_pool_size_min: f64 = 9.9;
/// µmol / g
pub const e_coli_aspartate_pool_size_max: f64 = 2.91;
/// µmol / g
pub const e_coli_methionine_pool_size: f64 = 0.29;
/// µmol / g
pub const e_coli_methionine_pool_size_min: f64 = 0.22;
/// µmol / g
pub const e_coli_methionine_pool_size_max: f64 = 0.36;
/// µmol / g
pub const e_coli_proline_pool_size: f64 = 1.1;
/// µmol / g
pub const e_coli_proline_pool_size_min: f64 = 0.95;
/// µmol / g
pub const e_coli_proline_pool_size_max: f64 = 1.25;
/// µmol / g
pub const e_coli_threonine_pool_size: f64 = 1.34;
/// µmol / g
pub const e_coli_threonine_pool_size_min: f64 = 1.50;
/// µmol / g
pub const e_coli_threonine_pool_size_max: f64 = 1.18;
/// µmol / g
pub const e_coli_tyrosine_pool_size: f64 = 0.41;
/// µmol / g
pub const e_coli_tyrosine_pool_size_min: f64 = 0.16;
/// µmol / g
pub const e_coli_tyrosine_pool_size_max: f64 = 0.66;
/// µmol / g
pub const e_coli_valine_pool_size: f64 = 2.41;
/// µmol / g
pub const e_coli_valine_pool_size_min: f64 = 2.14;
/// µmol / g
pub const e_coli_valine_pool_size_max: f64 = 2.68;
/// µmol / g
pub const e_coli_carbamoyl_aspartate_pool_size: f64 = 0.84;
/// µmol / g
pub const e_coli_carbamoyl_aspartate_pool_size_min: f64 = 0.56;
/// µmol / g
pub const e_coli_carbamoyl_aspartate_pool_size_max: f64 = 1.12;
/// µmol / g
pub const e_coli_imp_pool_size: f64 = 0.38;
/// µmol / g
pub const e_coli_imp_pool_size_min: f64 = 0.37;
/// µmol / g
pub const e_coli_imp_pool_size_max: f64 = 0.39;
/// %
pub const e_coli_percent_of_cell_total_dry_weight_that_is_lipopolysaccharide: f64 = 3.4;
/// Molecules / cell
pub const e_coli_reca_molecules_per_cell: f64 = 1000.0;
/// %
pub const e_coli_reca_molecules_per_cell_min: f64 = 800.0;
/// %
pub const e_coli_reca_molecules_per_cell_max: f64 = 1200.0;
/// µm^2/sec
pub const e_coli_apparent_diffusion_constant_of_protein_along_dna_segments: f64 = 0.4;
/// µm^2/sec
pub const e_coli_apparent_diffusion_constant_of_protein_along_dna_segments_min: f64 = 0.38;
/// µm^2/sec
pub const e_coli_apparent_diffusion_constant_of_protein_along_dna_segments_max: f64 = 0.42;
