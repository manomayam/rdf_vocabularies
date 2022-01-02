#[cfg(feature = "dataset")]
pub mod dataset;
#[cfg(feature = "ns")]
pub mod ns;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
