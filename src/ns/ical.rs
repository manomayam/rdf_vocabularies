// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `ical` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|ical|
//! |**Namespace base IRI**|[<http://www.w3.org/2002/12/cal/icaltzd#>](<http://www.w3.org/2002/12/cal/icaltzd#>)|
//! |**Description**||
//! |**Is defined by**|[<http://www.w3.org/2002/12/cal/icaltzd.rdf>](<http://www.w3.org/2002/12/cal/icaltzd.rdf>)|
//!

use crate::namespace;

namespace!(
    "<http://www.w3.org/2002/12/cal/icaltzd#>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `DomainOf_rrule`: 
    DomainOf_rrule, "DomainOf_rrule",
    /// `List_of_Float`: 
    List_of_Float, "List_of_Float",
    /// `Valarm`: Provide a grouping of component properties that define an alarm.
    Valarm, "Valarm",
    /// `Value_CAL-ADDRESS`: 
    Value_CAL_ADDRESS, "Value_CAL-ADDRESS",
    /// `Value_DATE`: 
    Value_DATE, "Value_DATE",
    /// `Value_DATE-TIME`: 
    Value_DATE_TIME, "Value_DATE-TIME",
    /// `Value_DURATION`: 
    Value_DURATION, "Value_DURATION",
    /// `Value_PERIOD`: 
    Value_PERIOD, "Value_PERIOD",
    /// `Value_RECUR`: 
    Value_RECUR, "Value_RECUR",
    /// `VCALENDAR`: 
    Vcalendar, "Vcalendar",
    /// `Event`: Provide a grouping of component properties that describe an event.
    Vevent, "Vevent",
    /// `Vfreebusy`: Provide a grouping of component properties that describe either a request for free/busy time, describe a response to a request for free/busy time or describe a published set of busy time.
    Vfreebusy, "Vfreebusy",
    /// `Vjournal`: Provide a grouping of component properties that describe a journal entry.
    Vjournal, "Vjournal",
    /// `Vtimezone`: Provide a grouping of component properties that defines a time zone.
    Vtimezone, "Vtimezone",
    /// `To-do`: Provide a grouping of calendar properties that describe a to-do.
    Vtodo, "Vtodo",
    /// `X-`: This class of property provides a framework for defining non-standard properties.
    X_, "X-",
    /// `action`: This property defines the action to be invoked when an alarm is triggered.
    action, "action",
    /// `altrep`: To specify an alternate text representation for the property value.
    altrep, "altrep",
    /// `attach`: The property provides the capability to associate a document object with a calendar component.
    attach, "attach",
    /// `attendee`: The property defines an "Attendee" within a calendar component.
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
    /// `calAddress`: 
    calAddress, "calAddress",
    /// `calscale`: This property defines the calendar scale used for the calendar information specified in the iCalendar object.
    calscale, "calscale",
    /// `categories`: This property defines the categories for a calendar component.
    categories, "categories",
    /// `class`: This property defines the access classification for a calendar component.
    class, "class",
    /// `cn`: To specify the common name to be associated with the calendar user specified by the property.
    cn, "cn",
    /// `comment`: This property specifies non-processing information intended to provide a comment to the calendar user.
    comment, "comment",
    /// `completed`: This property defines the date and time that a to-do was actually completed.
    completed, "completed",
    /// `component`: 
    component, "component",
    /// `contact`: The property is used to represent contact information or alternately a reference to contact information associated with the calendar component.
    contact, "contact",
    /// `COUNT`: 
    count, "count",
    /// `created`: This property specifies the date and time that the calendar information was created by the calendar user agent in the calendar store. Note: This is analogous to the creation date and time for a file in the file system.
    created, "created",
    /// `cutype`: To specify the type of calendar user specified by the property.
    cutype, "cutype",
    /// `dateTime`: 
    dateTime, "dateTime",
    /// `DAYLIGHT`: 
    daylight, "daylight",
    /// `delegatedFrom`: To specify the calendar users that have delegated their participation to the calendar user specified by the property.
    delegatedFrom, "delegatedFrom",
    /// `delegatedTo`: To specify the calendar users to whom the calendar user specified by the property has delegated participation.
    delegatedTo, "delegatedTo",
    /// `description`: This property provides a more complete description of the calendar component, than that provided by the "SUMMARY" property.
    description, "description",
    /// `dir`: To specify reference to a directory entry associated with the calendar user specified by the property.
    dir, "dir",
    /// `end`: This property specifies the date and time that a calendar component ends.
    dtend, "dtend",
    /// `dtstamp`: The property indicates the date/time that the instance of the iCalendar object was created.
    dtstamp, "dtstamp",
    /// `start`: This property specifies when the calendar component begins.
    dtstart, "dtstart",
    /// `due`: This property defines the date and time that a to-do is expected to be completed.
    due, "due",
    /// `duration`: The property specifies a positive duration of time.
    duration, "duration",
    /// `encoding`: To specify an alternate inline encoding for the property value.
    encoding, "encoding",
    /// `exdate`: This property defines the list of date/time exceptions for a recurring calendar component.
    exdate, "exdate",
    /// `exrule`: This property defines a rule or repeating pattern for an exception to a recurrence set.
    exrule, "exrule",
    /// `fbtype`: To specify the free or busy time type.
    fbtype, "fbtype",
    /// `fmttype`: To specify the content type of a referenced object.
    fmttype, "fmttype",
    /// `freebusy`: The property defines one or more free or busy time intervals.
    freebusy, "freebusy",
    /// `FREQ`: 
    freq, "freq",
    /// `geo`: This property specifies information related to the global position for the activity specified by a calendar component.
    geo, "geo",
    /// `INTERVAL`: 
    interval, "interval",
    /// `language`: To specify the language for text values in a property or property parameter.
    language, "language",
    /// `lastModified`: The property specifies the date and time that the information associated with the calendar component was last revised in the calendar store. Note: This is analogous to the modification date and time for a file in the file system.
    lastModified, "lastModified",
    /// `location`: The property defines the intended venue for the activity defined by a calendar component.
    location, "location",
    /// `member`: To specify the group or list membership of the calendar user specified by the property.
    member, "member",
    /// `method`: This property defines the iCalendar object method associated with the calendar object.
    method, "method",
    /// `organizer`: The property defines the organizer for a calendar component.
    organizer, "organizer",
    /// `partstat`: To specify the participation status for the calendar user specified by the property.
    partstat, "partstat",
    /// `percentComplete`: This property is used by an assignee or delegatee of a to-do to convey the percent completion of a to-do to the Organizer.
    percentComplete, "percentComplete",
    /// `priority`: The property defines the relative priority for a calendar component.
    priority, "priority",
    /// `prodid`: This property specifies the identifier for the product that created the iCalendar object.
    prodid, "prodid",
    /// `range`: To specify the effective range of recurrence instances from the instance specified by the recurrence identifier specified by the property.
    range, "range",
    /// `rdate`: This property defines the list of date/times for a recurrence set.
    rdate, "rdate",
    /// `recurrenceId`: This property is used in conjunction with the "UID" and "SEQUENCE" property to identify a specific instance of a recurring "VEVENT", "VTODO" or "VJOURNAL" calendar component. The property value is the effective value of the "DTSTART" property of the recurrence instance.
    recurrenceId, "recurrenceId",
    /// `related`: To specify the relationship of the alarm trigger with respect to the start or end of the calendar component.
    related, "related",
    /// `relatedTo`: The property is used to represent a relationship or reference between one calendar component and another.
    relatedTo, "relatedTo",
    /// `reltype`: To specify the type of hierarchical relationship associated with the calendar component specified by the property.
    reltype, "reltype",
    /// `repeat`: This property defines the number of time the alarm should be repeated, after the initial trigger.
    repeat, "repeat",
    /// `requestStatus`: This property defines the status code returned for a scheduling request.
    requestStatus, "requestStatus",
    /// `resources`: This property defines the equipment or resources anticipated for an activity specified by a calendar entity..
    resources, "resources",
    /// `role`: To specify the participation role for the calendar user specified by the property.
    role, "role",
    /// `rrule`: This property defines a rule or repeating pattern for recurring events, to-dos, or time zone definitions.
    rrule, "rrule",
    /// `rsvp`: To specify whether there is an expectation of a favor of a reply from the calendar user specified by the property value.
    rsvp, "rsvp",
    /// `sentBy`: To specify the calendar user that is acting on behalf of the calendar user specified by the property.
    sentBy, "sentBy",
    /// `sequence`: This property defines the revision sequence number of the calendar component within a sequence of revisions.
    sequence, "sequence",
    /// `STANDARD`: 
    standard, "standard",
    /// `status`: This property defines the overall status or confirmation for the calendar component.
    status, "status",
    /// `summary`: This property defines a short summary or subject for the calendar component.
    summary, "summary",
    /// `transp`: This property defines whether an event is transparent or not to busy time searches.
    transp, "transp",
    /// `trigger`: This property specifies when an alarm will trigger.
    trigger, "trigger",
    /// `tzid`: This property specifies the text value that uniquely identifies the "VTIMEZONE" calendar component.
    tzid, "tzid",
    /// `tzname`: This property specifies the customary designation for a time zone description.
    tzname, "tzname",
    /// `tzoffsetfrom`: This property specifies the offset which is in use prior to this time zone observance.
    tzoffsetfrom, "tzoffsetfrom",
    /// `tzoffsetto`: This property specifies the offset which is in use in this time zone observance.
    tzoffsetto, "tzoffsetto",
    /// `tzurl`: The TZURL provides a means for a VTIMEZONE component to point to a network location that can be used to retrieve an up-to- date version of itself.
    tzurl, "tzurl",
    /// `uid`: This property defines the persistent, globally unique identifier for the calendar component.
    uid, "uid",
    /// `UNTIL`: 
    until, "until",
    /// `see also`: This property defines a Uniform Resource Locator (URL) associated with the iCalendar object.
    url, "url",
    /// `version`: This property specifies the identifier corresponding to the highest version number or the minimum and maximum range of the iCalendar specification that is required in order to interpret the iCalendar object.
    version, "version",
    /// `WKST`: 
    wkst, "wkst"
);
