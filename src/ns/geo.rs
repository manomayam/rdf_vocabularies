// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `geo` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|geo|
//! |**Namespace base IRI**|[<http://www.opengis.net/ont/geosparql#>](<http://www.opengis.net/ont/geosparql#>)|
//! |**Description**|An RDF/OWL vocabulary for representing spatial information|
//! |**Is defined by**|[<http://www.modelservers.org/public/ontologies/vcon/geosparql_vocab_all.rdf>](<http://www.modelservers.org/public/ontologies/vcon/geosparql_vocab_all.rdf>)|
//!

use crate::namespace;

namespace!(
    "<http://www.opengis.net/ont/geosparql#>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Feature`:        This class represents the top-level feature type. This class is        equivalent to GFI_Feature defined in ISO 19156:2011, and it is        superclass of all feature types.     
    Feature, "Feature",
    /// `Geometry`:        The class represents the top-level geometry type. This class is        equivalent to the UML class GM_Object defined in ISO 19107, and        it is superclass of all geometry types.     
    Geometry, "Geometry",
    /// `SpatialObject`:        The class spatial-object represents everything that can have        a spatial representation. It is superclass of feature and geometry.     
    SpatialObject, "SpatialObject",
    /// `asGML`:        The GML serialization of a geometry     
    asGML, "asGML",
    /// `asWKT`:        The WKT serialization of a geometry     
    asWKT, "asWKT",
    /// `coordinateDimension`:        The number of measurements or axes needed to describe the position of this       geometry in a coordinate system.     
    coordinateDimension, "coordinateDimension",
    /// `defaultGeometry`:        The default geometry to be used in spatial calculations.       It is Usually the most detailed geometry.     
    defaultGeometry, "defaultGeometry",
    /// `dimension`:        The topological dimension of this geometric object, which        must be less than or equal to the coordinate dimension.        In non-homogeneous collections, this will return the largest        topological dimension of the contained objects.     
    dimension, "dimension",
    /// `contains`:        Exists if the subject SpatialObject spatially contains the        object SpatialObject. DE-9IM: T*TFF*FF*     
    ehContains, "ehContains",
    /// `coveredBy`:        Exists if the subject SpatialObject is spatially covered        by the object SpatialObject. DE-9IM: TFF*TFT**     
    ehCoveredBy, "ehCoveredBy",
    /// `covers`:        Exists if the subject SpatialObject spatially covers the        object SpatialObject. DE-9IM: T*TFT*FF*     
    ehCovers, "ehCovers",
    /// `disjoint`:        Exists if the subject SpatialObject is spatially disjoint       from the object SpatialObject. DE-9IM: FF*FF****     
    ehDisjoint, "ehDisjoint",
    /// `equals`:        Exists if the subject SpatialObject spatially equals the        object SpatialObject. DE-9IM: TFFFTFFFT     
    ehEquals, "ehEquals",
    /// `inside`:        Exists if the subject SpatialObject is spatially inside        the object SpatialObject. DE-9IM: TFF*FFT**     
    ehInside, "ehInside",
    /// `meet`:        Exists if the subject SpatialObject spatially meets the        object SpatialObject.        DE-9IM: FT******* ^ F**T***** ^ F***T****     
    ehMeet, "ehMeet",
    /// `overlap`:        Exists if the subject SpatialObject spatially overlaps the        object SpatialObject. DE-9IM: T*T***T**     
    ehOverlap, "ehOverlap",
    /// `GML Literal`:        A GML serialization of a geometry object.     
    gmlLiteral, "gmlLiteral",
    /// `hasGeometry`:        A spatial representation for a given feature.     
    hasGeometry, "hasGeometry",
    /// `has serialization`:        Connects a geometry object with its text-based serialization.     
    hasSerialization, "hasSerialization",
    /// `isEmpty`:        (true) if this geometric object is the empty Geometry. If        true, then this geometric object represents the empty point        set for the coordinate space.     
    isEmpty, "isEmpty",
    /// `isSimple`:        (true) if this geometric object has no anomalous geometric        points, such as self intersection or self tangency.     
    isSimple, "isSimple",
    /// `disconnected`:        Exists if the subject SpatialObject is spatially disjoint       from the object SpatialObject. DE-9IM: FFTFFTTTT     
    rcc8dc, "rcc8dc",
    /// `externally connected`:        Exists if the subject SpatialObject spatially meets the        object SpatialObject. DE-9IM: FFTFTTTTT     
    rcc8ec, "rcc8ec",
    /// `equals`:        Exists if the subject SpatialObject spatially equals the        object SpatialObject. DE-9IM: TFFFTFFFT     
    rcc8eq, "rcc8eq",
    /// `non-tangential proper part`:        Exists if the subject SpatialObject is spatially inside        the object SpatialObject. DE-9IM: TFFTFFTTT     
    rcc8ntpp, "rcc8ntpp",
    /// `non-tangential proper part inverse`:        Exists if the subject SpatialObject spatially contains the        object SpatialObject. DE-9IM: TTTFFTFFT     
    rcc8ntppi, "rcc8ntppi",
    /// `partially overlapping`:        Exists if the subject SpatialObject spatially overlaps the        object SpatialObject. DE-9IM: TTTTTTTTT     
    rcc8po, "rcc8po",
    /// `tangential proper part`:        Exists if the subject SpatialObject is spatially covered        by the object SpatialObject. DE-9IM: TFFTTFTTT     
    rcc8tpp, "rcc8tpp",
    /// `tangential proper part inverse`:        Exists if the subject SpatialObject spatially covers the        object SpatialObject. DE-9IM: TTTFTTFFT     
    rcc8tppi, "rcc8tppi",
    /// `contains`:        Exists if the subject SpatialObject spatially contains the        object SpatialObject. DE-9IM: T*****FF*     
    sfContains, "sfContains",
    /// `crosses`:        Exists if the subject SpatialObject spatially crosses the        object SpatialObject. DE-9IM: T*T******     
    sfCrosses, "sfCrosses",
    /// `disjoint`:        Exists if the subject SpatialObject is spatially disjoint        from the object SpatialObject. DE-9IM: FF*FF****     
    sfDisjoint, "sfDisjoint",
    /// `equals`:        Exists if the subject SpatialObject spatially equals the        object SpatialObject. DE-9IM: TFFFTFFFT     
    sfEquals, "sfEquals",
    /// `intersects`:        Exists if the subject SpatialObject is not spatially disjoint        from the object SpatialObject.       DE-9IM: T******** ^ *T******* ^ ***T***** ^ ****T****      
    sfIntersects, "sfIntersects",
    /// `overlaps`:        Exists if the subject SpatialObject spatially overlaps the        object SpatialObject. DE-9IM: T*T***T**      
    sfOverlaps, "sfOverlaps",
    /// `touches`:        Exists if the subject SpatialObject spatially touches the        object SpatialObject.       DE-9IM: FT******* ^ F**T***** ^ F***T****     
    sfTouches, "sfTouches",
    /// `within`:        Exists if the subject SpatialObject is spatially within the        object SpatialObject. DE-9IM: T*F**F***     
    sfWithin, "sfWithin",
    /// `spatialDimension`:        The number of measurements or axes needed to describe the spatial position of        this geometry in a coordinate system.     
    spatialDimension, "spatialDimension",
    /// `Well-known Text Literal`:        A Well-known Text serialization of a geometry object.     
    wktLiteral, "wktLiteral"
);
