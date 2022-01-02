// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `xsd` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|xsd|
//! |**Namespace base IRI**|[http://www.w3.org/2001/XMLSchema#](http://www.w3.org/2001/XMLSchema#)|
//! |**Description**||
//! |**Is defined by**|[https://raw.githubusercontent.com/ruby-rdf/rdf/master/etc/xsd.ttl](https://raw.githubusercontent.com/ruby-rdf/rdf/master/etc/xsd.ttl)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2001/XMLSchema#",;
    /// `ENTITIES`: <br>    ENTITIES represents the ENTITIES attribute type from [XML]. The ·value<br>    space· of ENTITIES is the set of finite, non-zero-length sequences of<br>    ·ENTITY· values that have been declared as unparsed entities in a document<br>    type definition. The ·lexical space· of ENTITIES is the set of<br>    space-separated lists of tokens, of which each token is in the ·lexical<br>    space· of ENTITY. The ·item type· of ENTITIES is ENTITY. ENTITIES is<br>    derived from ·anySimpleType· in two steps: an anonymous list type is<br>    defined, whose ·item type· is ENTITY; this is the ·base type· of ENTITIES,<br>    which restricts its value space to lists with at least one item.<br>  
    ENTITIES, "ENTITIES",
    /// `ENTITY`: <br>     ENTITY represents the ENTITY attribute type from [XML]. The ·value space·<br>     of ENTITY is the set of all strings that ·match· the NCName production in<br>     [Namespaces in XML] and have been declared as an unparsed entity in a<br>     document type definition. The ·lexical space· of ENTITY is the set of all<br>     strings that ·match· the NCName production in [Namespaces in XML]. The<br>     ·base type· of ENTITY is NCName.<br>  
    ENTITY, "ENTITY",
    /// `ID`: <br>     ID represents the ID attribute type from [XML]. The ·value space· of ID is<br>     the set of all strings that ·match· the NCName production in [Namespaces<br>     in XML]. The ·lexical space· of ID is the set of all strings that ·match·<br>     the NCName production in [Namespaces in XML]. The ·base type· of ID is<br>     NCName.<br>  
    ID, "ID",
    /// `IDREF`: <br>    IDREF represents the IDREF attribute type from [XML]. The ·value space· of<br>    IDREF is the set of all strings that ·match· the NCName production in<br>    [Namespaces in XML]. The ·lexical space· of IDREF is the set of strings<br>    that ·match· the NCName production in [Namespaces in XML]. The ·base type·<br>    of IDREF is NCName.<br>  
    IDREF, "IDREF",
    /// `IDREFS`: <br>    IDREFS represents the IDREFS attribute type from [XML]. The ·value space·<br>    of IDREFS is the set of finite, non-zero-length sequences of IDREFs. The<br>    ·lexical space· of IDREFS is the set of space-separated lists of tokens, of<br>    which each token is in the ·lexical space· of IDREF. The ·item type· of<br>    IDREFS is IDREF. IDREFS is derived from ·anySimpleType· in two steps: an<br>    anonymous list type is defined, whose ·item type· is IDREF; this is the<br>    ·base type· of IDREFS, which restricts its value space to lists with at<br>    least one item.<br>  
    IDREFS, "IDREFS",
    /// `NCName`: <br>     NCName represents XML "non-colonized" Names. The ·value space· of NCName<br>     is the set of all strings which ·match· the NCName production of<br>     [Namespaces in XML]. The ·lexical space· of NCName is the set of all<br>     strings which ·match· the NCName production of [Namespaces in XML]. The<br>     ·base type· of NCName is Name.<br>  
    NCName, "NCName",
    /// `NMTOKEN`: <br>     NMTOKEN represents the NMTOKEN attribute type from [XML]. The ·value<br>     space· of NMTOKEN is the set of tokens that ·match· the Nmtoken production<br>     in [XML]. The ·lexical space· of NMTOKEN is the set of strings that<br>     ·match· the Nmtoken production in [XML]. The ·base type· of NMTOKEN is<br>     token.<br>  
    NMTOKEN, "NMTOKEN",
    /// `NMTOKENS`: <br>    NMTOKENS represents the NMTOKENS attribute type from [XML]. The ·value<br>    space· of NMTOKENS is the set of finite, non-zero-length sequences of<br>    ·NMTOKEN·s. The ·lexical space· of NMTOKENS is the set of space-separated<br>    lists of tokens, of which each token is in the ·lexical space· of NMTOKEN.<br>    The ·item type· of NMTOKENS is NMTOKEN. NMTOKENS is derived from<br>    ·anySimpleType· in two steps: an anonymous list type is defined, whose<br>    ·item type· is NMTOKEN; this is the ·base type· of NMTOKENS, which<br>    restricts its value space to lists with at least one item.<br>  
    NMTOKENS, "NMTOKENS",
    /// `NOTATION`: <br>    NOTATION represents the NOTATION attribute type from [XML]. The ·value<br>    space· of NOTATION is the set of QNames of notations declared in the<br>    current schema. The ·lexical space· of NOTATION is the set of all names of<br>    notations declared in the current schema (in the form of QNames).<br>  
    NOTATION, "NOTATION",
    /// `Name`: <br>    Name represents XML Names. The ·value space· of Name is the set of all<br>    strings which ·match· the Name production of [XML]. The ·lexical space· of<br>    Name is the set of all strings which ·match· the Name production of [XML].<br>    The ·base type· of Name is token.<br>  
    Name, "Name",
    /// `QName`: <br>    QName represents XML qualified names. The ·value space· of QName is the set<br>    of tuples {namespace name, local part}, where namespace name is an anyURI<br>    and local part is an NCName. The ·lexical space· of QName is the set of<br>    strings that ·match· the QName production of [Namespaces in XML].<br>  
    QName, "QName",
    /// `anySimpleType`: <br>    anyAtomicType is a special ·restriction· of anySimpleType. The ·value· and<br>    ·lexical spaces· of anyAtomicType are the unions of the ·value· and<br>    ·lexical spaces· of all the ·primitive· datatypes, and anyAtomicType is<br>    their ·base type·.<br>  
    anyAtomicType, "anyAtomicType",
    /// `anySimpleType`: <br>    The definition of anySimpleType is a special ·restriction· of anyType. The<br>    ·lexical space· of anySimpleType is the set of all sequences of Unicode<br>    characters, and its ·value space· includes all ·atomic values· and all<br>    finite-length lists of zero or more ·atomic values·.<br>  
    anySimpleType, "anySimpleType",
    /// `anyType`: <br>    The root of the [XML Schema 1.1] datatype heirarchy.<br>  
    anyType, "anyType",
    /// `anyURI`: <br>    anyURI represents an Internationalized Resource Identifier Reference<br>    (IRI). An anyURI value can be absolute or relative, and may have an<br>    optional fragment identifier (i.e., it may be an IRI Reference). This<br>    type should be used when the value fulfills the role of an IRI, as<br>    defined in [RFC 3987] or its successor(s) in the IETF Standards Track.<br>  
    anyURI, "anyURI",
    /// `base64Binary`: <br>    base64Binary represents arbitrary Base64-encoded binary data. For<br>    base64Binary data the entire binary stream is encoded using the Base64<br>    Encoding defined in [RFC 3548], which is derived from the encoding<br>    described in [RFC 2045].<br>  
    base64Binary, "base64Binary",
    /// `boolean`: <br>    boolean represents the values of two-valued logic.<br>  
    boolean, "boolean",
    /// `byte`: <br>    byte is ·derived· from short by setting the value of ·maxInclusive· to be<br>    127 and ·minInclusive· to be -128. The ·base type· of byte is short.<br>  
    byte, "byte",
    /// `date`: <br>    date represents top-open intervals of exactly one day in length on the<br>    timelines of dateTime, beginning on the beginning moment of each day, up to<br>    but not including the beginning moment of the next day). For non-timezoned<br>    values, the top-open intervals disjointly cover the non-timezoned timeline,<br>    one per day. For timezoned values, the intervals begin at every minute and<br>    therefore overlap.<br>  
    date, "date",
    /// `dateTime`: <br>    dateTime represents instants of time, optionally marked with a particular<br>    time zone offset. Values representing the same instant but having different<br>    time zone offsets are equal but not identical.<br>  
    dateTime, "dateTime",
    /// `dateTimeStamp`: <br>    The dateTimeStamp datatype is ·derived· from dateTime by giving the value<br>    required to its explicitTimezone facet. The result is that all values of<br>    dateTimeStamp are required to have explicit time zone offsets and the<br>    datatype is totally ordered.<br>  
    dateTimeStamp, "dateTimeStamp",
    /// `dayTimeDuration`: <br>     dayTimeDuration is a datatype ·derived· from duration by restricting its<br>     ·lexical representations· to instances of dayTimeDurationLexicalRep. The<br>     ·value space· of dayTimeDuration is therefore that of duration restricted<br>     to those whose ·months· property is 0. This results in a duration datatype<br>     which is totally ordered.<br>  
    dayTimeDuration, "dayTimeDuration",
    /// `decimal`: <br>    decimal represents a subset of the real numbers, which can be represented<br>    by decimal numerals. The ·value space· of decimal is the set of numbers<br>    that can be obtained by dividing an integer by a non-negative power of ten,<br>    i.e., expressible as i / 10n where i and n are integers and n ≥ 0.<br>    Precision is not reflected in this value space; the number 2.0 is not<br>    distinct from the number 2.00. The order relation on decimal is the order<br>    relation on real numbers, restricted to this subset.<br>  
    decimal, "decimal",
    /// `double`: <br>    The double datatype is patterned after the IEEE double-precision 64-bit<br>    floating point datatype [IEEE 754-2008]. Each floating point datatype has a<br>    value space that is a subset of the rational numbers. Floating point<br>    numbers are often used to approximate arbitrary real numbers.<br>  
    double, "double",
    /// `duration`: <br>    duration is a datatype that represents durations of time. The concept of<br>    duration being captured is drawn from those of [ISO 8601], specifically<br>    durations without fixed endpoints. For example, "15 days" (whose most<br>    common lexical representation in duration is "'P15D'") is a duration value;<br>    "15 days beginning 12 July 1995" and "15 days ending 12 July 1995" are not<br>    duration values. duration can provide addition and subtraction operations<br>    between duration values and between duration/dateTime value pairs, and can<br>    be the result of subtracting dateTime values. However, only addition to<br>    dateTime is required for XML Schema processing and is defined in the<br>    function ·dateTimePlusDuration·.<br>  
    duration, "duration",
    /// `float`: <br>    The float datatype is patterned after the IEEE single-precision 32-bit<br>    floating point datatype [IEEE 754-2008]. Its value space is a subset of the<br>    rational numbers. Floating point numbers are often used to approximate<br>    arbitrary real numbers.<br>  
    float, "float",
    /// `gDay`: <br>    gDay represents whole days within an arbitrary month—days that recur at the<br>    same point in each (Gregorian) month. This datatype is used to represent a<br>    specific day of the month. To indicate, for example, that an employee gets<br>    a paycheck on the 15th of each month. (Obviously, days beyond 28 cannot<br>    occur in all months; they are nonetheless permitted, up to 31.)<br>  
    gDay, "gDay",
    /// `gMonth`: <br>    gMonth represents whole (Gregorian) months within an arbitrary year—months<br>    that recur at the same point in each year. It might be used, for example,<br>    to say what month annual Thanksgiving celebrations fall in different<br>    countries (--11 in the United States, --10 in Canada, and possibly other<br>    months in other countries).<br>  
    gMonth, "gMonth",
    /// `gMonthDay`: <br>    gMonthDay represents whole calendar days that recur at the same point in<br>    each calendar year, or that occur in some arbitrary calendar year.<br>    (Obviously, days beyond 28 cannot occur in all Februaries; 29 is<br>    nonetheless permitted.)<br>  
    gMonthDay, "gMonthDay",
    /// `gYear`: <br>    gYear represents Gregorian calendar years.<br>  
    gYear, "gYear",
    /// `gYearMonth`: <br>    gYearMonth represents specific whole Gregorian months in specific Gregorian years.<br>  
    gYearMonth, "gYearMonth",
    /// `hexBinary`: <br>    hexBinary represents arbitrary hex-encoded binary data. <br>  
    hexBinary, "hexBinary",
    /// `int`: <br>      int is ·derived· from long by setting the value of ·maxInclusive· to be<br>      2147483647 and ·minInclusive· to be -2147483648. The ·base type· of int<br>      is long.<br>  
    int, "int",
    /// `integer`: <br>     integer is ·derived· from decimal by fixing the value of ·fractionDigits·<br>     to be 0 and disallowing the trailing decimal point. This results in the<br>     standard mathematical concept of the integer numbers. The ·value space· of<br>     integer is the infinite set {...,-2,-1,0,1,2,...}. The ·base type· of<br>     integer is decimal.<br>  
    integer, "integer",
    /// `language`: <br>    language represents formal natural language identifiers, as defined by [BCP<br>    47] (currently represented by [RFC 4646] and [RFC 4647]) or its<br>    successor(s). The ·value space· and ·lexical space· of language are the set<br>    of all strings that conform to the pattern [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*<br>  
    language, "language",
    /// `long`: <br>     long is ·derived· from integer by setting the value of ·maxInclusive· to<br>     be 9223372036854775807 and ·minInclusive· to be -9223372036854775808. The<br>     ·base type· of long is integer.<br>  
    long, "long",
    /// `negativeInteger`: <br>     negativeInteger is ·derived· from nonPositiveInteger by setting the value<br>     of ·maxInclusive· to be -1. This results in the standard mathematical<br>     concept of the negative integers. The ·value space· of negativeInteger is<br>     the infinite set {...,-2,-1}. The ·base type· of negativeInteger is<br>     nonPositiveInteger.<br>  
    negativeInteger, "negativeInteger",
    /// `nonNegativeInteger`: <br>     nonNegativeInteger is ·derived· from integer by setting the value of<br>     ·minInclusive· to be 0. This results in the standard mathematical concept<br>     of the non-negative integers. The ·value space· of nonNegativeInteger is<br>     the infinite set {0,1,2,...}. The ·base type· of nonNegativeInteger is<br>     integer.<br>  
    nonNegativeInteger, "nonNegativeInteger",
    /// `nonPositiveInteger`: <br>    nonPositiveInteger is ·derived· from integer by setting the value of<br>    ·maxInclusive· to be 0. This results in the standard mathematical concept<br>    of the non-positive integers. The ·value space· of nonPositiveInteger is<br>    the infinite set {...,-2,-1,0}. The ·base type· of nonPositiveInteger is<br>    integer.<br>  
    nonPositiveInteger, "nonPositiveInteger",
    /// `normalizedString`: <br>    normalizedString represents white space normalized strings. The ·value<br>    space· of normalizedString is the set of strings that do not contain the<br>    carriage return (#xD), line feed (#xA) nor tab (#x9) characters. The<br>    ·lexical space· of normalizedString is the set of strings that do not<br>    contain the carriage return (#xD), line feed (#xA) nor tab (#x9)<br>    characters. The ·base type· of normalizedString is string.<br>  
    normalizedString, "normalizedString",
    /// `positiveInteger`: <br>     positiveInteger is ·derived· from nonNegativeInteger by setting the value<br>     of ·minInclusive· to be 1. This results in the standard mathematical<br>     concept of the positive integer numbers. The ·value space· of<br>     positiveInteger is the infinite set {1,2,...}. The ·base type· of<br>     positiveInteger is nonNegativeInteger.<br>  
    positiveInteger, "positiveInteger",
    /// `short`: <br>    short is ·derived· from int by setting the value of ·maxInclusive· to be<br>    32767 and ·minInclusive· to be -32768. The ·base type· of short is int.<br>  
    short, "short",
    /// `string`: <br>    The string datatype represents character strings in XML.<br>  
    string, "string",
    /// `time`: <br>    time represents instants of time that recur at the same point in each<br>    calendar day, or that occur in some arbitrary calendar day.<br>  
    time, "time",
    /// `token`: <br>     token represents tokenized strings. The ·value space· of token is the set<br>     of strings that do not contain the carriage return (#xD), line feed (#xA)<br>     nor tab (#x9) characters, that have no leading or trailing spaces (#x20)<br>     and that have no internal sequences of two or more spaces. The ·lexical<br>     space· of token is the set of strings that do not contain the carriage<br>     return (#xD), line feed (#xA) nor tab (#x9) characters, that have no<br>     leading or trailing spaces (#x20) and that have no internal sequences of<br>     two or more spaces. The ·base type· of token is normalizedString.<br>  
    token, "token",
    /// `unsignedByte`: <br>      nsignedByte is ·derived· from unsignedShort by setting the value of<br>      ·maxInclusive· to be 255. The ·base type· of unsignedByte is<br>      unsignedShort.<br>    
    unsignedByte, "unsignedByte",
    /// `unsignedInt`: <br>    unsignedInt is ·derived· from unsignedLong by setting the value of<br>    ·maxInclusive· to be 4294967295. The ·base type· of unsignedInt is<br>    unsignedLong.<br>  
    unsignedInt, "unsignedInt",
    /// `unsignedLong`: <br>     unsignedLong is ·derived· from nonNegativeInteger by setting the value of<br>     ·maxInclusive· to be 18446744073709551615. The ·base type· of unsignedLong<br>     is nonNegativeInteger.<br>  
    unsignedLong, "unsignedLong",
    /// `unsignedShort`: <br>       unsignedShort is ·derived· from unsignedInt by setting the value of<br>       ·maxInclusive· to be 65535. The ·base type· of unsignedShort is<br>       unsignedInt.<br>    
    unsignedShort, "unsignedShort",
    /// `yearMonthDuration`: <br>     yearMonthDuration is a datatype ·derived· from duration by restricting its<br>     ·lexical representations· to instances of yearMonthDurationLexicalRep. The<br>     ·value space· of yearMonthDuration is therefore that of duration<br>     restricted to those whose ·seconds· property is 0. This results in a<br>     duration datatype which is totally ordered.<br>  
    yearMonthDuration, "yearMonthDuration"
);
