// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `WGS84 Geo Positioning: an RDF vocabulary` vocabulary
//!
//! This module requires `ns-wgs` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|WGS84 Geo Positioning: an RDF vocabulary|
//! |**Prefix**|wgs|
//! |**Namespace base IRI**|<http://www.w3.org/2003/01/geo/wgs84_pos#>|
//! |**Description**|A vocabulary for representing latitude, longitude and   altitude information in the WGS84 geodetic reference datum.   Version $Id: wgs84_pos.rdf,v 1.22 2009/04/20 15:00:30 timbl Exp $. See http://www.w3.org/2003/01/geo/ for more details.|
//! |**Is defined by**|<http://www.w3.org/2003/01/geo/wgs84_pos#>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/2003/01/geo/wgs84_pos#",

    terms: [
        /// `point`:   Uniquely identified by lat/long/alt. i.e.  spaciallyIntersects(P1, P2) :- lat(P1, LAT), long(P1, LONG), alt(P1, ALT),   lat(P2, LAT), long(P2, LONG), alt(P2, ALT).  sameThing(P1, P2) :- type(P1, Point), type(P2, Point), spaciallyIntersects(P1, P2).
        (Point, "Point"),
        /// `SpatialThing`: Anything with spatial extent, i.e. size, shape, or position.  e.g. people, places, bowling balls, as well as abstract areas like cubes.
        (SpatialThing, "SpatialThing"),
        /// `altitude`: The WGS84 altitude of a SpatialThing (decimal meters  above the local reference ellipsoid).
        (alt, "alt"),
        /// `latitude`: The WGS84 latitude of a SpatialThing (decimal degrees).
        (lat, "lat"),
        /// `lat/long`: A comma-separated representation of a latitude, longitude coordinate.
        (lat_long, "lat_long"),
        /// `location`: The relation between something and the point,   or other geometrical thing in space, where it is.  For example, the realtionship between  a radio tower and a Point with a given lat and long.  Or a relationship between a park and its outline as a closed arc of points, or a road and  its location as a arc (a sequence of points).  Clearly in practice there will be limit to the accuracy of any such statement, but one would expect  an accuracy appropriate for the size of the object and uses such as mapping .
        (location, "location"),
        /// `longitude`: The WGS84 longitude of a SpatialThing (decimal degrees).
        (long, "long")    ]
);
