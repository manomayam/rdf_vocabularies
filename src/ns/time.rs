// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `OWL-Time` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|OWL-Time|
//! |**Prefix**|time|
//! |**Namespace base IRI**|[http://www.w3.org/2006/time#](http://www.w3.org/2006/time#)|
//! |**Description**||
//! |**Is defined by**|[http://www.w3.org/2006/time#](http://www.w3.org/2006/time#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2006/time#",;
    /// ``: 
    2006, "2006",
    /// ``: 
    2016, "2016",
    /// `Date-Time description`: Description of date and time structured with separate values for the various elements of a calendar-clock system. The temporal reference system is fixed to Gregorian Calendar, and the range of year, month, day properties restricted to corresponding XML Schema types xsd:gYear, xsd:gMonth and xsd:gDay, respectively.
    DateTimeDescription, "DateTimeDescription",
    /// `Date-time interval`: 'intervalo de fecha-hora' es una subclase de 'intervalo propio', definida utilizando el multi-elemento 'descripción de fecha-hora'.
    DateTimeInterval, "DateTimeInterval",
    /// `Day of week`: The day of week
    DayOfWeek, "DayOfWeek",
    /// `duración de tiempo`: Duration of a temporal extent expressed as a number scaled by a temporal unit
    Duration, "Duration",
    /// `Duration description`: Descripción de extensión temporal estructurada con valores separados para los distintos elementos de un sistema de horario-calendario. El sistema de referencia temporal se fija al calendario gregoriano, y el intervalo de cada una de las propiedades numéricas se restringe a xsd:decimal.
    DurationDescription, "DurationDescription",
    /// `Friday`: 
    Friday, "Friday",
    /// `descripción de fecha-hora generalizada`: Description of date and time structured with separate values for the various elements of a calendar-clock system
    GeneralDateTimeDescription, "GeneralDateTimeDescription",
    /// `Generalized duration description`: Description of temporal extent structured with separate values for the various elements of a calendar-clock system.
    GeneralDurationDescription, "GeneralDurationDescription",
    /// `Time instant`: A temporal entity with zero extent or duration
    Instant, "Instant",
    /// `intervalo de tiempo`: A temporal entity with an extent or duration
    Interval, "Interval",
    /// `January`: 
    January, "January",
    /// `Monday`: 
    Monday, "Monday",
    /// `Month of year`: El mes del año.
    MonthOfYear, "MonthOfYear",
    /// `intervalo propio`: A temporal entity with non-zero extent or duration, i.e. for which the value of the beginning and end are different
    ProperInterval, "ProperInterval",
    /// `Saturday`: 
    Saturday, "Saturday",
    /// `Sunday`: 
    Sunday, "Sunday",
    /// `Temporal Reference System`: Un sistema de referencia temporal, tal como un sistema de coordenadas temporales (con un origen, una dirección y una escala), una combinación calendario-reloj, o un sistema ordinal (posiblemente jerárquico).<br>        Esta clase comodín representa el conjunto de todos los sistemas de referencia temporal.
    TRS, "TRS",
    /// `duración temporal`: Time extent; duration of a time interval separate from its particular start position
    TemporalDuration, "TemporalDuration",
    /// `entidad temporal`: A temporal interval or instant.
    TemporalEntity, "TemporalEntity",
    /// `Temporal position`: A position on a time-line
    TemporalPosition, "TemporalPosition",
    /// `unidad de tiempo`: Una duración estándar, que proporciona un factor de escala para una extensión de tiempo, o la granularidad o precisión para una posición de tiempo.
    TemporalUnit, "TemporalUnit",
    /// `Thursday`: 
    Thursday, "Thursday",
    /// `Time position`: A temporal position described using either a (nominal) value from an ordinal reference system, or a (numeric) value in a temporal coordinate system. 
    TimePosition, "TimePosition",
    /// `huso horario`: Un huso horario especifica la cantidad en que la hora local está desplazada con respecto a UTC.<br>        Un huso horario normalmente se denota geográficamente (p.ej. el horario de verano del este de Australia), con un valor constante en una región dada.<br>        La región donde aplica y el desplazamiento desde UTC las especifica una autoridad gubernamental localmente reconocida.
    TimeZone, "TimeZone",
    /// `Tuesday`: 
    Tuesday, "Tuesday",
    /// `Wednesday`: 
    Wednesday, "Wednesday",
    /// `Year`: Year duration
    Year, "Year",
    /// `after`: Gives directionality to time. If a temporal entity T1 is after another temporal entity T2, then the beginning of T1 is after the end of T2.
    after, "after",
    /// `antes`: Asume una dirección en el tiempo. Si una entidad temporal T1 está antes que otra entidad temporal T2, entonces el final de T1 está antes que el principio de T2. Así, "antes" se puede considerar básica para instantes y derivada para intervalos.
    before, "before",
    /// `day`: Day position in a calendar-clock system.<br><br>The range of this property is not specified, so can be replaced by any specific representation of a calendar day from any calendar. 
    day, "day",
    /// `día de la semana`: The day of week, whose value is a member of the class time:DayOfWeek
    dayOfWeek, "dayOfWeek",
    /// `día del año`: The number of the day within the year
    dayOfYear, "dayOfYear",
    /// `days duration`: length of, or element of the length of, a temporal extent expressed in days
    days, "days",
    /// `Generalized day`: Day of month - formulated as a text string with a pattern constraint to reproduce the same lexical form as gDay, except that values up to 99 are permitted, in order to support calendars with more than 31 days in a month. <br>Note that the value-space is not defined, so a generic OWL2 processor cannot compute ordering relationships of values of this type.
    generalDay, "generalDay",
    /// `Mes generalizado`: Mes del año - formulado como una cadena de texto con una restricción patrón para reproducir la misma forma léxica que gMonth, excepto que se permiten valores hasta el 20, con el propósito de proporcionar soporte a calendarios con años con más de 12 meses.<br>            Nótese que el espacio de valores no está definido, por tanto, un procesador genérico de OWL2 no puede computar relaciones de orden de valores de este tipo.
    generalMonth, "generalMonth",
    /// `Generalized year`: Número de año - formulado como una cadena de texto con una restricción patrón para reproducir la misma forma léxica que gYear, aunque no está restringido a valores del calendario gregoriano.<br>            Nótese que el espacio de valores no está definido, por tanto, un procesador genérico de OWL2 no puede computar relaciones de orden de valores de este tipo.
    generalYear, "generalYear",
    /// `tiene principio`: Beginning of a temporal entity
    hasBeginning, "hasBeginning",
    /// `has Date-Time description`: Value of DateTimeInterval expressed as a structured value. The beginning and end of the interval coincide with the limits of the shortest element in the description.
    hasDateTimeDescription, "hasDateTimeDescription",
    /// `has duration`: Duration of a temporal entity, expressed as a scaled value or nominal value
    hasDuration, "hasDuration",
    /// `has duration description`: Duración de una entidad temporal, expresada utilizando una descripción estructurada.
    hasDurationDescription, "hasDurationDescription",
    /// `tiene fin`: End of a temporal entity.
    hasEnd, "hasEnd",
    /// `sistema de referencia temporal utilizado`: El sistema de referencia temporal utilizado por una posición temporal o descripción de extensión.
    hasTRS, "hasTRS",
    /// `has temporal duration`: Duration of a temporal entity.
    hasTemporalDuration, "hasTemporalDuration",
    /// `tiene tiempo`: Proporciona soporte a la asociación de una entidad temporal (instante o intervalo) a cualquier cosa.
    hasTime, "hasTime",
    /// `tiene duración XSD`: Extent of a temporal entity, expressed using xsd:duration
    hasXSDDuration, "hasXSDDuration",
    /// `hour`: Posición de hora en un sistema calendario-reloj.
    hour, "hour",
    /// `hours duration`: Longitud de, o elemento de la longitud de, una extensión temporal expresada en horas.
    hours, "hours",
    /// `in date-time description`: Position of an instant, expressed using a structured description
    inDateTime, "inDateTime",
    /// `posición temporal`: Position of a time instant
    inTemporalPosition, "inTemporalPosition",
    /// `Time position`: Posición de un instante, expresada como una coordenada temporal o un valor nominal.
    inTimePosition, "inTimePosition",
    /// `in XSD date`: Posición de un instante, expresado utilizando xsd:date.
    inXSDDate, "inXSDDate",
    /// `in XSD Date-Time`: Posición de un instante, expresado utilizando xsd:dateTime.
    inXSDDateTime, "inXSDDateTime",
    /// `in XSD Date-Time-Stamp`: Posición de un instante, expresado utilizando xsd:dateTimeStamp.
    inXSDDateTimeStamp, "inXSDDateTimeStamp",
    /// `en año gregoriano XSD`: Posición de un instante, expresado utilizando xsd:gYear.
    inXSDgYear, "inXSDgYear",
    /// `in XSD g-YearMonth`: Position of an instant, expressed using xsd:gYearMonth
    inXSDgYearMonth, "inXSDgYearMonth",
    /// `has time instant inside`: An instant that falls inside the interval. It is not intended to include beginnings and ends of intervals.
    inside, "inside",
    /// `interval after`: If a proper interval T1 is intervalAfter another proper interval T2, then the beginning of T1 is after the end of T2.
    intervalAfter, "intervalAfter",
    /// `interval before`: Si un intervalo propio T1 está antes que otro intervalo propio T2, entonces el final de T1 está antes que el principio de T2.
    intervalBefore, "intervalBefore",
    /// `interval contains`: Si un intervalo propio T1 contiene otro intervalo propio T2, entonces el principio de T1 está antes que el principio de T2, y el final de T1 está después del final de T2.
    intervalContains, "intervalContains",
    /// `interval disjoint`: If a proper interval T1 is intervalDisjoint another proper interval T2, then the beginning of T1 is after the end of T2, or the end of T1 is before the beginning of T2, i.e. the intervals do not overlap in any way, but their ordering relationship is not known.
    intervalDisjoint, "intervalDisjoint",
    /// `interval during`: Si un intervalo propio T1 está durante otro intervalo propio T2, entonces del principio de T1 está después del principio de T2, y el final de T1 está antes que el final de T2.
    intervalDuring, "intervalDuring",
    /// `interval equals`: If a proper interval T1 is intervalEquals another proper interval T2, then the beginning of T1 is coincident with the beginning of T2, and the end of T1 is coincident with the end of T2.
    intervalEquals, "intervalEquals",
    /// `interval finished by`: Si un intervalo propio T1 está terminado por otro intervalo propio T2, entonces el principio de T1 está antes que el principio de T2, y el final de T1 coincide con el final de T2.
    intervalFinishedBy, "intervalFinishedBy",
    /// `intervalo termina`: Si un intervalo propio T1 termina otro intervalo propio T2, entonces del principio de T1 está después del principio de T2, y el final de T1 coincide con el final de T2.
    intervalFinishes, "intervalFinishes",
    /// `intervalo interior`: Si un intervalo propio T1 es un intervalo interior a otro intervalo propio T2, entonces el principio de T1 está después del principio de T2 o coincide con el principio de T2, y el final de T1 está antes que el final de T2, o coincide con el final de T2, excepto que el final de T1 puede no coincidir con el final de T2 si el principio de T1 coincide con el principio de T2.
    intervalIn, "intervalIn",
    /// `intervalo se encuentra`: Si un intervalo propio T1 se encuentra con otro intervalo propio T2, entonces el final de T1 coincide con el principio de T2.
    intervalMeets, "intervalMeets",
    /// `intervalo encontrado por`: Si un intervalo propio T1 es 'intervalo encontrado por' otro intervalo propio T2, entonces el principio de T1 coincide con el final de T2.
    intervalMetBy, "intervalMetBy",
    /// `intervalo solapado por`: Si un intervalo propio T1 es 'intervalo solapado por' otro intervalo propio T2, entonces el principio de T1 es posterior al principio de T2, y el principio de T1 es anterior al final de T2, y el final de T1 es posterior al final de T2.
    intervalOverlappedBy, "intervalOverlappedBy",
    /// `intervalo se solapa`: If a proper interval T1 is intervalOverlaps another proper interval T2, then the beginning of T1 is before the beginning of T2, the end of T1 is after the beginning of T2, and the end of T1 is before the end of T2.
    intervalOverlaps, "intervalOverlaps",
    /// `interval started by`: Si un intervalo propio T1 es empezado por otro intervalo propio T2, entonces el principio de T1 coincide con el principio de T2, y el final de T1 es posterior al final de T2.
    intervalStartedBy, "intervalStartedBy",
    /// `intervalo empieza`: If a proper interval T1 is intervalStarts another proper interval T2, then the beginning of T1 is coincident with the beginning of T2, and the end of T1 is before the end of T2.
    intervalStarts, "intervalStarts",
    /// `minuto`: Posición de minuto en un sistema calendario-reloj.
    minute, "minute",
    /// `minutos`: length, or element of, a temporal extent expressed in minutes
    minutes, "minutes",
    /// `mes`: Month position in a calendar-clock system.<br><br>The range of this property is not specified, so can be replaced by any specific representation of a calendar month from any calendar. 
    month, "month",
    /// `month of year`: El mes del año, cuyo valor es un miembro de la clase 'mes del año'.
    monthOfYear, "monthOfYear",
    /// `duración en meses`: Longitud de, o elemento de la longitud de, una extensión temporal expresada en meses.
    months, "months",
    /// `Name of temporal position`: The (nominal) value indicating temporal position in an ordinal reference system 
    nominalPosition, "nominalPosition",
    /// `Numeric value of temporal duration`: Valor de una extensión temporal expresada como un número decimal escalado por una unidad de tiempo.
    numericDuration, "numericDuration",
    /// `valor numérico de posición temporal`: El valor (numérico) que indica posición temporal en un sistema de referencia ordinal.
    numericPosition, "numericPosition",
    /// `second`: Second position in a calendar-clock system.
    second, "second",
    /// `seconds duration`: length of, or element of the length of, a temporal extent expressed in seconds
    seconds, "seconds",
    /// `in time zone`: The time zone for clock elements in the temporal position
    timeZone, "timeZone",
    /// `Day (unit of temporal duration)`: 
    unitDay, "unitDay",
    /// `Hour (unit of temporal duration)`: 
    unitHour, "unitHour",
    /// `Minute (unit of temporal duration)`: 
    unitMinute, "unitMinute",
    /// `Month (unit of temporal duration)`: 
    unitMonth, "unitMonth",
    /// `Second (unit of temporal duration)`: 
    unitSecond, "unitSecond",
    /// `temporal unit type`: La unidad de tiempo que proporciona la precisión de un valor fecha-hora o la escala de una extensión temporal.
    unitType, "unitType",
    /// `Week (unit of temporal duration)`: 
    unitWeek, "unitWeek",
    /// `Year (unit of temporal duration)`: 
    unitYear, "unitYear",
    /// `week`: Week number within the year.
    week, "week",
    /// `duración en semanas`: length of, or element of the length of, a temporal extent expressed in weeks
    weeks, "weeks",
    /// `has XSD date-time`: Valor de 'intervalo de fecha-hora' expresado como un valor compacto.
    xsdDateTime, "xsdDateTime",
    /// `year`: Year position in a calendar-clock system.<br><br>The range of this property is not specified, so can be replaced by any specific representation of a calendar year from any calendar. 
    year, "year",
    /// `years duration`: Longitud de, o elemento de la longitud de, una extensión temporal expresada en años.
    years, "years"
);
