// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-time")))]
//! This module provides terms for `OWL-Time` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|OWL-Time|
//! |**Prefix**|time|
//! |**Namespace base IRI**|<http://www.w3.org/2006/time#>|
//! |**Description**||
//! |**Is defined by**|<http://www.w3.org/2006/time#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2006/time#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `2006`: 
    N_2006, "2006",
    /// `2016`: 
    N_2016, "2016",
    /// `Date-Time description`: Description of date and time structured with separate values for the various elements of a calendar-clock system. The temporal reference system is fixed to Gregorian Calendar, and the range of year, month, day properties restricted to corresponding XML Schema types xsd:gYear, xsd:gMonth and xsd:gDay, respectively.
    DateTimeDescription, "DateTimeDescription",
    /// `Date-time interval`: DateTimeInterval is a subclass of ProperInterval, defined using the multi-element DateTimeDescription.
    DateTimeInterval, "DateTimeInterval",
    /// `Day of week`: The day of week
    DayOfWeek, "DayOfWeek",
    /// `Time duration`: Duration of a temporal extent expressed as a number scaled by a temporal unit
    Duration, "Duration",
    /// `Duration description`: Description of temporal extent structured with separate values for the various elements of a calendar-clock system. The temporal reference system is fixed to Gregorian Calendar, and the range of each of the numeric properties is restricted to xsd:decimal
    DurationDescription, "DurationDescription",
    /// `Friday`: 
    Friday, "Friday",
    /// `Generalized date-time description`: Description of date and time structured with separate values for the various elements of a calendar-clock system
    GeneralDateTimeDescription, "GeneralDateTimeDescription",
    /// `Generalized duration description`: Description of temporal extent structured with separate values for the various elements of a calendar-clock system.
    GeneralDurationDescription, "GeneralDurationDescription",
    /// `Time instant`: A temporal entity with zero extent or duration
    Instant, "Instant",
    /// `Time interval`: A temporal entity with an extent or duration
    Interval, "Interval",
    /// `January`: 
    January, "January",
    /// `Monday`: 
    Monday, "Monday",
    /// `Month of year`: The month of the year
    MonthOfYear, "MonthOfYear",
    /// `Proper interval`: A temporal entity with non-zero extent or duration, i.e. for which the value of the beginning and end are different
    ProperInterval, "ProperInterval",
    /// `Saturday`: 
    Saturday, "Saturday",
    /// `Sunday`: 
    Sunday, "Sunday",
    /// `Temporal Reference System`: A temporal reference system, such as a temporal coordinate system (with an origin, direction, and scale), a calendar-clock combination, or a (possibly hierarchical) ordinal system.   This is a stub class, representing the set of all temporal reference systems.
    TRS, "TRS",
    /// `Temporal duration`: Time extent; duration of a time interval separate from its particular start position
    TemporalDuration, "TemporalDuration",
    /// `Temporal entity`: A temporal interval or instant.
    TemporalEntity, "TemporalEntity",
    /// `Temporal position`: A position on a time-line
    TemporalPosition, "TemporalPosition",
    /// `Temporal unit`: A standard duration, which provides a scale factor for a time extent, or the granularity or precision for a time position.
    TemporalUnit, "TemporalUnit",
    /// `Thursday`: 
    Thursday, "Thursday",
    /// `Time position`: A temporal position described using either a (nominal) value from an ordinal reference system, or a (numeric) value in a temporal coordinate system. 
    TimePosition, "TimePosition",
    /// `Time Zone`: A Time Zone specifies the amount by which the local time is offset from UTC.  	A time zone is usually denoted geographically (e.g. Australian Eastern Daylight Time), with a constant value in a given region.  The region where it applies and the offset from UTC are specified by a locally recognised governing authority.
    TimeZone, "TimeZone",
    /// `Tuesday`: 
    Tuesday, "Tuesday",
    /// `Wednesday`: 
    Wednesday, "Wednesday",
    /// `Year`: Year duration
    Year, "Year",
    /// `after`: Gives directionality to time. If a temporal entity T1 is after another temporal entity T2, then the beginning of T1 is after the end of T2.
    after, "after",
    /// `before`: Gives directionality to time. If a temporal entity T1 is before another temporal entity T2, then the end of T1 is before the beginning of T2. Thus, "before" can be considered to be basic to instants and derived for intervals.
    before, "before",
    /// `day`: Day position in a calendar-clock system.  The range of this property is not specified, so can be replaced by any specific representation of a calendar day from any calendar. 
    day, "day",
    /// `day of week`: The day of week, whose value is a member of the class time:DayOfWeek
    dayOfWeek, "dayOfWeek",
    /// `day of year`: The number of the day within the year
    dayOfYear, "dayOfYear",
    /// `days duration`: length of, or element of the length of, a temporal extent expressed in days
    days, "days",
    /// `Generalized day`: Day of month - formulated as a text string with a pattern constraint to reproduce the same lexical form as gDay, except that values up to 99 are permitted, in order to support calendars with more than 31 days in a month.  Note that the value-space is not defined, so a generic OWL2 processor cannot compute ordering relationships of values of this type.
    generalDay, "generalDay",
    /// `Generalized month`: Month of year - formulated as a text string with a pattern constraint to reproduce the same lexical form as gMonth, except that values up to 20 are permitted, in order to support calendars with more than 12 months in the year.  Note that the value-space is not defined, so a generic OWL2 processor cannot compute ordering relationships of values of this type.
    generalMonth, "generalMonth",
    /// `Generalized year`: Year number - formulated as a text string with a pattern constraint to reproduce the same lexical form as gYear, but not restricted to values from the Gregorian calendar.  Note that the value-space is not defined, so a generic OWL2 processor cannot compute ordering relationships of values of this type.
    generalYear, "generalYear",
    /// `has beginning`: Beginning of a temporal entity
    hasBeginning, "hasBeginning",
    /// `has Date-Time description`: Value of DateTimeInterval expressed as a structured value. The beginning and end of the interval coincide with the limits of the shortest element in the description.
    hasDateTimeDescription, "hasDateTimeDescription",
    /// `has duration`: Duration of a temporal entity, expressed as a scaled value or nominal value
    hasDuration, "hasDuration",
    /// `has duration description`: Duration of a temporal entity, expressed using a structured description
    hasDurationDescription, "hasDurationDescription",
    /// `has end`: End of a temporal entity.
    hasEnd, "hasEnd",
    /// `Temporal reference system used`: The temporal reference system used by a temporal position or extent description. 
    hasTRS, "hasTRS",
    /// `has temporal duration`: Duration of a temporal entity.
    hasTemporalDuration, "hasTemporalDuration",
    /// `has time`: Supports the association of a temporal entity (instant or interval) to any thing
    hasTime, "hasTime",
    /// `has XSD duration`: Extent of a temporal entity, expressed using xsd:duration
    hasXSDDuration, "hasXSDDuration",
    /// `hour`: Hour position in a calendar-clock system.
    hour, "hour",
    /// `hours duration`: length of, or element of the length of, a temporal extent expressed in hours
    hours, "hours",
    /// `in date-time description`: Position of an instant, expressed using a structured description
    inDateTime, "inDateTime",
    /// `Temporal position`: Position of a time instant
    inTemporalPosition, "inTemporalPosition",
    /// `Time position`: Position of an instant, expressed as a temporal coordinate or nominal value
    inTimePosition, "inTimePosition",
    /// `in XSD date`: Position of an instant, expressed using xsd:date
    inXSDDate, "inXSDDate",
    /// `in XSD Date-Time`: Position of an instant, expressed using xsd:dateTime
    inXSDDateTime, "inXSDDateTime",
    /// `in XSD Date-Time-Stamp`: Position of an instant, expressed using xsd:dateTimeStamp
    inXSDDateTimeStamp, "inXSDDateTimeStamp",
    /// `in XSD g-Year`: Position of an instant, expressed using xsd:gYear
    inXSDgYear, "inXSDgYear",
    /// `in XSD g-YearMonth`: Position of an instant, expressed using xsd:gYearMonth
    inXSDgYearMonth, "inXSDgYearMonth",
    /// `has time instant inside`: An instant that falls inside the interval. It is not intended to include beginnings and ends of intervals.
    inside, "inside",
    /// `interval after`: If a proper interval T1 is intervalAfter another proper interval T2, then the beginning of T1 is after the end of T2.
    intervalAfter, "intervalAfter",
    /// `interval before`: If a proper interval T1 is intervalBefore another proper interval T2, then the end of T1 is before the beginning of T2.
    intervalBefore, "intervalBefore",
    /// `interval contains`: If a proper interval T1 is intervalContains another proper interval T2, then the beginning of T1 is before the beginning of T2, and the end of T1 is after the end of T2.
    intervalContains, "intervalContains",
    /// `interval disjoint`: If a proper interval T1 is intervalDisjoint another proper interval T2, then the beginning of T1 is after the end of T2, or the end of T1 is before the beginning of T2, i.e. the intervals do not overlap in any way, but their ordering relationship is not known.
    intervalDisjoint, "intervalDisjoint",
    /// `interval during`: If a proper interval T1 is intervalDuring another proper interval T2, then the beginning of T1 is after the beginning of T2, and the end of T1 is before the end of T2.
    intervalDuring, "intervalDuring",
    /// `interval equals`: If a proper interval T1 is intervalEquals another proper interval T2, then the beginning of T1 is coincident with the beginning of T2, and the end of T1 is coincident with the end of T2.
    intervalEquals, "intervalEquals",
    /// `interval finished by`: If a proper interval T1 is intervalFinishedBy another proper interval T2, then the beginning of T1 is before the beginning of T2, and the end of T1 is coincident with the end of T2.
    intervalFinishedBy, "intervalFinishedBy",
    /// `interval finishes`: If a proper interval T1 is intervalFinishes another proper interval T2, then the beginning of T1 is after the beginning of T2, and the end of T1 is coincident with the end of T2.
    intervalFinishes, "intervalFinishes",
    /// `interval in`: If a proper interval T1 is intervalIn another proper interval T2, then the beginning of T1 is after the beginning of T2 or is coincident with the beginning of T2, and the end of T1 is before the end of T2, or is coincident with the end of T2, except that end of T1 may not be coincident with the end of T2 if the beginning of T1 is coincident with the beginning of T2.
    intervalIn, "intervalIn",
    /// `interval meets`: If a proper interval T1 is intervalMeets another proper interval T2, then the end of T1 is coincident with the beginning of T2.
    intervalMeets, "intervalMeets",
    /// `interval met by`: If a proper interval T1 is intervalMetBy another proper interval T2, then the beginning of T1 is coincident with the end of T2.
    intervalMetBy, "intervalMetBy",
    /// `interval overlapped by`: If a proper interval T1 is intervalOverlappedBy another proper interval T2, then the beginning of T1 is after the beginning of T2, the beginning of T1 is before the end of T2, and the end of T1 is after the end of T2.
    intervalOverlappedBy, "intervalOverlappedBy",
    /// `interval overlaps`: If a proper interval T1 is intervalOverlaps another proper interval T2, then the beginning of T1 is before the beginning of T2, the end of T1 is after the beginning of T2, and the end of T1 is before the end of T2.
    intervalOverlaps, "intervalOverlaps",
    /// `interval started by`: If a proper interval T1 is intervalStarted another proper interval T2, then the beginning of T1 is coincident with the beginning of T2, and the end of T1 is after the end of T2.
    intervalStartedBy, "intervalStartedBy",
    /// `interval starts`: If a proper interval T1 is intervalStarts another proper interval T2, then the beginning of T1 is coincident with the beginning of T2, and the end of T1 is before the end of T2.
    intervalStarts, "intervalStarts",
    /// `minute`: Minute position in a calendar-clock system.
    minute, "minute",
    /// `minutes`: length, or element of, a temporal extent expressed in minutes
    minutes, "minutes",
    /// `month`: Month position in a calendar-clock system.  The range of this property is not specified, so can be replaced by any specific representation of a calendar month from any calendar. 
    month, "month",
    /// `month of year`: The month of the year, whose value is a member of the class time:MonthOfYear
    monthOfYear, "monthOfYear",
    /// `months duration`: length of, or element of the length of, a temporal extent expressed in months
    months, "months",
    /// `Name of temporal position`: The (nominal) value indicating temporal position in an ordinal reference system 
    nominalPosition, "nominalPosition",
    /// `Numeric value of temporal duration`: Value of a temporal extent expressed as a decimal number scaled by a temporal unit
    numericDuration, "numericDuration",
    /// `Numeric value of temporal position`: The (numeric) value indicating position within a temporal coordinate system 
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
    /// `temporal unit type`: The temporal unit which provides the precision of a date-time value or scale of a temporal extent
    unitType, "unitType",
    /// `Week (unit of temporal duration)`: 
    unitWeek, "unitWeek",
    /// `Year (unit of temporal duration)`: 
    unitYear, "unitYear",
    /// `week`: Week number within the year.
    week, "week",
    /// `weeks duration`: length of, or element of the length of, a temporal extent expressed in weeks
    weeks, "weeks",
    /// `has XSD date-time`: Value of DateTimeInterval expressed as a compact value.
    xsdDateTime, "xsdDateTime",
    /// `year`: Year position in a calendar-clock system.  The range of this property is not specified, so can be replaced by any specific representation of a calendar year from any calendar. 
    year, "year",
    /// `years duration`: length of, or element of the length of, a temporal extent expressed in years
    years, "years"
);
