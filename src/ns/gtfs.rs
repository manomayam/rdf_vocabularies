// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `General Transit Feed Specification` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|General Transit Feed Specification|
//! |**Prefix**|gtfs|
//! |**Namespace base IRI**|[http://vocab.gtfs.org/terms#](http://vocab.gtfs.org/terms#)|
//! |**Description**|This ontology is a translation of the General Transit Feed Specification towards URIs. Its intended use is creating an exchange platform where the Linked GTFS model can be used as a start to get the right data into the right format. For semantics of the classes and properties, see https://developers.google.com/transit/gtfs/reference.|
//! |**Is defined by**|[http://vocab.gtfs.org/terms#](http://vocab.gtfs.org/terms#)|
//!

use crate::namespace;

namespace!(
    "http://vocab.gtfs.org/terms#",;
    Agency, "Agency",
    BeforeBoarding, "BeforeBoarding",
    Bus, "Bus",
    CableCar, "CableCar",
    CalendarDateRule, "CalendarDateRule",
    CalendarRule, "CalendarRule",
    CheckParentStation, "CheckParentStation",
    DropOffType, "DropOffType",
    EnsuredTransfer, "EnsuredTransfer",
    FareClass, "FareClass",
    FareRule, "FareRule",
    Feed, "Feed",
    Ferry, "Ferry",
    Frequency, "Frequency",
    Funicular, "Funicular",
    Gondola, "Gondola",
    LightRail, "LightRail",
    MinimumTimeTransfer, "MinimumTimeTransfer",
    MustCoordinateWithDriver, "MustCoordinateWithDriver",
    MustPhone, "MustPhone",
    NoTransfer, "NoTransfer",
    NoTransfersAllowed, "NoTransfersAllowed",
    NotAvailable, "NotAvailable",
    NotWheelchairAccessible, "NotWheelchairAccessible",
    OnBoard, "OnBoard",
    OneTransfersAllowed, "OneTransfersAllowed",
    PaymentMethod, "PaymentMethod",
    PickupType, "PickupType",
    Rail, "Rail",
    RecommendedTransfer, "RecommendedTransfer",
    Regular, "Regular",
    Route, "Route",
    RouteType, "RouteType",
    Service, "Service",
    ServiceRule, "ServiceRule",
    Shape, "Shape",
    ShapePoint, "ShapePoint",
    Station, "Station",
    Stop, "Stop",
    StopTime, "StopTime",
    Subway, "Subway",
    Transfer, "Transfer",
    TransferRule, "TransferRule",
    TransferType, "TransferType",
    TransfersAllowedType, "TransfersAllowedType",
    Trip, "Trip",
    TwoTransfersAllowed, "TwoTransfersAllowed",
    UnlimitedTransfersAllowed, "UnlimitedTransfersAllowed",
    WheelchairAccessible, "WheelchairAccessible",
    WheelchairBoardingStatus, "WheelchairBoardingStatus",
    Zone, "Zone",
    agency, "agency",
    arrivalTime, "arrivalTime",
    bikesAllowed, "bikesAllowed",
    block, "block",
    code, "code",
    color, "color",
    comment, "comment",
    dateAddition, "dateAddition",
    departureTime, "departureTime",
    destinationStop, "destinationStop",
    destinationZone, "destinationZone",
    direction, "direction",
    distanceTraveled, "distanceTraveled",
    dropOffType, "dropOffType",
    endTime, "endTime",
    fareUrl, "fareUrl",
    friday, "friday",
    headsign, "headsign",
    headwaySeconds, "headwaySeconds",
    longName, "longName",
    minimumTransferTime, "minimumTransferTime",
    monday, "monday",
    originStop, "originStop",
    originZone, "originZone",
    parentStation, "parentStation",
    paymentMethod, "paymentMethod",
    pickupType, "pickupType",
    pointSequence, "pointSequence",
    route, "route",
    routeType, "routeType",
    saturday, "saturday",
    service, "service",
    serviceRule, "serviceRule",
    shape, "shape",
    shapePoint, "shapePoint",
    shortName, "shortName",
    startTime, "startTime",
    stop, "stop",
    stopSequence, "stopSequence",
    sunday, "sunday",
    textColor, "textColor",
    thursday, "thursday",
    timeZone, "timeZone",
    transferExpiryTime, "transferExpiryTime",
    transferType, "transferType",
    transfers, "transfers",
    trip, "trip",
    tuesday, "tuesday",
    usesExactTimes, "usesExactTimes",
    wednesday, "wednesday",
    wheelchairAccessible, "wheelchairAccessible",
    zone, "zone"
);