// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
use rio_turtle::{NQuadsParser, TurtleError};
use rdf_utils::models::arc::IndexedArcDataset;
use sophia_api::quad::stream::QuadSource;
use sophia_rio::parser::StrictRioSource;
use sophia_turtle::parser::nq;

/// Static str content of ontology dataset, encoded as nquads
pub static DATASET_CONTENT: &str = include_str!("../../ontologies/grddl.nq");

/// It returns an instance of a type that implements [`QuadSource`](QuadSource) trait.
/// We can collect the quads from this instance to any type that implements [`Dataset`](sophia_api::dataset::Dataset) trait using [`.collect_quads`](QuadSource::collect_quads) method
pub fn get_quad_source() -> StrictRioSource<NQuadsParser<&'static [u8]>, TurtleError> {
    nq::parse_str(DATASET_CONTENT)
}

/// This method returns an instance of [`IndexedArcDataset`](IndexedArcDataset), which implements  [`Dataset`](sophia_api::dataset::Dataset)
pub fn get_indexed_dataset() -> IndexedArcDataset {
    match get_quad_source().collect_quads::<IndexedArcDataset>() {
        Ok(dataset) => dataset,
        Err(_) => unreachable!("If it is error, included static content must be faulty"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sophia_api::dataset::Dataset;

    fn validate_iris(dataset: IndexedArcDataset) {
        let iris_result = dataset.iris();
        claim::assert_ok!(&iris_result);
    }

    #[test]
    pub fn can_collect_into_dataset() {
        let dataset: IndexedArcDataset = get_indexed_dataset();
        validate_iris(dataset);
    }
}
