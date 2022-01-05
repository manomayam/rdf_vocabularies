// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `CSVW Namespace Vocabulary Terms` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|CSVW Namespace Vocabulary Terms|
//! |**Prefix**|csvw|
//! |**Namespace base IRI**|[http://www.w3.org/ns/csvw#](http://www.w3.org/ns/csvw#)|
//! |**Description**|This document describes the RDFS vocabulary description used in the Metadata Vocabulary for Tabular Data [[tabular-metadata]] along with the default JSON-LD Context.|
//! |**Is defined by**|[http://www.w3.org/ns/csvw#](http://www.w3.org/ns/csvw#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/csvw#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Cell`: A Cell represents a cell at the intersection of a Row and a Column within a Table.
    Cell, "Cell",
    /// `Column Description`: A Column represents a vertical arrangement of Cells within a Table.
    Column, "Column",
    /// `Datatype`: Describes facets of a datatype.
    Datatype, "Datatype",
    /// `Dialect Description`: A Dialect Description provides hints to parsers about how to parse a linked file.
    Dialect, "Dialect",
    /// `Direction`: The class of table/text directions.
    Direction, "Direction",
    /// `Foreign Key Definition`: Describes relationships between Columns in one or more Tables.
    ForeignKey, "ForeignKey",
    /// `JSON`: A literal containing JSON.
    JSON, "JSON",
    /// `Numeric Format`: If the datatype is a numeric type, the format property indicates the expected format for that number. Its value must be either a single string or an object with one or more properties.
    NumericFormat, "NumericFormat",
    /// `Row`: A Row represents a horizontal arrangement of cells within a Table.
    Row, "Row",
    /// `Schema`: A Schema is a definition of a tabular format that may be common to multiple tables.
    Schema, "Schema",
    /// `Annotated Table`: An annotated table is a table that is annotated with additional metadata.
    Table, "Table",
    /// `Group of Tables`: A Group of Tables comprises a set of Annotated Tables and a set of annotations that relate to those Tables.
    TableGroup, "TableGroup",
    /// `Table Reference`: An object property that identifies a referenced table and a set of referenced columns within that table.
    TableReference, "TableReference",
    /// `Transformation Definition`: A Transformation Definition is a definition of how tabular data can be transformed into another format.
    Transformation, "Transformation",
    /// `about URL`: A URI template property that MAY be used to indicate what a cell contains information about.
    aboutUrl, "aboutUrl",
    /// `auto`: Indicates whether the tables in the group should be displayed based on the first character in the table that has a specific direction.
    auto, "auto",
    /// `base`: An atomic property that contains a single string: a term defined in the default context representing a built-in datatype URL, as listed above.
    base, "base",
    /// `column`: An array property of column descriptions as described in section 5.6 Columns.
    column, "column",
    /// `column reference`: A column reference property that holds either a single reference to a column description object within this schema, or an array of references. These form the referencing columns for the foreign key definition.
    columnReference, "columnReference",
    /// `comment prefix`: An atomic property that sets the comment prefix flag to the single provided value, which MUST be a string.
    commentPrefix, "commentPrefix",
    /// `CSV Encoded Tabular Data`: Describes the role of a CSV file in the tabular data mapping.
    csvEncodedTabularData, "csvEncodedTabularData",
    /// `datatype`: An object property that contains either a single string that is the main datatype of the values of the cell or a datatype description object. If the value of this property is a string, it MUST be one of the built-in datatypes defined in section 5.11.1 Built-in Datatypes or an absolute URL; if it is an object then it describes a more specialised datatype.
    datatype, "datatype",
    /// `decimal character`: A string whose value is used to represent a decimal point within the number.
    decimalChar, "decimalChar",
    /// `default`: An atomic property holding a single string that is used to create a default value for the cell in cases where the original string value is an empty string.
    default, "default",
    /// `delimiter`: An atomic property that sets the delimiter flag to the single provided value, which MUST be a string.
    delimiter, "delimiter",
    /// `describes`: From IANA describes: The relationship A 'describes' B asserts that resource A provides a description of resource B. There are no constraints on the format or representation of either A or B, neither are there any further constraints on either resource.
    describes, "describes",
    /// `dialect`: An object property that provides a single dialect description. If provided, dialect provides hints to processors about how to parse the referenced files to create tabular data models for the tables in the group.
    dialect, "dialect",
    /// `double quote`: A boolean atomic property that, if `true`, sets the escape character flag to `"`.
    doubleQuote, "doubleQuote",
    /// `encoding`: An atomic property that sets the encoding flag to the single provided string value, which MUST be a defined in [[encoding]]. The default is "utf-8".
    encoding, "encoding",
    /// `foreign key`: For a Table: a list of foreign keys on the table.  For a Schema: an array property of foreign key definitions that define how the values from specified columns within this table link to rows within this table or other tables.
    foreignKey, "foreignKey",
    /// `format`: An atomic property that contains either a single string or an object that defines the format of a value of this type, used when parsing a string value as described in Parsing Cells in [[tabular-data-model]].
    format, "format",
    /// `group character`: A string whose value is used to group digits within the number.
    groupChar, "groupChar",
    /// `header`: A boolean atomic property that, if `true`, sets the header row count flag to `1`, and if `false` to `0`, unless headerRowCount is provided, in which case the value provided for the header property is ignored.
    header, "header",
    /// `header row count`: An numeric atomic property that sets the header row count flag to the single provided value, which must be a non-negative integer.
    headerRowCount, "headerRowCount",
    /// `inherit`: For `textDirection`, indicates that the direction is inherited from the `tableDirection` annotation of the `table`.
    inherit, "inherit",
    /// `language`: An atomic property giving a single string language code as defined by [[BCP47]].
    lang, "lang",
    /// `length`: The exact length of the value of the cell.
    length, "length",
    /// `line terminators`: An atomic property that sets the line terminators flag to either an array containing the single provided string value, or the provided array.
    lineTerminators, "lineTerminators",
    /// `left to right`: Indicates whether the tables in the group should be displayed with the first column on the right.
    ltr, "ltr",
    /// `max exclusive`: An atomic property that contains a single number that is the maximum valid value (exclusive).
    maxExclusive, "maxExclusive",
    /// `max inclusive`: An atomic property that contains a single number that is the maximum valid value (inclusive).
    maxInclusive, "maxInclusive",
    /// `max length`: A numeric atomic property that contains a single integer that is the maximum length of the value.
    maxLength, "maxLength",
    /// `min exclusive`: An atomic property that contains a single number that is the minimum valid value (exclusive).
    minExclusive, "minExclusive",
    /// `min inclusive`: An atomic property that contains a single number that is the minimum valid value (inclusive).
    minInclusive, "minInclusive",
    /// `min length`: An atomic property that contains a single integer that is the minimum length of the value.
    minLength, "minLength",
    /// `name`: An atomic property that gives a single canonical name for the column. The value of this property becomes the name annotation for the described column.
    name, "name",
    /// `note`: An array property that provides an array of objects representing arbitrary annotations on the annotated tabular data model.
    note, "note",
    /// `null`: An atomic property giving the string or strings used for null values within the data. If the string value of the cell is equal to any one of these values, the cell value is `null`.
    null, "null",
    /// `ordered`: A boolean atomic property taking a single value which indicates whether a list that is the value of the cell is ordered (if `true`) or unordered (if `false`).
    ordered, "ordered",
    /// `pattern`: A regular expression string, in the syntax and interpreted as defined by [[ECMASCRIPT]].
    pattern, "pattern",
    /// `primary key`: For Schema: A column reference property that holds either a single reference to a column description object or an array of references.  For Row: a possibly empty list of cells whose values together provide a unique identifier for this row. This is similar to the name of a column.
    primaryKey, "primaryKey",
    /// `property URL`: An URI template property that MAY be used to create a URI for a property if the table is mapped to another format. 
    propertyUrl, "propertyUrl",
    /// `quote char`: An atomic property that sets the quote character flag to the single provided value, which must be a string or `null`.
    quoteChar, "quoteChar",
    /// `reference`: An object property that identifies a **referenced table** and a set of **referenced columns** within that table.
    reference, "reference",
    /// `referenced rows`: A possibly empty list of pairs of a foreign key and a row in a table within the same group of tables.
    referencedRow, "referencedRow",
    /// `required`: A boolean atomic property taking a single value which indicates whether the cell must have a non-null value. The default is `false`. 
    required, "required",
    /// `resource`: A link property holding a URL that is the identifier for a specific table that is being referenced.
    resource, "resource",
    /// `row`: Relates a Table to each Row output.
    row, "row",
    /// `row titles`: A column reference property that holds either a single reference to a column description object or an array of references.
    rowTitle, "rowTitle",
    /// `row number`: The position of the row amongst the rows of the Annotated Tabl, starting from 1
    rownum, "rownum",
    /// `right to left`: Indicates whether the tables in the group should be displayed with the first column on the left.
    rtl, "rtl",
    /// `schema reference`: A link property holding a URL that is the identifier for a schema that is being referenced.
    schemaReference, "schemaReference",
    /// `script format`: A link property giving the single URL for the format that is used by the script or template.
    scriptFormat, "scriptFormat",
    /// `separator`: An atomic property that MUST have a single string value that is the character used to separate items in the string value of the cell.
    separator, "separator",
    /// `skip blank rows`: An boolean atomic property that sets the `skip blank rows` flag to the single provided boolean value.
    skipBlankRows, "skipBlankRows",
    /// `skip columns`: An numeric atomic property that sets the `skip columns` flag to the single provided numeric value, which MUST be a non-negative integer.
    skipColumns, "skipColumns",
    /// `skip initial space`: A boolean atomic property that, if `true`, sets the trim flag to "start". If `false`, to `false`.
    skipInitialSpace, "skipInitialSpace",
    /// `skip rows`: An numeric atomic property that sets the `skip rows` flag to the single provided numeric value, which MUST be a non-negative integer.
    skipRows, "skipRows",
    /// `source`: A single string atomic property that provides, if specified, the format to which the tabular data should be transformed prior to the transformation using the script or template.
    source, "source",
    /// `suppress output`: A boolean atomic property. If `true`, suppresses any output that would be generated when converting a table or cells within a column.
    suppressOutput, "suppressOutput",
    /// `table`: Relates an Table group to annotated tables.
    table, "table",
    /// `table direction`: One of `rtl`, `ltr` or `auto`. Indicates whether the tables in the group should be displayed with the first column on the right, on the left, or based on the first character in the table that has a specific direction.
    tableDirection, "tableDirection",
    /// `table schema`: An object property that provides a single schema description as described in section 5.5 Schemas, used as the default for all the tables in the group
    tableSchema, "tableSchema",
    /// `Tabular Metadata`: Describes the role of a Metadata file in the tabular data mapping.
    tabularMetadata, "tabularMetadata",
    /// `target format`: A link property giving the single URL for the format that will be created through the transformation.
    targetFormat, "targetFormat",
    /// `text direction`: An atomic property that must have a single value that is one of `rtl` or `ltr` (the default).
    textDirection, "textDirection",
    /// `title`: For a Transformation A natural language property that describes the format that will be generated from the transformation.  For a Column: A natural language property that provides possible alternative names for the column.
    title, "title",
    /// `transformations`: An array property of transformation definitions that provide mechanisms to transform the tabular data into other formats.
    transformations, "transformations",
    /// `trim`: An atomic property that, if the boolean `true`, sets the trim flag to `true` and if the boolean `false` to `false`. If the value provided is a string, sets the trim flag to the provided value, which must be one of "true", "false", "start" or "end".
    trim, "trim",
    /// `uri template`: 
    uriTemplate, "uriTemplate",
    /// `url`: For a Table: This link property gives the single URL of the CSV file that the table is held in, relative to the location of the metadata document.  For a Transformation: A link property giving the single URL of the file that the script or template is held in, relative to the location of the metadata document.
    url, "url",
    /// `valueUrl`: An URI template property that is used to map the values of cells into URLs.
    valueUrl, "valueUrl",
    /// `virtual`: A boolean atomic property taking a single value which indicates whether the column is a virtual column not present in the original source
    virtual_, "virtual"
);
