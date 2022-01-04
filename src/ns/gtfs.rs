// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `General Transit Feed Specification` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|General Transit Feed Specification|
//! |**Prefix**|gtfs|
//! |**Namespace base IRI**|[<http://vocab.gtfs.org/terms#>](<http://vocab.gtfs.org/terms#>)|
//! |**Description**|This ontology is a translation of the General Transit Feed Specification towards URIs. Its intended use is creating an exchange platform where the Linked GTFS model can be used as a start to get the right data into the right format. For semantics of the classes and properties, see https://developers.google.com/transit/gtfs/reference.|
//! |**Is defined by**|[<http://vocab.gtfs.org/terms#>](<http://vocab.gtfs.org/terms#>)|
//!

use crate::namespace;

namespace!(
    "<http://vocab.gtfs.org/terms#>",;
    /// `General Transit Feed Specification`: 
    NAMESPACE_BASE, "",
    /// `Agency`: An agency operates a certain schedule based transport mode
    Agency, "Agency",
    /// `Before Boarding`: Ticket should be bought before boarding
    BeforeBoarding, "BeforeBoarding",
    /// `Bus`: Used for short- and long-distance bus routes.
    Bus, "Bus",
    /// `Cable car`: Used for street-level cable cars where the cable runs beneath the car.
    CableCar, "CableCar",
    /// `Calendar date rule`: Defines service availability for a specific date
    CalendarDateRule, "CalendarDateRule",
    /// `Calendar rule`: Defines on which days of the week for a certain period the service is available.
    CalendarRule, "CalendarRule",
    /// `Check the parent station`: Check the (parent) station for accessibility information.
    CheckParentStation, "CheckParentStation",
    /// `Drop Off Type`: Instances of this class can be used by gtfs:dropOffType
    DropOffType, "DropOffType",
    /// `Ensured Transfer`: This is a timed transfer point between two routes. The departing vehicle is expected to wait for the arriving one, with sufficient time for a passenger to transfer between routes.
    EnsuredTransfer, "EnsuredTransfer",
    /// `Fare Class`: A class describing how the fare is calculated.
    FareClass, "FareClass",
    /// `Fare Rule`: A rule which binds a gtfs:FareClass to a part of the network.
    FareRule, "FareRule",
    /// `GTFS Feed`: An instance of a gtfs:Feed is a linked GTFS feed that complies to this specification.
    Feed, "Feed",
    /// `Ferry`: Used for short- and long-distance boat service.
    Ferry, "Ferry",
    /// `Frequency`: An alternative to defining departures and arrivals as StopTimes: define frequencies for transit vehicle
    Frequency, "Frequency",
    /// `Funicular`: Any rail system designed for steep inclines.
    Funicular, "Funicular",
    /// `Gondola, Suspended cable car`: Gondola, Suspended cable car. Typically used for aerial cable cars where the car is suspended from the cable.
    Gondola, "Gondola",
    /// `Tram, Streetcar, Light rail`: Any light rail or street level system within a metropolitan area.
    LightRail, "LightRail",
    /// `Minimum Time Transfer`: This transfer requires a minimum amount of time between arrival and departure to ensure a connection. The time required to transfer is specified by gtfs:minimumTransferTime
    MinimumTimeTransfer, "MinimumTimeTransfer",
    /// `Must coordinate with driver to arrange pickup or drop off`: 
    MustCoordinateWithDriver, "MustCoordinateWithDriver",
    /// `Must phone agency to arrange pickup or drop off`: 
    MustPhone, "MustPhone",
    /// `No Transfer`: Transfers are not possible between routes at this location.
    NoTransfer, "NoTransfer",
    /// `No transfers allowed`: Indicate that a passenger should buy a new ticket when transferring
    NoTransfersAllowed, "NoTransfersAllowed",
    /// `Pickup or drop off not available`: 
    NotAvailable, "NotAvailable",
    /// `Not wheelchair accessible`: No riders in wheelchairs can be accommodated on this trip or wheelchair boarding is not possible at this stop and/or there exists no accessible path from outside the station to the specific stop.
    NotWheelchairAccessible, "NotWheelchairAccessible",
    /// `On Board`: Fare should be bought on board
    OnBoard, "OnBoard",
    /// `One transfer allowed`: Indicate that a passenger should buy a new ticket when transferring a second time
    OneTransfersAllowed, "OneTransfersAllowed",
    /// `Payment Method`: Method to pay for the public transit service
    PaymentMethod, "PaymentMethod",
    /// `Pickup Type`: Instances of this class can be used by gtfs:pickupType
    PickupType, "PickupType",
    /// `Rail`: Used for intercity or long-distance travel.
    Rail, "Rail",
    /// `Recommended Transfer`: This is a recommended transfer point between two routes
    RecommendedTransfer, "RecommendedTransfer",
    /// `Regularly scheduled pickup or drop off`: 
    Regular, "Regular",
    /// `Route`: A gtfs:Route is a commercial route followed entirely or partly by gtfs:Trips
    Route, "Route",
    /// `Route type`: Describes the type of transportation used on a route
    RouteType, "RouteType",
    /// `Service`: A gtfs:Service identifies a set of dates when a service is available for one or more routes
    Service, "Service",
    /// `Service rule`: One or more service rules define a set of dates
    ServiceRule, "ServiceRule",
    /// `Shape`: A polygon formed by gtfs:ShapePoints
    Shape, "Shape",
    /// `Shape Point`: A geographic point within a gtfs:Shape
    ShapePoint, "ShapePoint",
    /// `Station`: A physical structure or area that contains one or more stop.
    Station, "Station",
    /// `Stop`: A location where passengers board or disembark from a transit vehicle.
    Stop, "Stop",
    /// `Stop time`: Describes a stop time as part of a trip
    StopTime, "StopTime",
    /// `Subway, Metro`: Any underground rail system within a metropolitan area.
    Subway, "Subway",
    /// `Transfer`: 
    Transfer, "Transfer",
    /// `Transfer Rule`: Define additional rules for making connections between routes.
    TransferRule, "TransferRule",
    /// `Transfer Type`: Instances of this class can be used to describe how to handle a transfer rule
    TransferType, "TransferType",
    /// `Transfers Allowed Type`: Instances of this class describe whether tickets remain valid when transferring
    TransfersAllowedType, "TransfersAllowedType",
    /// `Trip`: A collection of gtfs:StopTimes followed by a transit vehicle
    Trip, "Trip",
    /// `Two transfers allowed`: Indicate that a passenger should buy a new ticket when transferring for the third time
    TwoTransfersAllowed, "TwoTransfersAllowed",
    /// `Unlimited transfers allowed`: Indicate that unlimited transfers are permitted
    UnlimitedTransfersAllowed, "UnlimitedTransfersAllowed",
    /// `Wheelchair accessible`: Indicates that the vehicle being used on this particular trip can accommodate at least one rider in a wheelchair, or identifies that wheelchair boardings are possible from the specified stop, some trips at this stop are accessible by wheelchair and if this stop has a parentStation, there exists some accessible path from outside the station to the specific stop.
    WheelchairAccessible, "WheelchairAccessible",
    /// `Wheelchair Boarding Information`: A class whom's instances indicate how accessible a gtfs:Trip, gtfs:Stop or gtfs:Station is.
    WheelchairBoardingStatus, "WheelchairBoardingStatus",
    /// `Zone`: Zones are required if you want to provide fare information using gtfs:FareClass
    Zone, "Zone",
    /// `agency`: links to an agency that this is part of.
    agency, "agency",
    /// `Arrival time`: Check the original specification for special cases
    arrivalTime, "arrivalTime",
    /// `Bikes allowed`: 
    bikesAllowed, "bikesAllowed",
    /// `Block`: Identifies the block to which the trip within this feed belongs. A block consists of two or more sequential trips made using the same vehicle, where a passenger can transfer from one trip to the next just by staying in the vehicle.
    block, "block",
    /// `Stop Code`: A gtfs:code predicate defines the stop code for a stop.
    code, "code",
    /// `Color`: A 6 character hexidecimal color (without #)
    color, "color",
    /// `Mapping comment`: A comment on the relation between the RDFS ontology and the CSV specification.
    comment, "comment",
    /// `Date Addition`: A boolean whether to add (true) or remove (false) a date
    dateAddition, "dateAddition",
    /// `Departure time`: Check the original specification for special cases
    departureTime, "departureTime",
    /// `Destination Stop`: Link to gtfs:Stop to go to
    destinationStop, "destinationStop",
    /// `Destination Zone`: La zona de destino donde una clase tarifaria se aplica
    destinationZone, "destinationZone",
    /// `Direction`: A binary property to indicate the direction the trip is going (e.g., outbound/inbound, center/airport)
    direction, "direction",
    /// `Distance Traveled`: Positions a stop as a distance from the first shape point. It represents a real distance traveled along the route in units such as feet or kilometers. This information allows the trip planner to determine how much of the shape to draw when showing part of a trip on the map. The values used for gtfs:distanceTraveled must increase along with gtfs:stopSequence: they cannot be used to show reverse travel along a route.
    distanceTraveled, "distanceTraveled",
    /// `Drop off type`: Which type of drop off
    dropOffType, "dropOffType",
    /// `End Time`: Indicates the time at which service changes to a different frequency (or ceases) at the first stop in the trip.
    endTime, "endTime",
    /// `fare URL`: The URL of a web page that allows a rider to purchase tickets or other fare instruments for that agency online
    fareUrl, "fareUrl",
    /// `Friday`: Service is available on Friday
    friday, "friday",
    /// `Headsign`: The gtfs:headsign property contains the text that appears on a sign that identifies the trip's destination to passengers.
    headsign, "headsign",
    /// `Headway Seconds`: Indicates the time between departures from the same stop (headway) for this trip type, during the time interval specified by gtfs:startTime and gtfs:endTime.
    headwaySeconds, "headwaySeconds",
    /// `Long name`: Long name given to a route
    longName, "longName",
    /// `Minimum Transfer Time`: The minimum transfer time when the gtfs:transferType is gtfs:MinimumTimeTransfer.
    minimumTransferTime, "minimumTransferTime",
    /// `Monday`: Service is available on Monday
    monday, "monday",
    /// `Origin Stop`: Link to gtfs:Stop to start from
    originStop, "originStop",
    /// `Origin Zone`: The origin zone where this applies
    originZone, "originZone",
    /// `Parent Station`: gtfs:parentStation identifies the station associated with the stop
    parentStation, "parentStation",
    /// `Payment method`: The payment method
    paymentMethod, "paymentMethod",
    /// `Pickup type`: Which type of pickup
    pickupType, "pickupType",
    /// `Point Sequence`: The shape_pt_sequence field associates the latitude and longitude of a shape point with its sequence order along the shape. The values for shape_pt_sequence must be non-negative integers, and they must increase along the trip.
    pointSequence, "pointSequence",
    /// `Route`: This thing is applicable to this gtfs:Route
    route, "route",
    /// `Route type`: Links to the type of vehicle operating on this route
    routeType, "routeType",
    /// `Saturday`: Service is available on Saturday
    saturday, "saturday",
    /// `Service`: Follows this gtfs:Service.
    service, "service",
    /// `Service rule`: Service rules which define together a set of dates
    serviceRule, "serviceRule",
    /// `Shape`: a link to a shape the trip follows.
    shape, "shape",
    /// `Shape Point`: Links to a gtfs:ShapePoint
    shapePoint, "shapePoint",
    /// `Short name`: Short name given to a route or a trip
    shortName, "shortName",
    /// `Start Time`: Specifies the time at which service begins with the specified frequency
    startTime, "startTime",
    /// `Stop`: Indicates that this has a certain stop.
    stop, "stop",
    /// `Stop sequence`: The stopSequence property identifies the order of the stops for a particular trip. The values must increase along the gtfs:Trip referenced in the gtfs:trip property.
    stopSequence, "stopSequence",
    /// `Sunday`: Service is available on Sunday
    sunday, "sunday",
    /// `Text color`: A 6 character hexidecimal color (without #) asigned to a text label.
    textColor, "textColor",
    /// `Thursday`: Service is available on Thursday
    thursday, "thursday",
    /// `Time zone`: The timezone where a person or organisation is located.
    timeZone, "timeZone",
    /// `Transfer Expiry Time`: The time a ticket remains valid when having left the previous vehicle
    transferExpiryTime, "transferExpiryTime",
    /// `Transfer Type`: 
    transferType, "transferType",
    /// `Transfers`: The validity of the ticket when transferring
    transfers, "transfers",
    /// `Trip`: Indicates that this is part of a certain gtfs:Trip
    trip, "trip",
    /// `Tuesday`: Service is available on Tuesday
    tuesday, "tuesday",
    /// `Frequency uses exact times`: 
    usesExactTimes, "usesExactTimes",
    /// `Wednesday`: Service is available on Wednesday
    wednesday, "wednesday",
    /// `wheelchair boarding`: Wheelchair boardings are possible from the specified stop or station. If this stop is part of a station, there is also a wheelchair accessible path from outside the station towards the stop.
    wheelchairAccessible, "wheelchairAccessible",
    /// `Zone`: A gtfs:Stop defines the fare zone. Zones are required if you want to provide fare information using gtfs:FareClass.
    zone, "zone"
);
