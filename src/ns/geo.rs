// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `geo` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|geo|
//! |**Namespace base IRI**|[http://www.opengis.net/ont/geosparql#](http://www.opengis.net/ont/geosparql#)|
//! |**Description**|An RDF/OWL vocabulary for representing spatial information|
//! |**Is defined by**|[http://www.modelservers.org/public/ontologies/vcon/geosparql_vocab_all.rdf](http://www.modelservers.org/public/ontologies/vcon/geosparql_vocab_all.rdf)|
//!

use crate::namespace;

namespace!(
    "http://www.opengis.net/ont/geosparql#",;
    /// `Feature`: <br>      This class represents the top-level feature type. This class is <br>      equivalent to GFI_Feature defined in ISO 19156:2011, and it is <br>      superclass of all feature types.<br>    
    Feature, "Feature",
    /// `Geometry`: <br>      The class represents the top-level geometry type. This class is <br>      equivalent to the UML class GM_Object defined in ISO 19107, and <br>      it is superclass of all geometry types.<br>    
    Geometry, "Geometry",
    /// `SpatialObject`: <br>      The class spatial-object represents everything that can have <br>      a spatial representation. It is superclass of feature and geometry.<br>    
    SpatialObject, "SpatialObject",
    /// `asGML`: <br>      The GML serialization of a geometry<br>    
    asGML, "asGML",
    /// `asWKT`: <br>      The WKT serialization of a geometry<br>    
    asWKT, "asWKT",
    /// `coordinateDimension`: <br>      The number of measurements or axes needed to describe the position of this<br>      geometry in a coordinate system.<br>    
    coordinateDimension, "coordinateDimension",
    /// `defaultGeometry`: <br>      The default geometry to be used in spatial calculations.<br>      It is Usually the most detailed geometry.<br>    
    defaultGeometry, "defaultGeometry",
    /// `dimension`: <br>      The topological dimension of this geometric object, which <br>      must be less than or equal to the coordinate dimension. <br>      In non-homogeneous collections, this will return the largest <br>      topological dimension of the contained objects.<br>    
    dimension, "dimension",
    /// `contains`: <br>      Exists if the subject SpatialObject spatially contains the <br>      object SpatialObject. DE-9IM: T*TFF*FF*<br>    
    ehContains, "ehContains",
    /// `coveredBy`: <br>      Exists if the subject SpatialObject is spatially covered <br>      by the object SpatialObject. DE-9IM: TFF*TFT**<br>    
    ehCoveredBy, "ehCoveredBy",
    /// `covers`: <br>      Exists if the subject SpatialObject spatially covers the <br>      object SpatialObject. DE-9IM: T*TFT*FF*<br>    
    ehCovers, "ehCovers",
    /// `disjoint`: <br>      Exists if the subject SpatialObject is spatially disjoint<br>      from the object SpatialObject. DE-9IM: FF*FF****<br>    
    ehDisjoint, "ehDisjoint",
    /// `equals`: <br>      Exists if the subject SpatialObject spatially equals the <br>      object SpatialObject. DE-9IM: TFFFTFFFT<br>    
    ehEquals, "ehEquals",
    /// `inside`: <br>      Exists if the subject SpatialObject is spatially inside <br>      the object SpatialObject. DE-9IM: TFF*FFT**<br>    
    ehInside, "ehInside",
    /// `meet`: <br>      Exists if the subject SpatialObject spatially meets the <br>      object SpatialObject. <br>      DE-9IM: FT******* ^ F**T***** ^ F***T****<br>    
    ehMeet, "ehMeet",
    /// `overlap`: <br>      Exists if the subject SpatialObject spatially overlaps the <br>      object SpatialObject. DE-9IM: T*T***T**<br>    
    ehOverlap, "ehOverlap",
    /// `GML Literal`: <br>      A GML serialization of a geometry object.<br>    
    gmlLiteral, "gmlLiteral",
    /// `hasGeometry`: <br>      A spatial representation for a given feature.<br>    
    hasGeometry, "hasGeometry",
    /// `has serialization`: <br>      Connects a geometry object with its text-based serialization.<br>    
    hasSerialization, "hasSerialization",
    /// `isEmpty`: <br>      (true) if this geometric object is the empty Geometry. If <br>      true, then this geometric object represents the empty point <br>      set for the coordinate space.<br>    
    isEmpty, "isEmpty",
    /// `isSimple`: <br>      (true) if this geometric object has no anomalous geometric <br>      points, such as self intersection or self tangency.<br>    
    isSimple, "isSimple",
    /// `disconnected`: <br>      Exists if the subject SpatialObject is spatially disjoint<br>      from the object SpatialObject. DE-9IM: FFTFFTTTT<br>    
    rcc8dc, "rcc8dc",
    /// `externally connected`: <br>      Exists if the subject SpatialObject spatially meets the <br>      object SpatialObject. DE-9IM: FFTFTTTTT<br>    
    rcc8ec, "rcc8ec",
    /// `equals`: <br>      Exists if the subject SpatialObject spatially equals the <br>      object SpatialObject. DE-9IM: TFFFTFFFT<br>    
    rcc8eq, "rcc8eq",
    /// `non-tangential proper part`: <br>      Exists if the subject SpatialObject is spatially inside <br>      the object SpatialObject. DE-9IM: TFFTFFTTT<br>    
    rcc8ntpp, "rcc8ntpp",
    /// `non-tangential proper part inverse`: <br>      Exists if the subject SpatialObject spatially contains the <br>      object SpatialObject. DE-9IM: TTTFFTFFT<br>    
    rcc8ntppi, "rcc8ntppi",
    /// `partially overlapping`: <br>      Exists if the subject SpatialObject spatially overlaps the <br>      object SpatialObject. DE-9IM: TTTTTTTTT<br>    
    rcc8po, "rcc8po",
    /// `tangential proper part`: <br>      Exists if the subject SpatialObject is spatially covered <br>      by the object SpatialObject. DE-9IM: TFFTTFTTT<br>    
    rcc8tpp, "rcc8tpp",
    /// `tangential proper part inverse`: <br>      Exists if the subject SpatialObject spatially covers the <br>      object SpatialObject. DE-9IM: TTTFTTFFT<br>    
    rcc8tppi, "rcc8tppi",
    /// `contains`: <br>      Exists if the subject SpatialObject spatially contains the <br>      object SpatialObject. DE-9IM: T*****FF*<br>    
    sfContains, "sfContains",
    /// `crosses`: <br>      Exists if the subject SpatialObject spatially crosses the <br>      object SpatialObject. DE-9IM: T*T******<br>    
    sfCrosses, "sfCrosses",
    /// `disjoint`: <br>      Exists if the subject SpatialObject is spatially disjoint <br>      from the object SpatialObject. DE-9IM: FF*FF****<br>    
    sfDisjoint, "sfDisjoint",
    /// `equals`: <br>      Exists if the subject SpatialObject spatially equals the <br>      object SpatialObject. DE-9IM: TFFFTFFFT<br>    
    sfEquals, "sfEquals",
    /// `intersects`: <br>      Exists if the subject SpatialObject is not spatially disjoint <br>      from the object SpatialObject.<br>      DE-9IM: T******** ^ *T******* ^ ***T***** ^ ****T**** <br>    
    sfIntersects, "sfIntersects",
    /// `overlaps`: <br>      Exists if the subject SpatialObject spatially overlaps the <br>      object SpatialObject. DE-9IM: T*T***T** <br>    
    sfOverlaps, "sfOverlaps",
    /// `touches`: <br>      Exists if the subject SpatialObject spatially touches the <br>      object SpatialObject.<br>      DE-9IM: FT******* ^ F**T***** ^ F***T****<br>    
    sfTouches, "sfTouches",
    /// `within`: <br>      Exists if the subject SpatialObject is spatially within the <br>      object SpatialObject. DE-9IM: T*F**F***<br>    
    sfWithin, "sfWithin",
    /// `spatialDimension`: <br>      The number of measurements or axes needed to describe the spatial position of <br>      this geometry in a coordinate system.<br>    
    spatialDimension, "spatialDimension",
    /// `Well-known Text Literal`: <br>      A Well-known Text serialization of a geometry object.<br>    
    wktLiteral, "wktLiteral"
);
