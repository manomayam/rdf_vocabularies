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
    /// `Agency`: Una empresa opera un cierto modo de transporte de manera programada
    Agency, "Agency",
    /// `Antes de subir a bordo`: La tarifa se debe pagar antes de subir a bordo
    BeforeBoarding, "BeforeBoarding",
    /// `Autobús`: Utilizado para rutas en autobús de corta y larga distancia.
    Bus, "Bus",
    /// `Cable car`: Used for street-level cable cars where the cable runs beneath the car.
    CableCar, "CableCar",
    /// `Regla de fecha de calendario`: Indica la disponibilidad de servicio para una fecha concreta
    CalendarDateRule, "CalendarDateRule",
    /// `Regla de calendario`: Define en qué días de la semana el servicio está disponible para un periodo concreto.
    CalendarRule, "CalendarRule",
    /// `Check the parent station`: Check the (parent) station for accessibility information.
    CheckParentStation, "CheckParentStation",
    /// `Drop Off Type`: Instancias de esta clase pueden ser usadas por gtfs:dropOffType. Indica si los pasajeros se bajan en una parada como parte del horario normal o si no pueden bajar en esa parada.
    DropOffType, "DropOffType",
    /// `Ensured Transfer`: This is a timed transfer point between two routes. The departing vehicle is expected to wait for the arriving one, with sufficient time for a passenger to transfer between routes.
    EnsuredTransfer, "EnsuredTransfer",
    /// `Clase tarifaria`: A class describing how the fare is calculated.
    FareClass, "FareClass",
    /// `Regla tarifaria`: Una regla que enlaza una clase tarifaria (gtfs:FareClass) a un zona de la red de transporte.
    FareRule, "FareRule",
    /// `GTFS Feed`: Una instancia de gtfs:Feed es un feed GTFS enlazado que cumple con esta especificación.
    Feed, "Feed",
    /// `Ferry`: Used for short- and long-distance boat service.
    Ferry, "Ferry",
    /// `Frecuencia`: An alternative to defining departures and arrivals as StopTimes: define frequencies for transit vehicle
    Frequency, "Frequency",
    /// `Funicular`: Cualquier sistema por raíl diseñado para recorridos con una gran inclinación.
    Funicular, "Funicular",
    /// `Teleférico, telecabina`: Gondola, Suspended cable car. Typically used for aerial cable cars where the car is suspended from the cable.
    Gondola, "Gondola",
    /// `Tranvía, metro ligero`: Cualquier metro ligero o sistema ferroviario en superficie dentro de un área metropolitana.
    LightRail, "LightRail",
    /// `Mínimo tiempo de transbordo`: This transfer requires a minimum amount of time between arrival and departure to ensure a connection. The time required to transfer is specified by gtfs:minimumTransferTime
    MinimumTimeTransfer, "MinimumTimeTransfer",
    /// `Must coordinate with driver to arrange pickup or drop off`: 
    MustCoordinateWithDriver, "MustCoordinateWithDriver",
    /// `Se debe llamar a la empresa para organizar la parada.`: 
    MustPhone, "MustPhone",
    /// `Sin transbordo`: No es posible realizar transbordos entre rutas en esta ubicación.
    NoTransfer, "NoTransfer",
    /// `No se permiten transbordos`: Indicate that a passenger should buy a new ticket when transferring
    NoTransfersAllowed, "NoTransfersAllowed",
    /// `Sin parada disponible`: 
    NotAvailable, "NotAvailable",
    /// `Not wheelchair accessible`: Pasajeros en silla de ruedas no pueden acceder a este viaje o el acceso con silla de ruedas no es posible en esta parada y/o no hay una ruta accesible desde el exterior de la estación hasta la parada.
    NotWheelchairAccessible, "NotWheelchairAccessible",
    /// `A bordo`: La tarifa se paga a bordo
    OnBoard, "OnBoard",
    /// `One transfer allowed`: Indica que el pasajero puede hacer un transbordo con su billete
    OneTransfersAllowed, "OneTransfersAllowed",
    /// `Método de pago`: Method to pay for the public transit service
    PaymentMethod, "PaymentMethod",
    /// `Pickup Type`: Instances of this class can be used by gtfs:pickupType
    PickupType, "PickupType",
    /// `Rail`: Used for intercity or long-distance travel.
    Rail, "Rail",
    /// `Recommended Transfer`: Este es un punto de transbordo recomendado entre dos rutas
    RecommendedTransfer, "RecommendedTransfer",
    /// `Regularly scheduled pickup or drop off`: 
    Regular, "Regular",
    /// `Route`: A gtfs:Route is a commercial route followed entirely or partly by gtfs:Trips
    Route, "Route",
    /// `Route type`: Describe el tipo de transporte usado en una ruta
    RouteType, "RouteType",
    /// `Service`: A gtfs:Service identifies a set of dates when a service is available for one or more routes
    Service, "Service",
    /// `Service rule`: One or more service rules define a set of dates
    ServiceRule, "ServiceRule",
    /// `forma del recorrido, trayecto, itinerario`: A polygon formed by gtfs:ShapePoints
    Shape, "Shape",
    /// `Shape Point`: A geographic point within a gtfs:Shape
    ShapePoint, "ShapePoint",
    /// `Station`: A physical structure or area that contains one or more stop.
    Station, "Station",
    /// `Parada`: Un lugar donde los pasajeros suben o bajan de un vehículo de transporte público.
    Stop, "Stop",
    /// `Stop time`: Describe una hora de parada como parte de un viaje
    StopTime, "StopTime",
    /// `Subway, Metro`: Any underground rail system within a metropolitan area.
    Subway, "Subway",
    /// ``: 
    Transfer, "Transfer",
    /// `Regla de transbordo`: Define reglas adicionales para transbordos entre rutas.
    TransferRule, "TransferRule",
    /// `Transfer Type`: Instancias de esta clase pueden ser usadas para describir cómo gestionar las reglas de transbordo
    TransferType, "TransferType",
    /// `Tipo de transbordo permitido`: Instances of this class describe whether tickets remain valid when transferring
    TransfersAllowedType, "TransfersAllowedType",
    /// `Viaje`: A collection of gtfs:StopTimes followed by a transit vehicle
    Trip, "Trip",
    /// `Dos transbordos permitidos`: Indicate that a passenger should buy a new ticket when transferring for the third time
    TwoTransfersAllowed, "TwoTransfersAllowed",
    /// `Unlimited transfers allowed`: Indicate that unlimited transfers are permitted
    UnlimitedTransfersAllowed, "UnlimitedTransfersAllowed",
    /// `Accesible para silla de ruedas`: Indicates that the vehicle being used on this particular trip can accommodate at least one rider in a wheelchair, or identifies that wheelchair boardings are possible from the specified stop, some trips at this stop are accessible by wheelchair and if this stop has a parentStation, there exists some accessible path from outside the station to the specific stop.
    WheelchairAccessible, "WheelchairAccessible",
    /// `Información de accesibilidad para silla de ruedas`: A class whom's instances indicate how accessible a gtfs:Trip, gtfs:Stop or gtfs:Station is.
    WheelchairBoardingStatus, "WheelchairBoardingStatus",
    /// `Zone`: Zones are required if you want to provide fare information using gtfs:FareClass
    Zone, "Zone",
    /// `empresa`: links to an agency that this is part of.
    agency, "agency",
    /// `hora de llegada`: Check the original specification for special cases
    arrivalTime, "arrivalTime",
    /// `Bikes allowed`: 
    bikesAllowed, "bikesAllowed",
    /// `Bloque`: Identifies the block to which the trip within this feed belongs. A block consists of two or more sequential trips made using the same vehicle, where a passenger can transfer from one trip to the next just by staying in the vehicle.
    block, "block",
    /// `Stop Code`: A gtfs:code predicate defines the stop code for a stop.
    code, "code",
    /// `color`: A 6 character hexidecimal color (without #)
    color, "color",
    /// `Comentario de enlace`: A comment on the relation between the RDFS ontology and the CSV specification.
    comment, "comment",
    /// `incorporación de fecha`: A boolean whether to add (true) or remove (false) a date
    dateAddition, "dateAddition",
    /// `Departure time`: Check the original specification for special cases
    departureTime, "departureTime",
    /// `Destination Stop`: Link to gtfs:Stop to go to
    destinationStop, "destinationStop",
    /// `Destination Zone`: The destination zone where this applies
    destinationZone, "destinationZone",
    /// `Direction`: A binary property to indicate the direction the trip is going (e.g., outbound/inbound, center/airport)
    direction, "direction",
    /// `Distance Traveled`: Positions a stop as a distance from the first shape point. It represents a real distance traveled along the route in units such as feet or kilometers. This information allows the trip planner to determine how much of the shape to draw when showing part of a trip on the map. The values used for gtfs:distanceTraveled must increase along with gtfs:stopSequence: they cannot be used to show reverse travel along a route.
    distanceTraveled, "distanceTraveled",
    /// `tipo de bajada`: Which type of drop off
    dropOffType, "dropOffType",
    /// `hora final`: Indicates the time at which service changes to a different frequency (or ceases) at the first stop in the trip.
    endTime, "endTime",
    /// `fare URL`: La URL de una página web que permite a un usuario comprar online billetes u otros productos de transporte de esa empresa
    fareUrl, "fareUrl",
    /// `Friday`: Servicio disponible los viernes
    friday, "friday",
    /// `destino del viaje`: The gtfs:headsign property contains the text that appears on a sign that identifies the trip's destination to passengers.
    headsign, "headsign",
    /// `tiempo entre viajes`: Indica el período de tiempo (en segundos) entre salidas desde la misma parada (tiempo entre viajes) para este tipo de viaje, durante el intervalo de tiempo especificado mediante start_time y end_time.
    headwaySeconds, "headwaySeconds",
    /// `Long name`: Nombre completo de una ruta
    longName, "longName",
    /// `Minimum Transfer Time`: El tiempo mínimo de transbordo cuando gtfs:transferType es gtfs:MinimumTimeTransfer.
    minimumTransferTime, "minimumTransferTime",
    /// `lunes`: Service is available on Monday
    monday, "monday",
    /// `Origin Stop`: Link to gtfs:Stop to start from
    originStop, "originStop",
    /// `Origin Zone`: The origin zone where this applies
    originZone, "originZone",
    /// `Parent Station`: gtfs:parentStation identifies the station associated with the stop
    parentStation, "parentStation",
    /// `Método de pago`: el método de pago
    paymentMethod, "paymentMethod",
    /// `Pickup type`: Indica si se recogen los pasajeros en una parada como parte del horario normal o si su recogida en dicha parada no se encuentra disponible.
    pickupType, "pickupType",
    /// `Point Sequence`: The shape_pt_sequence field associates the latitude and longitude of a shape point with its sequence order along the shape. The values for shape_pt_sequence must be non-negative integers, and they must increase along the trip.
    pointSequence, "pointSequence",
    /// `Route`: Este viaje forma parte de esta gtfs:Route
    route, "route",
    /// `Route type`: Links to the type of vehicle operating on this route
    routeType, "routeType",
    /// `sábado`: Service is available on Saturday
    saturday, "saturday",
    /// `Servicio`: Follows this gtfs:Service.
    service, "service",
    /// `regla de servicio`: Reglas de servicio que definen un conjunto de fechas
    serviceRule, "serviceRule",
    /// `Shape`: a link to a shape the trip follows.
    shape, "shape",
    /// `Shape Point`: Enlaza a gtfs:ShapePoint
    shapePoint, "shapePoint",
    /// `Short name`: Nombre corto dado a una ruta o viaje
    shortName, "shortName",
    /// `hora de inicio`: Define la hora a la que empieza el servicio con la frecuencia especificada
    startTime, "startTime",
    /// `Stop`: Indicates that this has a certain stop.
    stop, "stop",
    /// `Stop sequence`: The stopSequence property identifies the order of the stops for a particular trip. The values must increase along the gtfs:Trip referenced in the gtfs:trip property.
    stopSequence, "stopSequence",
    /// `Sunday`: Servicio disponible los domingos
    sunday, "sunday",
    /// `Text color`: A 6 character hexidecimal color (without #) asigned to a text label.
    textColor, "textColor",
    /// `jueves`: Service is available on Thursday
    thursday, "thursday",
    /// `Time zone`: La zona horaria donde una persona u organización está situada.
    timeZone, "timeZone",
    /// `Transfer Expiry Time`: El tiempo que el billete es válido desde que se abandona el vehículo para hacer un transbordo
    transferExpiryTime, "transferExpiryTime",
    /// `Transfer Type`: 
    transferType, "transferType",
    /// `Transfers`: Validez del ticket para transbordos
    transfers, "transfers",
    /// `Trip`: Indica que este concepto es parte de cierto gtfs:Trip
    trip, "trip",
    /// `martes`: Service is available on Tuesday
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
