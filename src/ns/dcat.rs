// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The data catalog vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The data catalog vocabulary|
//! |**Prefix**|dcat|
//! |**Namespace base IRI**|[http://www.w3.org/ns/dcat#](http://www.w3.org/ns/dcat#)|
//! |**Description**|DCAT is an RDF vocabulary designed to facilitate interoperability between data catalogs published on the Web. By using DCAT to describe datasets in data catalogs, publishers increase discoverability and enable applications easily to consume metadata from multiple catalogs. It further enables decentralized publishing of catalogs and facilitates federated dataset search across sites. Aggregated DCAT metadata can serve as a manifest file to facilitate digital preservation. DCAT is defined at http://www.w3.org/TR/vocab-dcat/. Any variance between that normative document and this schema is an error in this schema.|
//! |**Is defined by**|[http://www.w3.org/ns/dcat#](http://www.w3.org/ns/dcat#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/dcat#",;
    Catalog, "Catalog",
    CatalogRecord, "CatalogRecord",
    DataService, "DataService",
    Dataset, "Dataset",
    Distribution, "Distribution",
    Relationship, "Relationship",
    Resource, "Resource",
    Role, "Role",
    accessService, "accessService",
    accessURL, "accessURL",
    bbox, "bbox",
    byteSize, "byteSize",
    catalog, "catalog",
    centroid, "centroid",
    compressFormat, "compressFormat",
    contactPoint, "contactPoint",
    dataset, "dataset",
    distribution, "distribution",
    downloadURL, "downloadURL",
    endDate, "endDate",
    endpointDescription, "endpointDescription",
    endpointURL, "endpointURL",
    hadRole, "hadRole",
    keyword, "keyword",
    landingPage, "landingPage",
    mediaType, "mediaType",
    packageFormat, "packageFormat",
    qualifiedRelation, "qualifiedRelation",
    record, "record",
    servesDataset, "servesDataset",
    service, "service",
    spatialResolutionInMeters, "spatialResolutionInMeters",
    startDate, "startDate",
    temporalResolution, "temporalResolution",
    theme, "theme",
    themeTaxonomy, "themeTaxonomy"
);
