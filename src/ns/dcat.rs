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
    /// `Katalog`: データ・カタログは、データセットに関するキュレートされたメタデータの集合です。
    Catalog, "Catalog",
    /// `Catalog Record`: Un registre du catalogue ou une entrée du catalogue, décrivant un seul jeu de données.
    CatalogRecord, "CatalogRecord",
    /// `Datatjeneste`: Umístění či přístupový bod poskytující operace související s hledáním, přistupem k, či výkonem funkcí na datech či souvisejících zdrojích.
    DataService, "DataService",
    /// `Jeu de données`: Μία συλλογή από δεδομένα, δημοσιευμένη ή επιμελημένη από μία και μόνο πηγή, διαθέσιμη δε προς πρόσβαση ή μεταφόρτωση σε μία ή περισσότερες μορφές.
    Dataset, "Dataset",
    /// `التوزيع`: Représente une forme spécifique d'un jeu de données. Caque jeu de données peut être disponible sous différentes formes, celles-ci pouvant représenter différents formats du jeu de données ou différents endpoint. Des exemples de distribution sont des fichirs CSV, des API ou des flux RSS.
    Distribution, "Distribution",
    /// `Vztah`: Una classe di associazione per il collegamento di informazioni aggiuntive a una relazione tra le risorse DCAT.
    Relationship, "Relationship",
    /// `Katalogiseret ressource`: Zdroj publikovaný či řízený jediným činitelem.
    Resource, "Resource",
    /// `Role`: Role je funkce zdroje či agenta ve vztahu k jinému zdroji, v kontextu přiřazení zdrojů či vztahů mezi zdroji.
    Role, "Role",
    /// `dataadgangstjeneste`: A site or end-point that gives access to the distribution of the dataset.
    accessService, "accessService",
    /// `přístupová adresa`: A URL of a resource that gives access to a distribution of the dataset. E.g. landing page, feed, SPARQL endpoint. Use for all cases except a simple download link, in which case downloadURL is preferred.
    accessURL, "accessURL",
    /// `quadro di delimitazione`: The geographic bounding box of a resource.
    bbox, "bbox",
    /// `byte size`: Velikost distribuce v bajtech.
    byteSize, "byteSize",
    /// `catálogo`: A catalog whose contents are of interest in the context of this catalog.
    catalog, "catalog",
    /// `centroid`: Il centro geografico (centroide) di una risorsa.
    centroid, "centroid",
    /// `compression format`: Kompressionsformatet for distributionen som indeholder data i et komprimeret format, fx for at reducere størrelsen af downloadfilen.
    compressFormat, "compressFormat",
    /// `point de contact`: Relevante kontaktoplysninger for den katalogiserede ressource. Anvendelse af vCard anbefales.
    contactPoint, "contactPoint",
    /// `datová sada`: Una raccolta di dati che è elencata nel catalogo.
    dataset, "dataset",
    /// `توزيع`: Una distribuzione disponibile per il set di dati.
    distribution, "distribution",
    /// `URL μεταφόρτωσης`: URL til fil der kan downloades i et bestemt format. Fx en CSV-fil eller en RDF-fil. Formatet for distributionen angives ved hjælp af egenskaberne dct:format og/eller dcat:mediaType.
    downloadURL, "downloadURL",
    /// `slutdato`: La fine del periodo.
    endDate, "endDate",
    /// `descrizione dell'endpoint del servizio`: Una descrizione dell'endpoint del servizio, incluse le sue operazioni, parametri, ecc.
    endpointDescription, "endpointDescription",
    /// `přístupový bod služby`: La posición raíz o end-point principal del servicio (una IRI web).
    endpointURL, "endpointURL",
    /// `hadRole`: The function of an entity or agent with respect to another entity or resource.
    hadRole, "hadRole",
    /// `klíčové slovo`: Una palabra clave o etiqueta que describe un recurso.
    keyword, "keyword",
    /// `pagina di destinazione`: Une page Web accessible par un navigateur Web donnant accès au catalogue, un jeu de données, ses distributions et/ou des informations additionnelles.
    landingPage, "landingPage",
    /// `medietype`: يجب استخدام هذه الخاصية إذا كان نوع الملف معرف ضمن IANA
    mediaType, "mediaType",
    /// `formato de empaquetado`: Format til pakning af data med henblik på distribution af en eller flere relaterede datafiler der samles til en enhed med henblik på samlet distribution. 
    packageFormat, "packageFormat",
    /// `qualified relation`: Enlace a una descripción de la relación con otro recurso.
    qualifiedRelation, "qualifiedRelation",
    /// `registro`: カタログの一部であるカタログ・レコード。
    record, "record",
    /// `poskytuje datovou sadu`: Una colección de datos que este Servicio de Datos puede distribuir.
    servesDataset, "servesDataset",
    /// `service`: Un sitio o 'endpoint' que está listado en el catálogo.
    service, "service",
    /// `spatial resolution (meters)`: minimální prostorový rozestup rozeznatelný v datové sadě, měřeno v metrech.
    spatialResolutionInMeters, "spatialResolutionInMeters",
    /// `startdato`: El comienzo del período
    startDate, "startDate",
    /// `časové rozlišení`: mindste tidsperiode der kan resolveres i datasættet.
    temporalResolution, "temporalResolution",
    /// `tema`: Et centralt emne for ressourcen. En ressource kan have flere centrale emner.
    theme, "theme",
    /// `Ταξινομία θεματικών κατηγοριών.`: Il sistema di organizzazione della conoscenza (KOS) usato per classificare i dataset del catalogo.
    themeTaxonomy, "themeTaxonomy"
);
