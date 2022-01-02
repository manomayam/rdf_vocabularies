// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `WGS84 Geo Positioning: an RDF vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|WGS84 Geo Positioning: an RDF vocabulary|
//! |**Prefix**|wgs|
//! |**Namespace base IRI**|[http://www.w3.org/2003/01/geo/wgs84_pos#](http://www.w3.org/2003/01/geo/wgs84_pos#)|
//! |**Description**|A vocabulary for representing latitude, longitude and   altitude information in the WGS84 geodetic reference datum.   Version $Id: wgs84_pos.rdf,v 1.22 2009/04/20 15:00:30 timbl Exp $. See http://www.w3.org/2003/01/geo/ for more details.|
//! |**Is defined by**|[http://www.w3.org/2003/01/geo/wgs84_pos#](http://www.w3.org/2003/01/geo/wgs84_pos#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2003/01/geo/wgs84_pos#",;
    /// `point`: A point, typically described using a coordinate system relative to Earth, such as WGS84.<br>  
    Point, "Point",
    /// `SpatialThing`: Anything with spatial extent, i.e. size, shape, or position.<br> e.g. people, places, bowling balls, as well as abstract areas like cubes.<br>
    SpatialThing, "SpatialThing",
    /// `altitude`: The WGS84 altitude of a SpatialThing (decimal meters <br>above the local reference ellipsoid).
    alt, "alt",
    /// `latitude`: The WGS84 latitude of a SpatialThing (decimal degrees).
    lat, "lat",
    /// `lat/long`: A comma-separated representation of a latitude, longitude coordinate.
    lat_long, "lat_long",
    /// `location`: The relation between something and the point, <br> or other geometrical thing in space, where it is.  For example, the realtionship between<br> a radio tower and a Point with a given lat and long.<br> Or a relationship between a park and its outline as a closed arc of points, or a road and<br> its location as a arc (a sequence of points).<br> Clearly in practice there will be limit to the accuracy of any such statement, but one would expect<br> an accuracy appropriate for the size of the object and uses such as mapping .<br> 
    location, "location",
    /// `longitude`: The WGS84 longitude of a SpatialThing (decimal degrees).
    long, "long"
);
