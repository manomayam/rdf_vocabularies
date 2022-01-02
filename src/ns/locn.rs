// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `ISA Programme Location Core Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|ISA Programme Location Core Vocabulary|
//! |**Prefix**|locn|
//! |**Namespace base IRI**|[http://www.w3.org/ns/locn#](http://www.w3.org/ns/locn#)|
//! |**Description**|This is a new version of the final draft published by the European Commission in May 2012, revised according to the results of the ISA Core Location Pilot (see Section Change History for the list of changes). It is currently under the control of the Locations and Addresses Community Group, but is not under active development or review. Comments and queries should be sent to that group via public-locadd@w3.org. Terms defined here may be deprecated by that or future groups but will not disappear or their definition change.|
//! |**Is defined by**|[http://www.w3.org/ns/locn#](http://www.w3.org/ns/locn#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/locn#",;
    /// `Address`: An "address representation" as conceptually defined by the INSPIRE Address Representation data type. The locn:addressId property may be used to link this locn:Address to other representations.
    Address, "Address",
    /// `Geometry`: The locn:Geometry class provides the means to identify a location as a point, line, polygon, etc. expressed using coordinates in some coordinate reference system.
    Geometry, "Geometry",
    /// `address`: The locn:address property relationship associates any resource with the locn:Address class 
    address, "address",
    /// `address area`: The name or names of a geographic area or locality that groups a number of addressable objects for addressing purposes, without being an administrative unit. This would typically be part of a city, a neighbourhood or village. The domain of locn:addressArea is locn:Address.
    addressArea, "addressArea",
    /// `address ID`: The concept of adding a globally unique identifier for each instance of an address is a crucial part of the INSPIRE data spec. The domain of locn:addressId is locn:Address.
    addressId, "addressId",
    /// `admin unit level 1`: The uppermost administrative unit for the address, almost always a country. The domain of locn:adminUnitL1 is locn:Address and the range is a literal, conceptually defined by the INSPIRE Geographical Name data type.
    adminUnitL1, "adminUnitL1",
    /// `admin unit level 2`: The region of the address, usually a county, state or other such area that typically encompasses several localities. The domain of  locn:adminUnitL2 is locn:Address and the range is a literal, conceptually defined by the INSPIRE Geographical Name data type.
    adminUnitL2, "adminUnitL2",
    /// `full address`: The complete address written as a string, with or without formatting. The domain of locn:fullAddress is locn:Address.
    fullAddress, "fullAddress",
    /// `geographic name`: <br>A geographic name is a proper noun applied to a spatial object. Taking the example used in the relevant INSPIRE data specification (page 18), the following are all valid geographic names for the Greek capital:<br>- Αθήνα (the Greek endonym written in the Greek script)<br>- Athína (the standard Romanisation of the endonym)<br>- Athens (the English language exonym)<br>For INSPIRE-conformant data, provide the metadata for the geographic name using a skos:Concept as a datatype.<br>
    geographicName, "geographicName",
    /// `geometry`: Associates any resource with the corresponding geometry.
    geometry, "geometry",
    /// `location`: The location property links any resource to the Location Class. Asserting the location relationship implies only that the domain has some connection to a Location in time or space. It does not imply that the resource is necessarily at that location at the time when the assertion is made.
    location, "location",
    /// `locator designator`: A number or a sequence of characters that uniquely identifies the locator within the relevant scope(s). The full identification of the locator could include one or more locator designators.<br>    
    locatorDesignator, "locatorDesignator",
    /// `locator name`: Proper noun(s) applied to the real world entity identified by the locator. The locator name could be the name of the property or complex, of the building or part of the building, or it could be the name of a room inside a building. <br>    
    locatorName, "locatorName",
    /// `PO box`: The Post Office Box number. The domain of locn:poBox is locn:Address.
    poBox, "poBox",
    /// `post code`: The post code (a.k.a postal code, zip code etc.). Post codes are common elements in many countries' postal address systems. The domain of locn:postCode is locn:Address.
    postCode, "postCode",
    /// `post name`: The key postal division of the address, usually the city. (INSPIRE's definition is "One or more names created and maintained for postal purposes to identify a subdivision of addresses and postal delivery points."). The domain of locn:postName is locn:Address.
    postName, "postName",
    /// `thoroughfare`: An address component that represents the name of a passage or way through from one location to another. A thoroughfare is not necessarily a road, it might be a waterway or some other feature. The domain of locn:thoroughfare is locn:Address.
    thoroughfare, "thoroughfare"
);
