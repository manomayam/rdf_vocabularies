// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `ical` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|ical|
//! |**Namespace base IRI**|[http://www.w3.org/2002/12/cal/icaltzd#](http://www.w3.org/2002/12/cal/icaltzd#)|
//! |**Description**||
//! |**Is defined by**|[http://www.w3.org/2002/12/cal/icaltzd.rdf](http://www.w3.org/2002/12/cal/icaltzd.rdf)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2002/12/cal/icaltzd#",;
    /// ``: 
    DomainOf_rrule, "DomainOf_rrule",
    /// ``: 
    List_of_Float, "List_of_Float",
    /// ``: Provide a grouping of component properties that define an alarm.
    Valarm, "Valarm",
    /// ``: 
    Value_CAL_ADDRESS, "Value_CAL-ADDRESS",
    /// ``: 
    Value_DATE, "Value_DATE",
    /// ``: 
    Value_DATE_TIME, "Value_DATE-TIME",
    /// ``: 
    Value_DURATION, "Value_DURATION",
    /// ``: 
    Value_PERIOD, "Value_PERIOD",
    /// ``: 
    Value_RECUR, "Value_RECUR",
    /// `VCALENDAR`: 
    Vcalendar, "Vcalendar",
    /// `Event`: Provide a grouping of component properties that describe an event.
    Vevent, "Vevent",
    /// ``: Provide a grouping of component properties that describe either a request for free/busy time, describe a response to a request for free/busy time or describe a published set of busy time.
    Vfreebusy, "Vfreebusy",
    /// ``: Provide a grouping of component properties that describe a journal entry.
    Vjournal, "Vjournal",
    /// ``: Provide a grouping of component properties that defines a time zone.
    Vtimezone, "Vtimezone",
    /// `To-do`: Provide a grouping of calendar properties that describe a to-do.
    Vtodo, "Vtodo",
    /// ``: This class of property provides a framework for defining non-standard properties.
    X_, "X-",
    /// ``: This property defines the action to be invoked when an alarm is triggered.
    action, "action",
    /// ``: To specify an alternate text representation for the property value.
    altrep, "altrep",
    /// ``: The property provides the capability to associate a document object with a calendar component.
    attach, "attach",
    /// `attendee`: <br>	    value type: CAL-ADDRESS
    attendee, "attendee",
    /// `BYDAY`: 
    byday, "byday",
    /// `BYHOUR`: 
    byhour, "byhour",
    /// `BYMINUTE`: 
    byminute, "byminute",
    /// `BYMONTH`: 
    bymonth, "bymonth",
    /// `BYSECOND`: 
    bysecond, "bysecond",
    /// `BYSETPOS`: 
    bysetpos, "bysetpos",
    /// `BYWEEKNO`: 
    byweekno, "byweekno",
    /// `BYYEARDAY`: 
    byyearday, "byyearday",
    /// ``: 
    calAddress, "calAddress",
    /// ``: This property defines the calendar scale used for the calendar information specified in the iCalendar object.
    calscale, "calscale",
    /// ``: <br>	    value type: TEXT
    categories, "categories",
    /// ``: <br>	    value type: TEXT
    class, "class",
    /// ``: To specify the common name to be associated with the calendar user specified by the property.
    cn, "cn",
    /// ``: This property specifies non-processing information intended to provide a comment to the calendar user.
    comment, "comment",
    /// ``: <br>	    value type: DATE-TIME
    completed, "completed",
    /// ``: 
    component, "component",
    /// ``: The property is used to represent contact information or alternately a reference to contact information associated with the calendar component.
    contact, "contact",
    /// `COUNT`: 
    count, "count",
    /// ``: <br>	    value type: DATE-TIME
    created, "created",
    /// ``: To specify the type of calendar user specified by the property.
    cutype, "cutype",
    /// ``: 
    dateTime, "dateTime",
    /// `DAYLIGHT`: 
    daylight, "daylight",
    /// ``: To specify the calendar users that have delegated their participation to the calendar user specified by the property.
    delegatedFrom, "delegatedFrom",
    /// ``: To specify the calendar users to whom the calendar user specified by the property has delegated participation.
    delegatedTo, "delegatedTo",
    /// ``: This property provides a more complete description of the calendar component, than that provided by the "SUMMARY" property.
    description, "description",
    /// ``: To specify reference to a directory entry associated with the calendar user specified by the property.
    dir, "dir",
    /// `end`: This property specifies the date and time that a calendar component ends.
    dtend, "dtend",
    /// ``: The property indicates the date/time that the instance of the iCalendar object was created.
    dtstamp, "dtstamp",
    /// `start`: This property specifies when the calendar component begins.
    dtstart, "dtstart",
    /// ``: <br>	    default value type: DATE-TIME
    due, "due",
    /// ``: The property specifies a positive duration of time.
    duration, "duration",
    /// ``: To specify an alternate inline encoding for the property value.
    encoding, "encoding",
    /// ``: This property defines the list of date/time exceptions for a recurring calendar component.
    exdate, "exdate",
    /// ``: <br>	    value type: RECUR
    exrule, "exrule",
    /// ``: To specify the free or busy time type.
    fbtype, "fbtype",
    /// ``: To specify the content type of a referenced object.
    fmttype, "fmttype",
    /// ``: <br>	    value type: PERIOD
    freebusy, "freebusy",
    /// `FREQ`: 
    freq, "freq",
    /// ``: This property specifies information related to the global position for the activity specified by a calendar component.
    geo, "geo",
    /// `INTERVAL`: 
    interval, "interval",
    /// ``: To specify the language for text values in a property or property parameter.
    language, "language",
    /// ``: <br>	    value type: DATE-TIME
    lastModified, "lastModified",
    /// `location`: <br>	    value type: TEXT
    location, "location",
    /// ``: To specify the group or list membership of the calendar user specified by the property.
    member, "member",
    /// ``: This property defines the iCalendar object method associated with the calendar object.
    method, "method",
    /// ``: <br>	    value type: CAL-ADDRESS
    organizer, "organizer",
    /// ``: To specify the participation status for the calendar user specified by the property.
    partstat, "partstat",
    /// ``: <br>	    value type: INTEGER
    percentComplete, "percentComplete",
    /// ``: <br>	    value type: INTEGER
    priority, "priority",
    /// ``: <br>	    value type: TEXT
    prodid, "prodid",
    /// ``: To specify the effective range of recurrence instances from the instance specified by the recurrence identifier specified by the property.
    range, "range",
    /// ``: This property defines the list of date/times for a recurrence set.
    rdate, "rdate",
    /// ``: This property is used in conjunction with the "UID" and "SEQUENCE" property to identify a specific instance of a recurring "VEVENT", "VTODO" or "VJOURNAL" calendar component. The property value is the effective value of the "DTSTART" property of the recurrence instance.
    recurrenceId, "recurrenceId",
    /// ``: To specify the relationship of the alarm trigger with respect to the start or end of the calendar component.
    related, "related",
    /// ``: <br>	    value type: TEXT
    relatedTo, "relatedTo",
    /// ``: To specify the type of hierarchical relationship associated with the calendar component specified by the property.
    reltype, "reltype",
    /// ``: <br>	    value type: INTEGER
    repeat, "repeat",
    /// ``: This property defines the status code returned for a scheduling request.
    requestStatus, "requestStatus",
    /// ``: This property defines the equipment or resources anticipated for an activity specified by a calendar entity..
    resources, "resources",
    /// ``: To specify the participation role for the calendar user specified by the property.
    role, "role",
    /// ``: This property defines a rule or repeating pattern for recurring events, to-dos, or time zone definitions.
    rrule, "rrule",
    /// ``: To specify whether there is an expectation of a favor of a reply from the calendar user specified by the property value.
    rsvp, "rsvp",
    /// ``: To specify the calendar user that is acting on behalf of the calendar user specified by the property.
    sentBy, "sentBy",
    /// ``: This property defines the revision sequence number of the calendar component within a sequence of revisions.
    sequence, "sequence",
    /// `STANDARD`: 
    standard, "standard",
    /// ``: This property defines the overall status or confirmation for the calendar component.
    status, "status",
    /// `summary`: <br>	    value type: TEXT
    summary, "summary",
    /// ``: This property defines whether an event is transparent or not to busy time searches.
    transp, "transp",
    /// ``: This property specifies when an alarm will trigger.
    trigger, "trigger",
    /// ``: This property specifies the text value that uniquely identifies the "VTIMEZONE" calendar component.
    tzid, "tzid",
    /// ``: <br>	    value type: TEXT
    tzname, "tzname",
    /// ``: This property specifies the offset which is in use prior to this time zone observance.
    tzoffsetfrom, "tzoffsetfrom",
    /// ``: <br>	    value type: UTC-OFFSET
    tzoffsetto, "tzoffsetto",
    /// ``: <br>	    value type: URI
    tzurl, "tzurl",
    /// ``: This property defines the persistent, globally unique identifier for the calendar component.
    uid, "uid",
    /// `UNTIL`: 
    until, "until",
    /// `see also`: This property defines a Uniform Resource Locator (URL) associated with the iCalendar object.
    url, "url",
    /// ``: This property specifies the identifier corresponding to the highest version number or the minimum and maximum range of the iCalendar specification that is required in order to interpret the iCalendar object.
    version, "version",
    /// `WKST`: 
    wkst, "wkst"
);
