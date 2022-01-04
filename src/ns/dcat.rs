// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The data catalog vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The data catalog vocabulary|
//! |**Prefix**|dcat|
//! |**Namespace base IRI**|[<http://www.w3.org/ns/dcat#>](<http://www.w3.org/ns/dcat#>)|
//! |**Description**|DCAT is an RDF vocabulary designed to facilitate interoperability between data catalogs published on the Web. By using DCAT to describe datasets in data catalogs, publishers increase discoverability and enable applications easily to consume metadata from multiple catalogs. It further enables decentralized publishing of catalogs and facilitates federated dataset search across sites. Aggregated DCAT metadata can serve as a manifest file to facilitate digital preservation. DCAT is defined at http://www.w3.org/TR/vocab-dcat/. Any variance between that normative document and this schema is an error in this schema.|
//! |**Is defined by**|[<http://www.w3.org/ns/dcat#>](<http://www.w3.org/ns/dcat#>)|
//!

use crate::namespace;

namespace!(
    "<http://www.w3.org/ns/dcat#>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Catalog`: A curated collection of metadata about resources (e.g., datasets and data services in the context of a data catalog).
    Catalog, "Catalog",
    /// `Catalog Record`: A record in a data catalog, describing the registration of a single dataset or data service.
    CatalogRecord, "CatalogRecord",
    /// `Data service`: A site or end-point providing operations related to the discovery of, access to, or processing functions on, data or related resources.
    DataService, "DataService",
    /// `Dataset`: A collection of data, published or curated by a single source, and available for access or download in one or more representations.
    Dataset, "Dataset",
    /// `Distribution`: A specific representation of a dataset. A dataset might be available in multiple serializations that may differ in various ways, including natural language, media-type or format, schematic organization, temporal and spatial resolution, level of detail or profiles (which might specify any or all of the above).
    Distribution, "Distribution",
    /// `Relationship`: An association class for attaching additional information to a relationship between DCAT Resources.
    Relationship, "Relationship",
    /// `Catalogued resource`: Resource published or curated by a single agent.
    Resource, "Resource",
    /// `Role`: A role is the function of a resource or agent with respect to another resource, in the context of resource attribution or resource relationships.
    Role, "Role",
    /// `data access service`: A site or end-point that gives access to the distribution of the dataset.
    accessService, "accessService",
    /// `access address`: A URL of a resource that gives access to a distribution of the dataset. E.g. landing page, feed, SPARQL endpoint. Use for all cases except a simple download link, in which case downloadURL is preferred.
    accessURL, "accessURL",
    /// `bounding box`: 
    bbox, "bbox",
    /// `byte size`: The size of a distribution in bytes.
    byteSize, "byteSize",
    /// `catalog`: A catalog whose contents are of interest in the context of this catalog.
    catalog, "catalog",
    /// `centroid`: 
    centroid, "centroid",
    /// `compression format`: The compression format of the distribution in which the data is contained in a compressed form, e.g. to reduce the size of the downloadable file.
    compressFormat, "compressFormat",
    /// `contact point`: Relevant contact information for the catalogued resource. Use of vCard is recommended.
    contactPoint, "contactPoint",
    /// `dataset`: A collection of data that is listed in the catalog.
    dataset, "dataset",
    /// `distribution`: An available distribution of the dataset.
    distribution, "distribution",
    /// `download URL`: The URL of the downloadable file in a given format. E.g. CSV file or RDF file. The format is indicated by the distribution's dct:format and/or dcat:mediaType.
    downloadURL, "downloadURL",
    /// `end date`: 
    endDate, "endDate",
    /// `description of service end-point`: A description of the service end-point, including its operations, parameters etc.
    endpointDescription, "endpointDescription",
    /// `service end-point`: The root location or primary endpoint of the service (a web-resolvable IRI).
    endpointURL, "endpointURL",
    /// `hadRole`: The function of an entity or agent with respect to another entity or resource.
    hadRole, "hadRole",
    /// `keyword`: A keyword or tag describing a resource.
    keyword, "keyword",
    /// `landing page`: A Web page that can be navigated to in a Web browser to gain access to the catalog, a dataset, its distributions and/or additional information.
    landingPage, "landingPage",
    /// `media type`: The media type of the distribution as defined by IANA
    mediaType, "mediaType",
    /// `packaging format`: The package format of the distribution in which one or more data files are grouped together, e.g. to enable a set of related files to be downloaded together.
    packageFormat, "packageFormat",
    /// `qualified relation`: Link to a description of a relationship with another resource.
    qualifiedRelation, "qualifiedRelation",
    /// `record`: A record describing the registration of a single dataset or data service that is part of the catalog.
    record, "record",
    /// `serves dataset`: A collection of data that this DataService can distribute.
    servesDataset, "servesDataset",
    /// `service`: A site or endpoint that is listed in the catalog.
    service, "service",
    /// `geografisk opløsning (meter)`: mindste geografiske afstand som kan erkendes i et datasæt, målt i meter.
    spatialResolutionInMeters, "spatialResolutionInMeters",
    /// `start date`: 
    startDate, "startDate",
    /// `temporal resolution`: minimum time period resolvable in a dataset.
    temporalResolution, "temporalResolution",
    /// `theme`: A main category of the resource. A resource can have multiple themes.
    theme, "theme",
    /// `theme taxonomy`: The knowledge organization system (KOS) used to classify catalog's datasets.
    themeTaxonomy, "themeTaxonomy"
);
