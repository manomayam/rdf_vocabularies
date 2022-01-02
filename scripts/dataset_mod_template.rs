use rio_turtle::{NQuadsParser, TurtleError};
use sophia_api::quad::stream::QuadSource;
use sophia_inmem::dataset::FastDataset;
use sophia_rio::parser::StrictRioSource;
use sophia_turtle::parser::nq;

/// Static str content of ontology dataset, encoded as nquads
pub static DATASET_CONTENT: &str = include_str!("../../ontologies/TEMPLATE_DATASET_PREFIX.nq");

/// It returns an instance of a type that implements [`QuadSource`](QuadSource) trait.
/// We can collect the quads from this instance to any type that implements [`Dataset`](sophia_api::dataset::Dataset) trait using [`.collect_quads`](QuadSource::collect_quads) method
pub fn get_quad_source() -> StrictRioSource<NQuadsParser<&'static [u8]>, TurtleError> {
    nq::parse_str(DATASET_CONTENT)
}

/// This method returns an instance of [`FastDataset`](FastDataset), which implements  [`Dataset`](sophia_api::dataset::Dataset)
pub fn get_fast_dataset() -> FastDataset {
    match get_quad_source().collect_quads::<FastDataset>() {
        Ok(dataset) => dataset,
        Err(_) => unreachable!("If it is error, included static content must be faulty"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sophia_api::dataset::Dataset;
    use sophia_inmem::dataset::FastDataset;

    fn validate_iris(dataset: FastDataset) {
        let iris_result = dataset.iris();
        claim::assert_ok!(&iris_result);
    }

    #[test]
    pub fn can_collect_into_dataset() {
        let dataset: FastDataset = get_fast_dataset();
        validate_iris(dataset);
    }
}
