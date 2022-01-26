// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `qudt` vocabulary
//!
//! This module requires `ns-qudt` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|qudt|
//! |**Namespace base IRI**|<http://qudt.org/schema/qudt/>|
//! |**Description**||
//! |**Is defined by**|<http://qudt.org/2.1/schema/datatype>|
//!

use crate::namespace;

namespace!(
    "http://qudt.org/schema/qudt/",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Abstract Datatype`: 
    AbstractDatatype, "AbstractDatatype",
    /// `Quantity Kind (abstract)`: 
    AbstractQuantityKind, "AbstractQuantityKind",
    /// `Aerospace coordinate system`: 
    AerospaceCoordinateSystem, "AerospaceCoordinateSystem",
    /// `Algebraic Datatype`: <p>An "Algebraic Datatype" is a datatype each of whose values are data from other data types wrapped in one of the constructors of the data type. Any wrapped datum is an argument to the constructor. In contrast to other data types, the constructor is not executed and the only way to operate on the data is to unwrap the constructor using pattern matching.</p>  <p>The most common algebraic data type is a list with two constructors: Nil or [] for an empty list, and Cons (an abbreviation of constructor), ::, or : for the combination of a new element with a shorter list (for example (Cons 1 '(2 3 4)) or 1:[2,3,4]).</p>  <p>Special cases of algebraic types are product types i.e. records (only one constructor) and enumerated types (many constructors with no arguments). Algebraic types are one kind of composite type (i.e. a type formed by combining other types).</p>  <p>An algebraic data type may also be an abstract data type (ADT) if it is exported from a module without its constructors. Values of such a type can only be manipulated using functions defined in the same module as the type itself.</p>
    AlgebraicDatatype, "AlgebraicDatatype",
    /// `Alignment type`: 
    AlignmentType, "AlignmentType",
    /// `Array Type`: 
    Array, "Array",
    /// `Array data order type`: 
    ArrayDataOrderType, "ArrayDataOrderType",
    /// `Array Index Type`: 
    ArrayIndex, "ArrayIndex",
    /// `ArrayIndex-elementType`: 
    ArrayIndex_elementType, "ArrayIndex-elementType",
    /// `Array Index Type`: 
    ArrayIndexType, "ArrayIndexType",
    /// `Array Type`: <p>An <em>array type</em> is a type specification for ordered entries of values arranged according to the dimensions given. The dimensions are given as a list of integers where each integer is the cardinality of each dimension. The number of dimensions is specified in the attribute 'dimensionality'.</p>  <p>Optionally a reference can be made to a variable, whose values are the array entries. The data type of the array entries is an optional field, 'elementType', which points to a data type definition. If the data type is given, then it applies to all elements.  If it is not given then the elements can be of different types for each position in the array.</p>  <p>The property <em>qudt:typeMatrix</em> must refer to a matrix of types.  If a default value is given this can be used to place the appropriate value in a cell when an entry value is not known. If no dimensionality or dimension vector is specified the array is under-specified but still legitimate as a place-holder for a data type.</p>
    ArrayType, "ArrayType",
    /// `ArrayType-byRow`: 
    ArrayType_byRow, "ArrayType-byRow",
    /// `ArrayType-dataOrder`: 
    ArrayType_dataOrder, "ArrayType-dataOrder",
    /// `ArrayType-dimensionVector`: 
    ArrayType_dimensionVector, "ArrayType-dimensionVector",
    /// `ArrayType-dimensionality`: 
    ArrayType_dimensionality, "ArrayType-dimensionality",
    /// `ArrayType-typeMatrix`: 
    ArrayType_typeMatrix, "ArrayType-typeMatrix",
    /// `Pub article type`: 
    Article, "Article",
    /// `QUDT Aspect`: An aspect is an abstract type class that defines properties that can be reused.
    Aspect, "Aspect",
    /// `Aspect Class`: 
    AspectClass, "AspectClass",
    /// `Associative Array`: 
    AssociativeArray, "AssociativeArray",
    /// `Associative Array Type`: 
    AssociativeArrayType, "AssociativeArrayType",
    /// `Aural Cue`: An aural cue is a sound produced by a device or a system that is used to alert personnel of of an advisory, cautionary, warning, or emergency state.
    AuralCue, "AuralCue",
    /// `AuralCue-sound`: 
    AuralCue_sound, "AuralCue-sound",
    /// `AuralCueEnumeration-defaultValue`: 
    AuralCueEnumeration_defaultValue, "AuralCueEnumeration-defaultValue",
    /// `Axial Orientation Type`: 
    AxialOrientationType, "AxialOrientationType",
    /// `BOOLEAN`: 
    BOOLEAN, "BOOLEAN",
    /// `Byte`: 
    BYTE, "BYTE",
    /// `Balanced Tree Type`: 
    BalancedTree, "BalancedTree",
    /// `BalancedTree-maxDepth`: 
    BalancedTree_maxDepth, "BalancedTree-maxDepth",
    /// `Balanced Tree Type`: 
    BalancedTreeType, "BalancedTreeType",
    /// `Base Dimension Magnitude`: <p class="lm-para">A <em>Dimension</em> expresses a magnitude for a base quantiy kind such as mass, length and time.</p> <p class="lm-para">DEPRECATED - each exponent is expressed as a property. Keep until a validaiton of this has been done.</p>
    BaseDimensionMagnitude, "BaseDimensionMagnitude",
    /// `Big Endian`: 
    BigEndian, "BigEndian",
    /// `Big Integer Type`: 
    BigIntegerType, "BigIntegerType",
    /// `BigIntegerType-octets`: 
    BigIntegerType_octets, "BigIntegerType-octets",
    /// `Binary Prefix`: A <em>Binary Prefix</em> is a prefix for multiples of units in data processing, data transmission, and digital information, notably the bit and the byte, to indicate multiplication by a power of 2.
    BinaryPrefix, "BinaryPrefix",
    /// `Binary Tree Type`: 
    BinaryTree, "BinaryTree",
    /// `Binary Tree Type`: 
    BinaryTreeType, "BinaryTreeType",
    /// `Bit Aligned`: 
    BitAligned, "BitAligned",
    /// `Bit Encoding`: 
    BitEncodingType, "BitEncodingType",
    /// `bitfield`: 
    BitField, "BitField",
    /// `Bit Field of 1 bit`: 
    BitField01, "BitField01",
    /// `Bit Field of 2 bits`: 
    BitField02, "BitField02",
    /// `Bit Field of 3 bits`: 
    BitField03, "BitField03",
    /// `Bit Field of 4 bits`: 
    BitField04, "BitField04",
    /// `Bit Field of 5 bits`: 
    BitField05, "BitField05",
    /// `Bit Field of 6 bits`: 
    BitField06, "BitField06",
    /// `Bit Field of 7 bits`: 
    BitField07, "BitField07",
    /// `Bit Field of 8 bits`: 
    BitField08, "BitField08",
    /// `Bit Field of 9 bits`: 
    BitField09, "BitField09",
    /// `Bit Field Of 10 Bits`: 
    BitField10, "BitField10",
    /// `Bit Field Of 11 Bits`: 
    BitField11, "BitField11",
    /// `Bit Field of 12 bits`: 
    BitField12, "BitField12",
    /// `Bit Field Type`: 
    BitFieldType, "BitFieldType",
    /// `BitFieldType-encodedValue`: 
    BitFieldType_encodedValue, "BitFieldType-encodedValue",
    /// `Pub book type`: 
    Book, "Book",
    /// `Pub booklet type`: 
    Booklet, "Booklet",
    /// `Boolean Encoding`: 
    BooleanEncoding, "BooleanEncoding",
    /// `Boolean encoding type`: 
    BooleanEncodingType, "BooleanEncodingType",
    /// `BooleanStateType`: 
    BooleanStateType, "BooleanStateType",
    /// `Boolean Type`: 
    BooleanType, "BooleanType",
    /// `BooleanType-encoding`: 
    BooleanType_encoding, "BooleanType-encoding",
    /// `boolean value`: 
    BooleanTypeEnumeratedValue, "BooleanTypeEnumeratedValue",
    /// `By Column`: 
    ByColumn, "ByColumn",
    /// `By Left Most Index`: 
    ByLeftMostIndex, "ByLeftMostIndex",
    /// `By Row`: 
    ByRow, "ByRow",
    /// `Byte Aligned`: 
    ByteAligned, "ByteAligned",
    /// `Byte Encoding`: 
    ByteEncodingType, "ByteEncodingType",
    /// `Earth-centered Coordinate System Type`: 
    CCT_EarthCentered, "CCT_EarthCentered",
    /// `Mars-centered Coordinate System Type`: 
    CCT_MarsCentered, "CCT_MarsCentered",
    /// `Moon-centered Coordinate System Type`: 
    CCT_MoonCentered, "CCT_MoonCentered",
    /// `Sun-centered Coordinate System Type`: 
    CCT_SunCentered, "CCT_SunCentered",
    /// `Vehicle-centered Coordinate System Type`: 
    CCT_VehicleCentered, "CCT_VehicleCentered",
    /// `Concatenate matrix rows`: 
    CFN_ConcatenateMatrixRows, "CFN_ConcatenateMatrixRows",
    /// `Amber Color`: 
    COLOR_AMBER, "COLOR_AMBER",
    /// `Green color`: 
    COLOR_GREEN, "COLOR_GREEN",
    /// `Orange color`: 
    COLOR_ORANGE, "COLOR_ORANGE",
    /// `Red color`: 
    COLOR_RED, "COLOR_RED",
    /// `Yellow color`: 
    COLOR_YELLOW, "COLOR_YELLOW",
    /// `CRC-32`: 
    CRC32, "CRC32",
    /// `Countably Infinite Cardinality Type`: 
    CT_COUNTABLY_INFINITE, "CT_COUNTABLY-INFINITE",
    /// `Finite Cardinality Type`: 
    CT_FINITE, "CT_FINITE",
    /// `Uncountable Cardinality Type`: 
    CT_UNCOUNTABLE, "CT_UNCOUNTABLE",
    /// `Cardinality Type`: 
    CardinalityType, "CardinalityType",
    /// `Cartesian Coordinate Type`: 
    CartesianCoordinates, "CartesianCoordinates",
    /// `Cartesian Coordinate Type`: 
    CartesianCoordinatesType, "CartesianCoordinatesType",
    /// `Char Encoding`: 
    CharEncoding, "CharEncoding",
    /// `Char Encoding Type`: 
    CharEncodingType, "CharEncodingType",
    /// `Character Type`: 
    CharacterType, "CharacterType",
    /// `Citation`: Provides a simple way of making citations.
    Citation, "Citation",
    /// `Collection Type`: 
    Collection, "Collection",
    /// `Collection Type`: 
    CollectionType, "CollectionType",
    /// `Color Cue`: A visual cue that uses color to distinguish it from other cues. Each color cue has exactly one corresponding coordinate point in the RGB space.
    ColorCue, "ColorCue",
    /// `ColorCue-rgbCode`: 
    ColorCue_rgbCode, "ColorCue-rgbCode",
    /// `Comment`: 
    Comment, "Comment",
    /// `Composite Data Structure`: 'Composite Data Structure', also referred to as 'Data Record' is a datatype that aggregates element of possibly different types. The aggregated items are called fields or members and are usually identified or indexed by field labels.
    CompositeDataStructure, "CompositeDataStructure",
    /// `CompositeDataStructure-dataElement`: 
    CompositeDataStructure_dataElement, "CompositeDataStructure-dataElement",
    /// `Composite Data Type`: 
    CompositeDatatype, "CompositeDatatype",
    /// `CompositeDatatype-alignment`: 
    CompositeDatatype_alignment, "CompositeDatatype-alignment",
    /// `CompositeDatatype-elementType`: 
    CompositeDatatype_elementType, "CompositeDatatype-elementType",
    /// `CompositeDatatype-padding`: 
    CompositeDatatype_padding, "CompositeDatatype-padding",
    /// `Composite Table Type`: 
    CompositeTable, "CompositeTable",
    /// `Composition function`: 
    CompositionFunction, "CompositionFunction",
    /// `CompositionTreeType`: 
    CompositionTreeType, "CompositionTreeType",
    /// `CompositionTreeType-compositionFunction`: 
    CompositionTreeType_compositionFunction, "CompositionTreeType-compositionFunction",
    /// `QUDT Concept`: The root class for all QUDT concepts.
    Concept, "Concept",
    /// `Pub techreport type`: 
    Conference, "Conference",
    /// `Constant value`: Used to specify the values of a constant.
    ConstantValue, "ConstantValue",
    /// `Container`: 
    Container, "Container",
    /// `Container-elementTypeCount`: 
    Container_elementTypeCount, "Container-elementTypeCount",
    /// `Coordinate Center Type`: 
    CoordinateCenterType, "CoordinateCenterType",
    /// `Coordinate Member Type`: 
    CoordinateMember, "CoordinateMember",
    /// `Coordinate system`: 
    CoordinateSystem, "CoordinateSystem",
    /// `CoordinateSystem-abbreviation`: 
    CoordinateSystem_abbreviation, "CoordinateSystem-abbreviation",
    /// `CoordinateSystem-acronym`: 
    CoordinateSystem_acronym, "CoordinateSystem-acronym",
    /// `CoordinateSystem-coordinateCenter`: 
    CoordinateSystem_coordinateCenter, "CoordinateSystem-coordinateCenter",
    /// `CoordinateSystem-referenceFrame`: 
    CoordinateSystem_referenceFrame, "CoordinateSystem-referenceFrame",
    /// `Coordinate system frame`: 
    CoordinateSystemFrame, "CoordinateSystemFrame",
    /// `Coordinate System Type`: <p>A coordinate system is a mathematical structure for assigning a unique n-tuple of numbers or scalars to each point in an n-dimensional space. A Coordinate System Type is a data type that defines the properties of data structures that represent coordinate systems.</p>  <p>The primary attributes of any coordinate system are the coordinate frame or orientation of the axes of the coordinate system and the coordinate center or origin of the coordinate system. The coordinate center is the easier of the two attributes to define and in trajectory-related coordinate systems is often taken to be the center of mass of natural solar system bodies such as the Earth, the Moon, or Mars. Precise definition of the coordinate frame, however, usually takes much more effort. As a result, the primary purpose of this section is to provide a detailed description of a number of different coordinate frames commonly used in lunar and Mars mission analysis. </p>
    CoordinateSystemType, "CoordinateSystemType",
    /// `CoordinateSystemType-originDefinition`: 
    CoordinateSystemType_originDefinition, "CoordinateSystemType-originDefinition",
    /// `Coordinate System Type`: 
    Coordinates, "Coordinates",
    /// `2D Coordinate Type`: 
    Coordinates_2D, "Coordinates-2D",
    /// `Coordinates-2D-Double precision`: 
    Coordinates_2D_DoublePrecision, "Coordinates-2D-DoublePrecision",
    /// `Coordinates-2D-DoublePrecision-Double_X`: 
    Coordinates_2D_DoublePrecision_Double_X, "Coordinates-2D-DoublePrecision-Double_X",
    /// `Coordinates-2D-DoublePrecision-Double_Y`: 
    Coordinates_2D_DoublePrecision_Double_Y, "Coordinates-2D-DoublePrecision-Double_Y",
    /// `Cartesian Coordinates 2D Single Precision`: 
    Coordinates_2D_SinglePrecision, "Coordinates-2D-SinglePrecision",
    /// `Coordinates-2D-SinglePrecision-float_X`: 
    Coordinates_2D_SinglePrecision_float_X, "Coordinates-2D-SinglePrecision-float_X",
    /// `Coordinates-2D-SinglePrecision-float_Y`: 
    Coordinates_2D_SinglePrecision_float_Y, "Coordinates-2D-SinglePrecision-float_Y",
    /// `2D Coordinate Type`: 
    Coordinates_2D_Type, "Coordinates-2D-Type",
    /// `3D Coordinate Type`: 
    Coordinates_3D, "Coordinates-3D",
    /// `3D Coordinates (Double Precision)`: 
    Coordinates_3D_DoublePrecision, "Coordinates-3D-DoublePrecision",
    /// `Coordinates-3D-DoublePrecision-Double_X`: 
    Coordinates_3D_DoublePrecision_Double_X, "Coordinates-3D-DoublePrecision-Double_X",
    /// `Coordinates-3D-DoublePrecision-Double_Y`: 
    Coordinates_3D_DoublePrecision_Double_Y, "Coordinates-3D-DoublePrecision-Double_Y",
    /// `Coordinates-3D-DoublePrecision-Double_Z`: 
    Coordinates_3D_DoublePrecision_Double_Z, "Coordinates-3D-DoublePrecision-Double_Z",
    /// `3D Coordinates (Double Precision) type`: 
    Coordinates_3D_DoublePrecision_Type, "Coordinates-3D-DoublePrecision-Type",
    /// `3D Coordinates (Single Precision)`: 
    Coordinates_3D_SinglePrecision, "Coordinates-3D-SinglePrecision",
    /// `3D Coordinates (Single Precision)`: 
    Coordinates_3D_SinglePrecision_Type, "Coordinates-3D-SinglePrecision-Type",
    /// `Coordinates-3D-SinglePrecision-float_X`: 
    Coordinates_3D_SinglePrecision_float_X, "Coordinates-3D-SinglePrecision-float_X",
    /// `Coordinates-3D-SinglePrecision-float_Y`: 
    Coordinates_3D_SinglePrecision_float_Y, "Coordinates-3D-SinglePrecision-float_Y",
    /// `Coordinates-3D-SinglePrecision-float_Z`: 
    Coordinates_3D_SinglePrecision_float_Z, "Coordinates-3D-SinglePrecision-float_Z",
    /// `3D Coordinate Type`: 
    Coordinates_3D_Type, "Coordinates-3D-Type",
    /// `Coordinates-elementType`: 
    Coordinates_elementType, "Coordinates-elementType",
    /// `Counting Unit`: Used for all units that express counts. Examples are Atomic Number, Number, Number per Year, Percent and Sample per Second.
    CountingUnit, "CountingUnit",
    /// `Currency Unit`: Currency Units have their own subclass of unit because: (a) they have additonal properites such as 'country' and (b) their URIs do not conform to the same rules as other units.
    CurrencyUnit, "CurrencyUnit",
    /// `Date`: 
    DATE, "DATE",
    /// `Date Time`: 
    DATETIME, "DATETIME",
    /// `Decimal`: 
    DECIMAL, "DECIMAL",
    /// `Dimension Vector 2x4`: 
    DV_2x4, "DV_2x4",
    /// `Dimension Vector 4x4`: 
    DV_4x4, "DV_4x4",
    /// `DataElement`: 
    DataElement, "DataElement",
    /// `DataElement-type`: 
    DataElement_type, "DataElement-type",
    /// `Data Encoding`: <p><em>Data Encoding</em> expresses the properties that specify how data is represented at the bit and byte level. These properties are applicable to describing raw data.</p>
    DataEncoding, "DataEncoding",
    /// `QUDT DataSet Element`: A field is a tuple that carries a name, a type and optionally other properties that characterize a member element of a composite data strucuture.
    DataSetElement, "DataSetElement",
    /// `DataSetElement-elementLabel`: 
    DataSetElement_elementLabel, "DataSetElement-elementLabel",
    /// `DataSetElement-optional`: 
    DataSetElement_optional, "DataSetElement-optional",
    /// `DataSetElement-quantityKind`: 
    DataSetElement_quantityKind, "DataSetElement-quantityKind",
    /// `QUDT Datatype`: 
    Datatype, "Datatype",
    /// `Datatype-ansiSQLName`: 
    Datatype_ansiSQLName, "Datatype-ansiSQLName",
    /// `Datatype-basis`: 
    Datatype_basis, "Datatype-basis",
    /// `Datatype-bounded`: 
    Datatype_bounded, "Datatype-bounded",
    /// `Datatype-cName`: 
    Datatype_cName, "Datatype-cName",
    /// `Datatype-cardinality`: 
    Datatype_cardinality, "Datatype-cardinality",
    /// `Datatype-description`: 
    Datatype_description, "Datatype-description",
    /// `Datatype-id`: 
    Datatype_id, "Datatype-id",
    /// `Datatype-javaName`: 
    Datatype_javaName, "Datatype-javaName",
    /// `Datatype-jsName`: 
    Datatype_jsName, "Datatype-jsName",
    /// `Datatype-matlabName`: 
    Datatype_matlabName, "Datatype-matlabName",
    /// `Datatype-microsoftSQLServerName`: 
    Datatype_microsoftSQLServerName, "Datatype-microsoftSQLServerName",
    /// `Datatype-mySQLName`: 
    Datatype_mySQLName, "Datatype-mySQLName",
    /// `Datatype-odbcName`: 
    Datatype_odbcName, "Datatype-odbcName",
    /// `Datatype-oleDBName`: 
    Datatype_oleDBName, "Datatype-oleDBName",
    /// `Datatype-oracleSQLName`: 
    Datatype_oracleSQLName, "Datatype-oracleSQLName",
    /// `Datatype-protocolBuffersName`: 
    Datatype_protocolBuffersName, "Datatype-protocolBuffersName",
    /// `Datatype-pythonName`: 
    Datatype_pythonName, "Datatype-pythonName",
    /// `Datatype-vbName`: 
    Datatype_vbName, "Datatype-vbName",
    /// `DatatypeStructuredData`: 
    DatatypeStructuredData, "DatatypeStructuredData",
    /// `Date String Type`: 
    DateStringType, "DateStringType",
    /// `Date Time String Encoding Type`: 
    DateTimeStringEncodingType, "DateTimeStringEncodingType",
    /// `DateTimeStringEncodingType-allowedPattern`: 
    DateTimeStringEncodingType_allowedPattern, "DateTimeStringEncodingType-allowedPattern",
    /// `Temporal Type`: 
    DateTimeStringType, "DateTimeStringType",
    /// `DateTimeStringType-encoding`: 
    DateTimeStringType_encoding, "DateTimeStringType-encoding",
    /// `Decimal Prefix`: A <em>Decimal Prefix</em> is a prefix for multiples of units that are powers of 10.
    DecimalPrefix, "DecimalPrefix",
    /// `DecimalScaledUnit`: 
    DecimalScaledUnit, "DecimalScaledUnit",
    /// `DerivedCoherentUnit`: 
    DerivedCoherentUnit, "DerivedCoherentUnit",
    /// `DerivedNonCoherentUnit`: 
    DerivedNonCoherentUnit, "DerivedNonCoherentUnit",
    /// `Derived Unit`: A DerivedUnit is a type specification for units that are derived from other units.
    DerivedUnit, "DerivedUnit",
    /// `Dictionary Type`: 
    Dictionary, "Dictionary",
    /// `Dictionary Type`: 
    DictionaryType, "DictionaryType",
    /// `Dimension Vector`: 
    DimensionVector, "DimensionVector",
    /// `Dimensional Data Type`: 
    DimensionalDatatype, "DimensionalDatatype",
    /// `Dimensionless Unit`: A Dimensionless Unit is a quantity for which all the exponents of the factors corresponding to the base quantities in its quantity dimension are zero.
    DimensionlessUnit, "DimensionlessUnit",
    /// `Discipline`: 
    Discipline, "Discipline",
    /// `DiscreteState`: 
    DiscreteState, "DiscreteState",
    /// `Single Precision Real Encoding`: 
    DoublePrecisionEncoding, "DoublePrecisionEncoding",
    /// `Double Precision Type`: 
    DoublePrecisionType, "DoublePrecisionType",
    /// `Double_X`: 
    Double_X, "Double_X",
    /// `Double_Y`: 
    Double_Y, "Double_Y",
    /// `Double_Z`: 
    Double_Z, "Double_Z",
    /// `Earth Coordinate System Type`: 
    EarthCoordinateSystem, "EarthCoordinateSystem",
    /// `EarthCoordinateSystem-coordinateCenter`: 
    EarthCoordinateSystem_coordinateCenter, "EarthCoordinateSystem-coordinateCenter",
    /// `Encoding`: 
    Encoding, "Encoding",
    /// `Encoding-bits`: 
    Encoding_bits, "Encoding-bits",
    /// `Encoding-bytes`: 
    Encoding_bytes, "Encoding-bytes",
    /// `Endian Type`: 
    EndianType, "EndianType",
    /// `Engineering Value Tuple Member`: 
    EngineeringValueTupleMember, "EngineeringValueTupleMember",
    /// `EngineeringValueTupleMember-elementType`: 
    EngineeringValueTupleMember_elementType, "EngineeringValueTupleMember-elementType",
    /// `Enumerated Value`: 
    EnumeratedValue, "EnumeratedValue",
    /// `Enumeration`: <p>An enumeration is a set of literals from which a single value is selected. Each literal can have a tag as an integer within a standard encoding appropriate to the range of integer values. Consistency of enumeration types will allow them, and the enumerated values, to be referred to unambiguously either through symbolic name or encoding. Enumerated values are also controlled vocabularies and as such need to be standardized. Without this consistency enumeration literals can be stated differently and result in  data conflicts and misinterpretations.</p>  <p>The tags are a set of positive whole numbers, not necessarily contiguous and having no numerical significance, each corresponding to the associated literal identifier. An order attribute can also be given on the enumeration elements. An enumeration can itself be a member of an enumeration. This allows enumerations to be enumerated in a selection. Enumerations are also subclasses of <em>Scalar Datatype</em>. This allows them to be used as the reference of a datatype specification.</p>
    Enumeration, "Enumeration",
    /// `Enumeration-bits`: 
    Enumeration_bits, "Enumeration-bits",
    /// `Enumeration-defaultValue`: 
    Enumeration_defaultValue, "Enumeration-defaultValue",
    /// `Enumeration-encoding`: 
    Enumeration_encoding, "Enumeration-encoding",
    /// `Enumeration-value`: 
    Enumeration_value, "Enumeration-value",
    /// `Enumeration scale`: 
    EnumerationScale, "EnumerationScale",
    /// `False`: 
    FALSE, "FALSE",
    /// `FLAG`: 
    FLAG, "FLAG",
    /// `FLOAT-DP`: 
    FLOAT_DP, "FLOAT-DP",
    /// `Inertial Frame Type`: 
    FT_INERTIAL, "FT_INERTIAL",
    /// `Non-rotating Frame Type`: 
    FT_NON_ROTATING, "FT_NON-ROTATING",
    /// `Rotating Frame Type`: 
    FT_ROTATING, "FT_ROTATING",
    /// `Field Type`: 
    FieldType, "FieldType",
    /// `FieldType-elementName`: 
    FieldType_elementName, "FieldType-elementName",
    /// `FieldType-elementType`: 
    FieldType_elementType, "FieldType-elementType",
    /// `FieldType-fieldLabel`: 
    FieldType_fieldLabel, "FieldType-fieldLabel",
    /// `FieldType-fieldType`: 
    FieldType_fieldType, "FieldType-fieldType",
    /// `FieldType-optional`: 
    FieldType_optional, "FieldType-optional",
    /// `Figure`: 
    Figure, "Figure",
    /// `File format`: 
    FileFormat, "FileFormat",
    /// `Fixed Interval Time Series Array Type`: 
    FixedIntervalTimeSeriesArray, "FixedIntervalTimeSeriesArray",
    /// `Fixed Interval Time Series Array Type`: 
    FixedIntervalTimeSeriesArrayType, "FixedIntervalTimeSeriesArrayType",
    /// `Floating Point Encoding`: 
    FloatingPointEncodingType, "FloatingPointEncodingType",
    /// `Frame Type`: 
    FrameType, "FrameType",
    /// `Function`: 
    Function, "Function",
    /// `Function Data Type`: 
    FunctionDatatype, "FunctionDatatype",
    /// `FunctionDatatype-argType`: 
    FunctionDatatype_argType, "FunctionDatatype-argType",
    /// `FunctionDatatype-functionArity`: 
    FunctionDatatype_functionArity, "FunctionDatatype-functionArity",
    /// `FunctionDatatype-returnType`: 
    FunctionDatatype_returnType, "FunctionDatatype-returnType",
    /// `Graph Type`: 
    Graph, "Graph",
    /// `Graph Type`: 
    GraphType, "GraphType",
    /// `GreekCharacter`: 
    GreekCharacter, "GreekCharacter",
    /// `Ground coordinate system`: 
    GroundCoordinateSystem, "GroundCoordinateSystem",
    /// `HEAP`: 
    HEAP, "HEAP",
    /// `HEXBINARY`: 
    HEXBINARY, "HEXBINARY",
    /// `Hash Table Type`: 
    HashTable, "HashTable",
    /// `Hash Table Type`: 
    HashTableType, "HashTableType",
    /// `Heap Type`: 
    Heap, "Heap",
    /// `Heap Type`: A heap is a specialized tree-based data structure that satisfies the condition: if B is a child node of A, then <em>key(A) &gt; key(B)</em>. This implies that an element with the greatest key is always in the root node, and so such a heap is sometimes called a max heap. Alternatively, if the comparison is reversed, the smallest element is always in the root node, which results in a min heap. The function or method that implements the key() operator is specified by the heap type.
    HeapType, "HeapType",
    /// `Hexidecimal Binary Type`: 
    HexBinaryType, "HexBinaryType",
    /// `HexBinaryType-length`: 
    HexBinaryType_length, "HexBinaryType-length",
    /// `HexBinaryType-maxLength`: 
    HexBinaryType_maxLength, "HexBinaryType-maxLength",
    /// `HexBinaryType-minLength`: 
    HexBinaryType_minLength, "HexBinaryType-minLength",
    /// `HexBinaryType-pattern`: 
    HexBinaryType_pattern, "HexBinaryType-pattern",
    /// `High To Low`: 
    HighToLow, "HighToLow",
    /// `IEEE 754 1985 Real Encoding`: 
    IEEE754_1985RealEncoding, "IEEE754_1985RealEncoding",
    /// `IERS-TN-32-2004`: 
    IERS_TN_32_2004, "IERS-TN-32-2004",
    /// `IMPERIAL-DimensionVector`: 
    IMPERIAL_DimensionVector, "IMPERIAL-DimensionVector",
    /// `ISO-DimensionVector`: 
    ISO_DimensionVector, "ISO-DimensionVector",
    /// `ISO 8601 UTC Date Time - Basic Format`: 
    ISO8601_UTCDateTime_BasicFormat, "ISO8601-UTCDateTime-BasicFormat",
    /// `Iconic enumeration literal`: 
    IconicCue, "IconicCue",
    /// `IconicCue-image`: 
    IconicCue_image, "IconicCue-image",
    /// `IconicCueEnumeration-defaultValue`: 
    IconicCueEnumeration_defaultValue, "IconicCueEnumeration-defaultValue",
    /// `ImperialUnit`: 
    ImperialUnit, "ImperialUnit",
    /// `Pub inbook type`: 
    InBook, "InBook",
    /// `Pub incollection type`: 
    InCollection, "InCollection",
    /// `Pub inproceedings type`: 
    InProceedings, "InProceedings",
    /// `IndexedCollectionMember`: 
    IndexedCollectionMember, "IndexedCollectionMember",
    /// `IndexedList`: 
    IndexedList, "IndexedList",
    /// `Inertial Coordinate Frame`: 
    InertialCoordinateFrame, "InertialCoordinateFrame",
    /// `InertialCoordinateFrame-frameType`: 
    InertialCoordinateFrame_frameType, "InertialCoordinateFrame-frameType",
    /// `Inertial reference frame`: 
    InertialReferenceFrame, "InertialReferenceFrame",
    /// `Integer Datatype`: 
    IntegerDatatype, "IntegerDatatype",
    /// `Integer Encoding`: 
    IntegerEncodingType, "IntegerEncodingType",
    /// `Integer list`: 
    IntegerList, "IntegerList",
    /// `IntegerList-first`: 
    IntegerList_first, "IntegerList-first",
    /// `IntegerList-rest`: 
    IntegerList_rest, "IntegerList-rest",
    /// `Integer union list`: 
    IntegerUnionList, "IntegerUnionList",
    /// `Integer vector`: 
    IntegerVector, "IntegerVector",
    /// `Interpolated Table Type`: 
    InterpolatedTable, "InterpolatedTable",
    /// `Interval scale`: <p>The interval type allows for the degree of difference between items, but not the ratio between them. Examples include temperature with the Celsius scale, which has two defined points (the freezing and boiling point of water at specific conditions) and then separated into 100 intervals, date when measured from an arbitrary epoch (such as AD), percentage such as a percentage return on a stock,[16] location in Cartesian coordinates, and direction measured in degrees from true or magnetic north. Ratios are not meaningful since 20 °C cannot be said to be "twice as hot" as 10 °C, nor can multiplication/division be carried out between any two dates directly. However, ratios of differences can be expressed; for example, one difference can be twice another. Interval type variables are sometimes also called "scaled variables", but the formal mathematical term is an affine space (in this case an affine line).</p> <p>Characteristics: median, percentile &amp; Monotonic increasing (order (&lt;) &amp; totally ordered set</p>
    IntervalScale, "IntervalScale",
    /// `Kinesthetic Cue`: 
    KinestheticCue, "KinestheticCue",
    /// `KinestheticCue-code`: 
    KinestheticCue_code, "KinestheticCue-code",
    /// `KinestheticCueEnumeration-defaultValue`: 
    KinestheticCueEnumeration_defaultValue, "KinestheticCueEnumeration-defaultValue",
    /// `LIST`: 
    LIST, "LIST",
    /// `Large object`: 
    LargeObject, "LargeObject",
    /// `Large object`: 
    LargeObjectType, "LargeObjectType",
    /// `Latex String`: A type of string in which some characters may be wrapped with '\(' and '\) characters for LaTeX rendering.
    LatexString, "LatexString",
    /// `Limit type`: 
    LimitType, "LimitType",
    /// `List`: 
    List, "List",
    /// `List-first`: 
    List_first, "List-first",
    /// `List-rest`: 
    List_rest, "List-rest",
    /// `List type`: 
    ListType, "ListType",
    /// `Little Endian`: 
    LittleEndian, "LittleEndian",
    /// `Local Coordinate System Type`: 
    LocalCoordinateSystem, "LocalCoordinateSystem",
    /// `Logarithmic Unit`: Logarithmic units are abstract mathematical units that can be used to express any quantities (physical or mathematical) that are defined on a logarithmic scale, that is, as being proportional to the value of a logarithm function. Examples of logarithmic units include common units of information and entropy, such as the bit, and the byte, as well as units of relative signal strength magnitude such as the decibel.
    LogarithmicUnit, "LogarithmicUnit",
    /// `Long Integer Type`: 
    LongIntegerType, "LongIntegerType",
    /// `LongIntegerType-octets`: 
    LongIntegerType_octets, "LongIntegerType-octets",
    /// `Long Unsigned Integer Encoding`: 
    LongUnsignedIntegerEncoding, "LongUnsignedIntegerEncoding",
    /// `Low To High`: 
    LowToHigh, "LowToHigh",
    /// `Lunar Coordinate System`: 
    LunarCoordinateSystem, "LunarCoordinateSystem",
    /// `LunarCoordinateSystem-coordinateCenter`: 
    LunarCoordinateSystem_coordinateCenter, "LunarCoordinateSystem-coordinateCenter",
    /// `LunarCoordinateSystem-realization`: 
    LunarCoordinateSystem_realization, "LunarCoordinateSystem-realization",
    /// `LunarCoordinateSystem-xAxisDefinition`: 
    LunarCoordinateSystem_xAxisDefinition, "LunarCoordinateSystem-xAxisDefinition",
    /// `LunarCoordinateSystem-yAxisDefinition`: 
    LunarCoordinateSystem_yAxisDefinition, "LunarCoordinateSystem-yAxisDefinition",
    /// `LunarCoordinateSystem-zAxisDefinition`: 
    LunarCoordinateSystem_zAxisDefinition, "LunarCoordinateSystem-zAxisDefinition",
    /// `MASS PROPERTIES ARRAY`: 
    MASS_PROPERTIES_ARRAY, "MASS-PROPERTIES-ARRAY",
    /// `MATRIX`: 
    MATRIX, "MATRIX",
    /// `Matrix type  2x4`: 
    MATRIX_TYPE_2x4, "MATRIX-TYPE_2x4",
    /// `MKS-Unit`: 
    MKS_Unit, "MKS-Unit",
    /// `MULTI-DIMENSIONAL ARRAY`: 
    MULTi_DIMENSIONAL_ARRAY, "MULTi-DIMENSIONAL-ARRAY",
    /// `Major minor type`: 
    MajorMinorType, "MajorMinorType",
    /// `Pub manual type`: 
    Manual, "Manual",
    /// `Map Type`: 
    Map, "Map",
    /// `Map Type`: 
    MapType, "MapType",
    /// `Mars Coordinate System Type`: 
    MarsCoordinateSystem, "MarsCoordinateSystem",
    /// `MarsCoordinateSystem-coordinateCenter`: 
    MarsCoordinateSystem_coordinateCenter, "MarsCoordinateSystem-coordinateCenter",
    /// `Mass Properties Array Type`: 
    MassPropertiesArray, "MassPropertiesArray",
    /// `Mass Properties Array Type`: 
    MassPropertiesArrayType, "MassPropertiesArrayType",
    /// `Math Function Type`: 
    MathFunctionType, "MathFunctionType",
    /// `MathsFunctionType`: 
    MathsFunctionType, "MathsFunctionType",
    /// `Matrix Type`: 
    Matrix, "Matrix",
    /// `Matrix Type`: 
    MatrixType, "MatrixType",
    /// `Memory order type`: 
    MemoryOrderType, "MemoryOrderType",
    /// `Pub misc type`: 
    Misc, "Misc",
    /// `Modal Cue`: 
    ModalCue, "ModalCue",
    /// `ModalCue-duration`: 
    ModalCue_duration, "ModalCue-duration",
    /// `Modal Enumeration`: 
    ModalEnumeration, "ModalEnumeration",
    /// `ModalEnumeration-defaultValue`: 
    ModalEnumeration_defaultValue, "ModalEnumeration-defaultValue",
    /// `Multi Dimensional Array Type`: 
    MultiDimensionalArray, "MultiDimensionalArray",
    /// `Multi Dimensional Array Type`: 
    MultiDimensionalArrayType, "MultiDimensionalArrayType",
    /// `Multi dimensional data format`: 
    MultiDimensionalDataFormat, "MultiDimensionalDataFormat",
    /// `MultiDimensionalDataFormat-descriptor`: 
    MultiDimensionalDataFormat_descriptor, "MultiDimensionalDataFormat-descriptor",
    /// `Multi Dimensional Data Format Type`: 
    MultiDimensionalDataFormatType, "MultiDimensionalDataFormatType",
    /// `Multi modal enumeration`: 
    MultiModalEnumeration, "MultiModalEnumeration",
    /// `MultiModalEnumeration-auralCueEnumeration`: 
    MultiModalEnumeration_auralCueEnumeration, "MultiModalEnumeration-auralCueEnumeration",
    /// `MultiModalEnumeration-iconicCueEnumeration`: 
    MultiModalEnumeration_iconicCueEnumeration, "MultiModalEnumeration-iconicCueEnumeration",
    /// `MultiModalEnumeration-kinestheticCueEnumeration`: 
    MultiModalEnumeration_kinestheticCueEnumeration, "MultiModalEnumeration-kinestheticCueEnumeration",
    /// `MultiModalEnumeration-modalCueEnumeration`: 
    MultiModalEnumeration_modalCueEnumeration, "MultiModalEnumeration-modalCueEnumeration",
    /// `MultiModalEnumeration-visualCueEnumeration`: 
    MultiModalEnumeration_visualCueEnumeration, "MultiModalEnumeration-visualCueEnumeration",
    /// `Multi Modal Type`: 
    MultiModalType, "MultiModalType",
    /// `MultiModalType-auralCue`: 
    MultiModalType_auralCue, "MultiModalType-auralCue",
    /// `MultiModalType-iconicCue`: 
    MultiModalType_iconicCue, "MultiModalType-iconicCue",
    /// `MultiModalType-kinestheticCue`: 
    MultiModalType_kinestheticCue, "MultiModalType-kinestheticCue",
    /// `MultiModalType-modalCue`: 
    MultiModalType_modalCue, "MultiModalType-modalCue",
    /// `MultiModalType-visualCue`: 
    MultiModalType_visualCue, "MultiModalType-visualCue",
    /// `Bag`: A bag is a set in which elements may be repeated.
    MultiSet, "MultiSet",
    /// `N-Tuple Type`: 
    N_Tuple, "N-Tuple",
    /// `N-Tuple-elementType`: 
    N_Tuple_elementType, "N-Tuple-elementType",
    /// `N-Tuple Type`: 
    N_TupleType, "N-TupleType",
    /// `NIST SP~811 Comment`: 
    NIST_SP811_Comment, "NIST_SP811_Comment",
    /// `Numeric`: 
    NUMERIC, "NUMERIC",
    /// `No`: 
    No, "No",
    /// `Nominal scale`: A nominal scale differentiates between items or subjects based only on their names or (meta-)categories and other qualitative classifications they belong to; thus dichotomous data involves the construction of classifications as well as the classification of items. Discovery of an exception to a classification can be viewed as progress. Numbers may be used to represent the variables but the numbers do not have numerical value or relationship: For example, a Globally unique identifier. Examples of these classifications include gender, nationality, ethnicity, language, genre, style, biological species, and form. In a university one could also use hall of affiliation as an example.
    NominalScale, "NominalScale",
    /// `Non modifiable parameter`: Parameter is fixed, not modifiable.
    NonModifiableParameter, "NonModifiableParameter",
    /// `Non-negative union list`: 
    NonNegativeIntegerUnionList, "NonNegativeIntegerUnionList",
    /// `Non Rotating Coordinate Frame`: 
    NonRotatingInertialFrame, "NonRotatingInertialFrame",
    /// `NonRotatingInertialFrame-frameType`: 
    NonRotatingInertialFrame_frameType, "NonRotatingInertialFrame-frameType",
    /// `Numeric Type`: 
    NumericType, "NumericType",
    /// `NumericType-accuracy`: 
    NumericType_accuracy, "NumericType-accuracy",
    /// `NumericType-signedness`: 
    NumericType_signedness, "NumericType-signedness",
    /// `NumericUnionList`: 
    NumericUnionList, "NumericUnionList",
    /// `Off`: 
    OOST_OFF, "OOST_OFF",
    /// `On`: 
    OOST_ON, "OOST_ON",
    /// `Zero means off`: 
    OOST_ZERO_MEANS_OFF, "OOST_ZERO-MEANS-OFF",
    /// `Zero means on`: 
    OOST_ZERO_MEANS_ON, "OOST_ZERO-MEANS-ON",
    /// `OCTET Encoding`: 
    OctetEncoding, "OctetEncoding",
    /// `Octet Type`: 
    OctetType, "OctetType",
    /// `OffOnStateTypeEnumeration`: A discrete state enumeration whose values are 'off' and 'on'. The 'off' value is encoded as a zero (0) and the 'on' value as a one (1).
    OffOnStateTypeEnumeration, "OffOnStateTypeEnumeration",
    /// `On`: 
    On, "On",
    /// `On off state type`: 
    OnOffStateType, "OnOffStateType",
    /// `OnOffStateTypeEnumeration`: A discrete state enumeration whose values are 'off' and 'on'. The 'on' value is encoded as a zero (0) and the 'off' value as a one (1).
    OnOffStateTypeEnumeration, "OnOffStateTypeEnumeration",
    /// `One means off`: 
    OneMeansOff, "OneMeansOff",
    /// `Open`: 
    Open, "Open",
    /// `Open Close State Type`: 
    OpenCloseStateType, "OpenCloseStateType",
    /// `Ordered Collection Type`: 
    OrderedCollection, "OrderedCollection",
    /// `OrderedCollection-first`: 
    OrderedCollection_first, "OrderedCollection-first",
    /// `OrderedCollection-orderingRelation`: 
    OrderedCollection_orderingRelation, "OrderedCollection-orderingRelation",
    /// `OrderedCollection-rest`: 
    OrderedCollection_rest, "OrderedCollection-rest",
    /// `Ordered Collection Type`: 
    OrderedCollectionType, "OrderedCollectionType",
    /// `Ordered Tree Type`: 
    OrderedTree, "OrderedTree",
    /// `Ordered Tree Type`: 
    OrderedTreeType, "OrderedTreeType",
    /// `Ordered type`: 
    OrderedType, "OrderedType",
    /// `Ordinal scale`: The ordinal type allows for rank order (1st, 2nd, 3rd, etc.) by which data can be sorted, but still does not allow for relative degree of difference between them. Examples include, on one hand, dichotomous data with dichotomous (or dichotomized) values such as 'sick' vs. 'healthy' when measuring health, 'guilty' vs. 'innocent' when making judgments in courts, 'wrong/false' vs. 'right/true' when measuring truth value, and, on the other hand, non-dichotomous data consisting of a spectrum of values, such as 'completely agree', 'mostly agree', 'mostly disagree', 'completely disagree' when measuring opinion.
    OrdinalScale, "OrdinalScale",
    /// `Ordinal Data Type`: 
    OrdinalType, "OrdinalType",
    /// `Organization`: 
    Organization, "Organization",
    /// `PARTIAL ARRAY`: 
    PARTIAL_ARRAY, "PARTIAL-ARRAY",
    /// `Padding type`: 
    PaddingType, "PaddingType",
    /// `Parameter`: 
    Parameter, "Parameter",
    /// `Parameter modifiability type`: An enumeration of literals that signify whether a parameter is modifiable and if so, by whom.
    ParameterModifiabilityType, "ParameterModifiabilityType",
    /// `Partially Ordered`: 
    PartiallyOrdered, "PartiallyOrdered",
    /// `Percentage Type`: 
    Percentage, "Percentage",
    /// `Physical Address Type`: 
    PhysicalAddress, "PhysicalAddress",
    /// `Physical Constant`: A physical constant is a physical quantity that is generally believed to be both universal in nature and constant in time. It can be contrasted with a mathematical constant, which is a fixed numerical value but does not directly involve any physical measurement. There are many physical constants in science, some of the most widely recognized being the speed of light in vacuum c, Newton's gravitational constant G, Planck's constant h, the electric permittivity of free space ε0, and the elementary charge e. Physical constants can take many dimensional forms, or may be dimensionless depending on the system of quantities and units used.
    PhysicalConstant, "PhysicalConstant",
    /// `Polar Coordinate System Type`: 
    PolarCoordinates, "PolarCoordinates",
    /// `Polar Coordinate System Type`: 
    PolarCoordinatesType, "PolarCoordinatesType",
    /// `Polarity`: A "Tagged Enumeration" with the following instance(s): "negative", "positive".
    Polarity, "Polarity",
    /// `Positive Integer Type`: 
    PositiveIntegerType, "PositiveIntegerType",
    /// `Prefix`: 
    Prefix, "Prefix",
    /// `PrefixUnit`: 
    PrefixUnit, "PrefixUnit",
    /// `Pub proceedings type`: 
    Proceedings, "Proceedings",
    /// `Pub enumerated type`: 
    PubEnumeratedType, "PubEnumeratedType",
    /// `Quantifiable`: <p><em>Quantifiable</em> ascribes to some thing the capability of being measured, observed, or counted.</p>
    Quantifiable, "Quantifiable",
    /// `Quantity`: <p class="lm-para">A <b>quantity</b> is the measurement of an observable property of a particular object, event, or physical system. A quantity is always associated with the context of measurement (i.e. the thing measured, the measured value, the accuracy of measurement, etc.) whereas the underlying <b>quantity kind</b> is independent of any particular measurement. Thus, length is a quantity kind while the height of a rocket is a specific quantity of length; its magnitude that may be expressed in meters, feet, inches, etc. Examples of physical quantities include physical constants, such as the speed of light in a vacuum, Planck's constant, the electric permittivity of free space, and the fine structure constant. </p>  <p class="lm-para">In other words, quantities are quantifiable aspects of the world, such as the duration of a movie, the distance between two points, velocity of a car, the pressure of the atmosphere, and a person's weight; and units are used to describe their numerical measure.   <p class="lm-para">Many <b>quantity kinds</b> are related to each other by various physical laws, and as a result, the associated units of some quantity kinds can be expressed as products (or ratios) of powers of other quantity kinds (e.g., momentum is mass times velocity and velocity is defined as distance divided by time). In this way, some quantities can be calculated from other measured quantities using their associations to the quantity kinds in these expressions. These quantity kind relationships are also discussed in dimensional analysis. Those that cannot be so expressed can be regarded as "fundamental" in this sense.</p> <p class="lm-para">A quantity is distinguished from a "quantity kind" in that the former carries a value and the latter is a type specifier.</p>
    Quantity, "Quantity",
    /// `Quantity Kind`: A <b>Quantity Kind</b> is any observable property that can be  measured and quantified numerically. Familiar examples include physical properties such as length, mass, time, force, energy, power, electric charge, etc. Less familiar examples include currency, interest rate, price to earning ratio, and information capacity.
    QuantityKind, "QuantityKind",
    /// `Quantity Kind Dimension Vector`: <p class="lm-para">A  <em>Quantity Kind Dimension Vector</em> describes the dimensionality of a quantity kind in the context of a system of units. In the SI system of units, the dimensions of a quantity kind are expressed as a product of the basic physical dimensions mass (\(M\)), length (\(L\)), time (\(T\)) current (\(I\)), amount of substance (\(N\)), luminous intensity (\(J\)) and absolute temperature (\(\theta\)) as \(dim \, Q = L^{\alpha} \, M^{\beta} \, T^{\gamma} \, I ^{\delta} \, \theta ^{\epsilon} \, N^{\eta} \, J ^{\nu}\).</p>  <p class="lm-para">The rational powers of the dimensional exponents, \(\alpha, \, \beta, \, \gamma, \, \delta, \, \epsilon, \, \eta, \, \nu\), are positive, negative, or zero.</p>  <p class="lm-para">For example, the dimension of the physical quantity kind \(\it{speed}\) is \(\boxed{length/time}\), \(L/T\) or \(LT^{-1}\), and the dimension of the physical quantity kind force is \(\boxed{mass \times acceleration}\) or \(\boxed{mass \times (length/time)/time}\), \(ML/T^2\) or \(MLT^{-2}\) respectively.</p>
    QuantityKindDimensionVector, "QuantityKindDimensionVector",
    /// `CGS Dimension vector`: A <em>CGS Dimension Vector</em> is used to specify the dimensions for a C.G.S. quantity kind.
    QuantityKindDimensionVector_CGS, "QuantityKindDimensionVector_CGS",
    /// `CGS EMU Dimension vector`: A <em>CGS EMU Dimension Vector</em> is used to specify the dimensions for EMU C.G.S. quantity kind.
    QuantityKindDimensionVector_CGS_EMU, "QuantityKindDimensionVector_CGS-EMU",
    /// `CGS ESU Dimension vector`: A <em>CGS ESU Dimension Vector</em> is used to specify the dimensions for ESU C.G.S. quantity kind.
    QuantityKindDimensionVector_CGS_ESU, "QuantityKindDimensionVector_CGS-ESU",
    /// `CGS GAUSS Dimension vector`: A <em>CGS GAUSS Dimension Vector</em> is used to specify the dimensions for Gaussioan C.G.S. quantity kind.
    QuantityKindDimensionVector_CGS_GAUSS, "QuantityKindDimensionVector_CGS-GAUSS",
    /// `CGS LH Dimension vector`: A <em>CGS LH Dimension Vector</em> is used to specify the dimensions for Lorentz-Heaviside C.G.S. quantity kind.
    QuantityKindDimensionVector_CGS_LH, "QuantityKindDimensionVector_CGS-LH",
    /// `ISO Dimension vector`: 
    QuantityKindDimensionVector_ISO, "QuantityKindDimensionVector_ISO",
    /// `Imperial dimension vector`: 
    QuantityKindDimensionVector_Imperial, "QuantityKindDimensionVector_Imperial",
    /// `Quantity Kind Dimension vector (SI)`: 
    QuantityKindDimensionVector_SI, "QuantityKindDimensionVector_SI",
    /// `Quantity type`: 
    QuantityType, "QuantityType",
    /// `Quantity value`: A <i>Quantity Value</i> expresses the magnitude and kind of a quantity and is given by the product of a numerical value <code>n</code> and a unit of measure <code>U</code>. The number multiplying the unit is referred to as the numerical value of the quantity expressed in that unit. Refer to <a href="http://physics.nist.gov/Pubs/SP811/sec07.html">NIST SP 811 section 7</a> for more on quantity values.
    QuantityValue, "QuantityValue",
    /// `Quantity value type`: 
    QuantityValueType, "QuantityValueType",
    /// `QuantityValueType-basis`: 
    QuantityValueType_basis, "QuantityValueType-basis",
    /// `QuantityValueType-elementType`: 
    QuantityValueType_elementType, "QuantityValueType-elementType",
    /// `QuantityValueType-elementUnit`: 
    QuantityValueType_elementUnit, "QuantityValueType-elementUnit",
    /// `Relative Date And Time`: 
    RELATIVE_DATETIME, "RELATIVE-DATETIME",
    /// `Real number binary base`: A rational number can be expressed in the form m*b^e, where m (the mantissa), b (the base), and e (the exponent) are integers. In this case b is chosen to be 2, and then the values of m and e are determined given this choice of base.
    RN_BINARY_BASE, "RN_BINARY-BASE",
    /// `Real number decminal base`: A rational number can be expressed in the form m*b^e, where m (the mantissa), b (the base), and e (the exponent) are integers. In this case b is chosen to be 10, and then the values of m and e are determined given this choice of base.
    RN_DECIMAL_BASE, "RN_DECIMAL-BASE",
    /// `Ratio scale`: The ratio type takes its name from the fact that measurement is the estimation of the ratio between a magnitude of a continuous quantity and a unit magnitude of the same kind (Michell, 1997, 1999). A ratio scale possesses a meaningful (unique and non-arbitrary) zero value. Most measurement in the physical sciences and engineering is done on ratio scales. Examples include mass, length, duration, plane angle, energy and electric charge. In contrast to interval scales, ratios are now meaningful because having a non-arbitrary zero point makes it meaningful to say, for example, that one object has "twice the length" of another (= is "twice as long"). Very informally, many ratio scales can be described as specifying "how much" of something (i.e. an amount or magnitude) or "how many" (a count). The Kelvin temperature scale is a ratio scale because it has a unique, non-arbitrary zero point called absolute zero.
    RatioScale, "RatioScale",
    /// `Raw value tuple member`: 
    RawValueTupleMember, "RawValueTupleMember",
    /// `RawValueTupleMember-elementType`: 
    RawValueTupleMember_elementType, "RawValueTupleMember-elementType",
    /// `Raw value tuple member type`: 
    RawValueTupleMemberType, "RawValueTupleMemberType",
    /// `Real Datatype`: 
    RealDatatype, "RealDatatype",
    /// `RealDatatype-base`: 
    RealDatatype_base, "RealDatatype-base",
    /// `RealDatatype-maxExponent`: 
    RealDatatype_maxExponent, "RealDatatype-maxExponent",
    /// `RealDatatype-precision`: 
    RealDatatype_precision, "RealDatatype-precision",
    /// `Real Double Precision Type`: 
    RealDoublePrecisionType, "RealDoublePrecisionType",
    /// `Real number type`: This is the class of data values that approximate real numbers in finite precision. Often, such values are expressed in "mantissa, base, exponent" form. Any rational number can be expressed in the form m*b^e, where m (the mantissa), b (the base), and e (the exponent) are integers. Typically, b is chosen to be either 2 or 10, and then the values of m and e are determined given the choice of base.
    RealNumberType, "RealNumberType",
    /// `Real Single Precision Type`: 
    RealSinglePrecisionType, "RealSinglePrecisionType",
    /// `Record Type`: 
    Record, "Record",
    /// `Reference Data Type`: 
    ReferenceDatatype, "ReferenceDatatype",
    /// `Reference Frame`: 
    ReferenceFrame, "ReferenceFrame",
    /// `ReferenceFrame-comment`: 
    ReferenceFrame_comment, "ReferenceFrame-comment",
    /// `ReferenceFrame-description`: 
    ReferenceFrame_description, "ReferenceFrame-description",
    /// `ReferenceFrame-frameType`: 
    ReferenceFrame_frameType, "ReferenceFrame-frameType",
    /// `ReferenceFrame-realization`: 
    ReferenceFrame_realization, "ReferenceFrame-realization",
    /// `ReferenceFrame-xAxisDefinition`: 
    ReferenceFrame_xAxisDefinition, "ReferenceFrame-xAxisDefinition",
    /// `ReferenceFrame-xCoordinateDefinition`: 
    ReferenceFrame_xCoordinateDefinition, "ReferenceFrame-xCoordinateDefinition",
    /// `ReferenceFrame-yAxisDefinition`: 
    ReferenceFrame_yAxisDefinition, "ReferenceFrame-yAxisDefinition",
    /// `ReferenceFrame-yCoordinateDefinition`: 
    ReferenceFrame_yCoordinateDefinition, "ReferenceFrame-yCoordinateDefinition",
    /// `ReferenceFrame-zAxisDefinition`: 
    ReferenceFrame_zAxisDefinition, "ReferenceFrame-zAxisDefinition",
    /// `ReferenceFrame-zCoordinateDefinition`: 
    ReferenceFrame_zCoordinateDefinition, "ReferenceFrame-zCoordinateDefinition",
    /// `Rotating reference frame`: 
    RotatingReferenceFrame, "RotatingReferenceFrame",
    /// `Rule`: 
    Rule, "Rule",
    /// `Rule Type`: 
    RuleType, "RuleType",
    /// `Signed`: 
    SIGNED, "SIGNED",
    /// `STATE SPACE MATRIX`: 
    STATE_SPACE_MATRIX, "STATE-SPACE-MATRIX",
    /// `STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES`: 
    STATE_VECTOR_TYPE_LIST_MEMBER_MASS_PROPERTIES_ELEMENT_1, "STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES-ELEMENT-1",
    /// `STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES`: 
    STATE_VECTOR_TYPE_LIST_MEMBER_MASS_PROPERTIES_ELEMENT_2, "STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES-ELEMENT-2",
    /// `STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES`: 
    STATE_VECTOR_TYPE_LIST_MEMBER_MASS_PROPERTIES_ELEMENT_3, "STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES-ELEMENT-3",
    /// `STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES`: 
    STATE_VECTOR_TYPE_LIST_MEMBER_MASS_PROPERTIES_ELEMENT_4, "STATE-VECTOR-TYPE-LIST-MEMBER_MASS-PROPERTIES-ELEMENT-4",
    /// `STATE-VECTOR-TYPE-LIST_MASS-PROPERTIES`: 
    STATE_VECTOR_TYPE_LIST_MASS_PROPERTIES, "STATE-VECTOR-TYPE-LIST_MASS-PROPERTIES",
    /// `STATE-VECTOR-MASS-PROPERTIES`: 
    STATE_VECTOR_MASS_PROPERTIES, "STATE-VECTOR_MASS-PROPERTIES",
    /// `STRING`: 
    STRING, "STRING",
    /// `Scalar`: 
    Scalar, "Scalar",
    /// `Scalar Datatype`: 
    ScalarDatatype, "ScalarDatatype",
    /// `ScalarDatatype-bitOrder`: 
    ScalarDatatype_bitOrder, "ScalarDatatype-bitOrder",
    /// `ScalarDatatype-bits`: 
    ScalarDatatype_bits, "ScalarDatatype-bits",
    /// `ScalarDatatype-byteOrder`: 
    ScalarDatatype_byteOrder, "ScalarDatatype-byteOrder",
    /// `ScalarDatatype-bytes`: 
    ScalarDatatype_bytes, "ScalarDatatype-bytes",
    /// `ScalarDatatype-encoding`: 
    ScalarDatatype_encoding, "ScalarDatatype-encoding",
    /// `ScalarDatatype-length`: 
    ScalarDatatype_length, "ScalarDatatype-length",
    /// `ScalarDatatype-maxExclusive`: 
    ScalarDatatype_maxExclusive, "ScalarDatatype-maxExclusive",
    /// `ScalarDatatype-maxInclusive`: 
    ScalarDatatype_maxInclusive, "ScalarDatatype-maxInclusive",
    /// `ScalarDatatype-minExclusive`: 
    ScalarDatatype_minExclusive, "ScalarDatatype-minExclusive",
    /// `ScalarDatatype-minInclusive`: 
    ScalarDatatype_minInclusive, "ScalarDatatype-minInclusive",
    /// `ScalarDatatype-rdfsDatatype`: 
    ScalarDatatype_rdfsDatatype, "ScalarDatatype-rdfsDatatype",
    /// `Scale`: Scales (also called "scales of measurement" or "levels of measurement")  are expressions that typically refer to the theory of scale types.
    Scale, "Scale",
    /// `Scale type`: 
    ScaleType, "ScaleType",
    /// `ScaledUnit`: 
    ScaledUnit, "ScaledUnit",
    /// `Sequence`: 
    Sequence, "Sequence",
    /// `Sequence-first`: 
    Sequence_first, "Sequence-first",
    /// `Sequence-rest`: 
    Sequence_rest, "Sequence-rest",
    /// `Sequence`: 
    SequenceType, "SequenceType",
    /// `Set Type`: 
    Set, "Set",
    /// `Set Type`: 
    SetType, "SetType",
    /// `Short Integer Type`: 
    ShortIntegerType, "ShortIntegerType",
    /// `Short Signed Integer Encoding`: 
    ShortSignedIntegerEncoding, "ShortSignedIntegerEncoding",
    /// `Short Unsigned Integer Encoding`: 
    ShortUnsignedIntegerEncoding, "ShortUnsignedIntegerEncoding",
    /// `Signed Big Integer Type`: 
    SignedBigIntegerType, "SignedBigIntegerType",
    /// `SignedBigIntegerType-literal`: 
    SignedBigIntegerType_literal, "SignedBigIntegerType-literal",
    /// `SignedBigIntegerType-maxInclusive`: 
    SignedBigIntegerType_maxInclusive, "SignedBigIntegerType-maxInclusive",
    /// `SignedBigIntegerType-minInclusive`: 
    SignedBigIntegerType_minInclusive, "SignedBigIntegerType-minInclusive",
    /// `Signed Integer Encoding`: 
    SignedIntegerEncoding, "SignedIntegerEncoding",
    /// `Signed Integer Type`: 
    SignedIntegerType, "SignedIntegerType",
    /// `Signed Long Integer Type`: 
    SignedLongIntegerType, "SignedLongIntegerType",
    /// `SignedLongIntegerType-abbreviation`: 
    SignedLongIntegerType_abbreviation, "SignedLongIntegerType-abbreviation",
    /// `SignedLongIntegerType-maxInclusive`: 
    SignedLongIntegerType_maxInclusive, "SignedLongIntegerType-maxInclusive",
    /// `SignedLongIntegerType-minInclusive`: 
    SignedLongIntegerType_minInclusive, "SignedLongIntegerType-minInclusive",
    /// `Signed Integer Type`: 
    SignedMediumIntegerType, "SignedMediumIntegerType",
    /// `Signed Short Integer Type`: 
    SignedShortIntegerType, "SignedShortIntegerType",
    /// `SignedShortIntegerType-abbreviation`: 
    SignedShortIntegerType_abbreviation, "SignedShortIntegerType-abbreviation",
    /// `Signed Type`: 
    SignedType, "SignedType",
    /// `SignedType-signedness`: 
    SignedType_signedness, "SignedType-signedness",
    /// `Signed Variable Length Integer Type`: 
    SignedVariableLengthIntegerType, "SignedVariableLengthIntegerType",
    /// `Signedness type`: 
    SignednessType, "SignednessType",
    /// `Single Precision Real Encoding`: 
    SinglePrecisionRealEncoding, "SinglePrecisionRealEncoding",
    /// `Single Precision Type`: 
    SinglePrecisionType, "SinglePrecisionType",
    /// `Spline calibrator`: 
    SplineCalibrator, "SplineCalibrator",
    /// `Spline calibrator type`: 
    SplineCalibratorType, "SplineCalibratorType",
    /// `Spline point`: 
    SplinePoint, "SplinePoint",
    /// `Spline point type`: 
    SplinePointType, "SplinePointType",
    /// `StandardsUnit`: 
    StandardsUnit, "StandardsUnit",
    /// `State Space Matrix Type`: 
    StateSpaceMatrix, "StateSpaceMatrix",
    /// `State Space Matrix Type`: 
    StateSpaceMatrixType, "StateSpaceMatrixType",
    /// `State Space Vector Type`: 
    StateSpaceVector, "StateSpaceVector",
    /// `StateSpaceVector-coordinateSystem`: 
    StateSpaceVector_coordinateSystem, "StateSpaceVector-coordinateSystem",
    /// `State Space Vector Type`: 
    StateSpaceVectorType, "StateSpaceVectorType",
    /// `State Vector Type`: 
    StateVectorType, "StateVectorType",
    /// `Statement`: 
    Statement, "Statement",
    /// `String Encoding Type`: 
    StringEncodingType, "StringEncodingType",
    /// `String list`: 
    StringList, "StringList",
    /// `StringList-first`: 
    StringList_first, "StringList-first",
    /// `StringList-rest`: 
    StringList_rest, "StringList-rest",
    /// `String Type`: 
    StringType, "StringType",
    /// `StringType-dimensionality`: 
    StringType_dimensionality, "StringType-dimensionality",
    /// `StringType-elementType`: 
    StringType_elementType, "StringType-elementType",
    /// `StringType-isByteString`: 
    StringType_isByteString, "StringType-isByteString",
    /// `StringType-maxLength`: 
    StringType_maxLength, "StringType-maxLength",
    /// `StringType-typeMatrix`: 
    StringType_typeMatrix, "StringType-typeMatrix",
    /// `String UTF16 Type`: 
    StringUTF16, "StringUTF16",
    /// `StringUTF16-elementType`: 
    StringUTF16_elementType, "StringUTF16-elementType",
    /// `String UTF8 Type`: 
    StringUTF8, "StringUTF8",
    /// `StringUTF8-elementType`: 
    StringUTF8_elementType, "StringUTF8-elementType",
    /// `Structured Data Type`: 
    StructuredDatatype, "StructuredDatatype",
    /// `StructuredDatatype-elementType`: 
    StructuredDatatype_elementType, "StructuredDatatype-elementType",
    /// `Subject Area`: A "Enumeration Literal".
    SubjectArea, "SubjectArea",
    /// `Symbol`: 
    Symbol, "Symbol",
    /// `System modifiable parameter`: Parameter is modifiable by a (computer) system.
    SystemModifiableParameter, "SystemModifiableParameter",
    /// `System of Quantity Kinds`: A system of quantity kinds is a set of one or more quantity kinds together with a set of zero or more algebraic equations that define relationships between quantity kinds in the set. In the physical sciences, the equations relating quantity kinds are typically physical laws and definitional relations, and constants of proportionality. Examples include Newton’s First Law of Motion, Coulomb’s Law, and the definition of velocity as the instantaneous change in position.  In almost all cases, the system identifies a subset of base quantity kinds. The base set is chosen so that all other quantity kinds of interest can be derived from the base quantity kinds and the algebraic equations. If the unit system is explicitly associated with a quantity kind system, then the unit system must define at least one unit for each quantity kind.  From a scientific point of view, the division of quantities into base quantities and derived quantities is a matter of convention.
    SystemOfQuantityKinds, "SystemOfQuantityKinds",
    /// `System of Units`: A system of units is a set of units which are chosen as the reference scales for some set of quantity kinds together with the definitions of each unit. Units may be defined by experimental observation or by proportion to another unit not included in the system. If the unit system is explicitly associated with a quantity kind system, then the unit system must define at least one unit for each quantity kind.
    SystemOfUnits, "SystemOfUnits",
    /// `TABLE-COMPOSITION-EXAMPLE-SUBTREE_1`: 
    TABLE_COMPOSITION_EXAMPLE_SUBTREE_1, "TABLE-COMPOSITION-EXAMPLE-SUBTREE_1",
    /// `TABLE-COMPOSITION-EXAMPLE-SUBTREE_2`: 
    TABLE_COMPOSITION_EXAMPLE_SUBTREE_2, "TABLE-COMPOSITION-EXAMPLE-SUBTREE_2",
    /// `TABLE-COMPOSITION-SUBTREE-1`: 
    TABLE_COMPOSITION_SUBTREE_LIST_MEMBER_1, "TABLE-COMPOSITION-SUBTREE-LIST-MEMBER_1",
    /// `TABLE-COMPOSITION-SUBTREE_2`: 
    TABLE_COMPOSITION_SUBTREE_LIST_MEMBER_2, "TABLE-COMPOSITION-SUBTREE-LIST-MEMBER_2",
    /// `TABLE-COMPOSTION-SUBTREES-EXAMPLE_1`: 
    TABLE_COMPOSTION_SUBTREE_LIST_EXAMPLE_1, "TABLE-COMPOSTION-SUBTREE-LIST-EXAMPLE_1",
    /// `Table type  2x4`: 
    TABLE_TYPE_2x4, "TABLE-TYPE_2x4",
    /// `TIME`: 
    TIME, "TIME",
    /// `TIME-SERIES-TABLE-A-2x4`: 
    TIME_SERIES_TABLE_A_2x4, "TIME-SERIES-TABLE-A-2x4",
    /// `TIME-SERIES-TABLE-B-2x4`: 
    TIME_SERIES_TABLE_B_2x4, "TIME-SERIES-TABLE-B-2x4",
    /// `True`: 
    TRUE, "TRUE",
    /// `Table Type`: 
    Table, "Table",
    /// `Table-byRow`: 
    Table_byRow, "Table-byRow",
    /// `Table-columns`: 
    Table_columns, "Table-columns",
    /// `Table-dimensionality`: 
    Table_dimensionality, "Table-dimensionality",
    /// `Table-rows`: 
    Table_rows, "Table-rows",
    /// `Table Type`: A Table Type is a data type that specifies the properties of table data structures. A table is both a mode of visual communication and a means of arranging data. The use of tables is pervasive throughout NASA The precise conventions and terminology for describing tables varies depending on the context. Moreover, tables differ significantly in variety, structure, flexibility, notation, representation and use.
    TableType, "TableType",
    /// `Tagged Enumeration`: 
    TaggedEnumeration, "TaggedEnumeration",
    /// `TaggedEnumeration-code`: 
    TaggedEnumeration_code, "TaggedEnumeration-code",
    /// `Pub techreport type`: 
    TechReport, "TechReport",
    /// `Term`: 
    Term, "Term",
    /// `Text String Type`: 
    TextStringType, "TextStringType",
    /// `Three-Tuple Type`: 
    Three_Tuple, "Three-Tuple",
    /// `Three-Tuple-elementType`: 
    Three_Tuple_elementType, "Three-Tuple-elementType",
    /// `Three-Tuple-elementTypeCount`: 
    Three_Tuple_elementTypeCount, "Three-Tuple-elementTypeCount",
    /// `Three Body Rotating Coordinate System Type`: 
    ThreeBodyRotatingCoordinateSystem, "ThreeBodyRotatingCoordinateSystem",
    /// `Three-Tuple Type`: 
    ThreeTupleType, "ThreeTupleType",
    /// `Time`: The class of data values that denote a point in time. Time values may be encoded in a 12-hour clock or a 24-hour clock, such as 1:35 AM, or 13:35.
    Time, "Time",
    /// `Time-type`: 
    Time_type, "Time-type",
    /// `Time data type`: 
    TimeDataType, "TimeDataType",
    /// `Time interval`: A relative interval that is an increment in time. For example, this is used in time series arrays to express the time point of a vector of values.
    TimeInterval, "TimeInterval",
    /// `TimeInterval-type`: 
    TimeInterval_type, "TimeInterval-type",
    /// `Time Series Array Type`: 
    TimeSeriesArray, "TimeSeriesArray",
    /// `TimeSeriesArray-dimensionVector`: 
    TimeSeriesArray_dimensionVector, "TimeSeriesArray-dimensionVector",
    /// `TimeSeriesArray-incrementDatatype`: 
    TimeSeriesArray_incrementDatatype, "TimeSeriesArray-incrementDatatype",
    /// `TimeSeriesArray-vector`: 
    TimeSeriesArray_vector, "TimeSeriesArray-vector",
    /// `Time Series Array Type`: 
    TimeSeriesArrayType, "TimeSeriesArrayType",
    /// `Time Type`: 
    TimeStringType, "TimeStringType",
    /// `Totally Ordered`: 
    TotallyOrdered, "TotallyOrdered",
    /// `Trajectory Coordinate System`: 
    TrajectoryCoordinateSystem, "TrajectoryCoordinateSystem",
    /// `TransformType`: 
    TransformType, "TransformType",
    /// `Tree Type`: 
    Tree, "Tree",
    /// `TreeListMember`: 
    TreeListMember, "TreeListMember",
    /// `Tree Type`: 
    TreeType, "TreeType",
    /// `Triplet`: 
    Triplet, "Triplet",
    /// `true`: 
    True, "True",
    /// `Tuple Type`: 
    Tuple, "Tuple",
    /// `Tuple-elementType`: 
    Tuple_elementType, "Tuple-elementType",
    /// `Tuple-elementTypeCount`: 
    Tuple_elementTypeCount, "Tuple-elementTypeCount",
    /// `Tuple-length`: 
    Tuple_length, "Tuple-length",
    /// `Tuple Member Type`: 
    TupleMember, "TupleMember",
    /// `TupleMember-elementType`: 
    TupleMember_elementType, "TupleMember-elementType",
    /// `TupleMember-orderInStructure`: 
    TupleMember_orderInStructure, "TupleMember-orderInStructure",
    /// `Tuple Member Type`: 
    TupleMemberType, "TupleMemberType",
    /// `Tuple Type`: 
    TupleType, "TupleType",
    /// `Two-Tuple Type`: 
    Two_Tuple, "Two-Tuple",
    /// `Two-Tuple-elementType`: 
    Two_Tuple_elementType, "Two-Tuple-elementType",
    /// `Two-Tuple-elementTypeCount`: 
    Two_Tuple_elementTypeCount, "Two-Tuple-elementTypeCount",
    /// `Two-Tuple Type`: 
    TwoTupleType, "TwoTupleType",
    /// `Type list`: 
    TypeList, "TypeList",
    /// `Type matrix`: Members of this class are matrix data structures that describe the datatypes of a class of matrices. That is, the members of this class are matrices with cells that contain datatypes (c.f. type:Datatype) and are used to describe the datatype structure of other matrices.
    TypeMatrix, "TypeMatrix",
    /// `Type Vector Type`: A Type Vector is a vector whose elements are data types. They are used, for instance, to specify the type of each component of a vector or class of vectors. A <em>Type Vector Type</em> is a further abstraction that specifies the structure of Type Vectors.
    TypeVector, "TypeVector",
    /// `TypeVector-objectValue`: 
    TypeVector_objectValue, "TypeVector-objectValue",
    /// `TypeVector-type`: 
    TypeVector_type, "TypeVector-type",
    /// `TypeVector-typeVector`: 
    TypeVector_typeVector, "TypeVector-typeVector",
    /// `case-insensitive UCUM code`: Lexical pattern for the case-insensitive version of UCUM code
    UCUMci, "UCUMci",
    /// `case-insensitive UCUM term`: Lexical pattern for the terminal symbols in the case-insensitive version of UCUM code
    UCUMci_term, "UCUMci-term",
    /// `case-sensitive UCUM code`: Lexical pattern for the case-sensitive version of UCUM code
    UCUMcs, "UCUMcs",
    /// `case-sensitive UCUM terminal`: Lexical pattern for the terminal symbols in the case-sensitive version of UCUM code
    UCUMcs_term, "UCUMcs-term",
    /// `UNARY-FUNCTION`: 
    UNARY_FUNCTION, "UNARY-FUNCTION",
    /// `Unsigned`: 
    UNSIGNED, "UNSIGNED",
    /// `UNSIGNED-INTEGER`: 
    UNSIGNED_INTEGER, "UNSIGNED-INTEGER",
    /// `UTC Date Time`: 
    UTC_DATETIME, "UTC-DATETIME",
    /// `UTC DAY TIME`: 
    UTC_DAYTIME, "UTC-DAYTIME",
    /// `UTC Day time`: 
    UTC_DayTime, "UTC-DayTime",
    /// `UTF16-CHAR`: 
    UTF16_CHAR, "UTF16-CHAR",
    /// `UTF16 String`: 
    UTF16_STRING, "UTF16-STRING",
    /// `UTF-16 String`: 
    UTF16_StringEncoding, "UTF16-StringEncoding",
    /// `UTF8-CHAR`: 
    UTF8_CHAR, "UTF8-CHAR",
    /// `UTF8 String`: 
    UTF8_STRING, "UTF8-STRING",
    /// `UTF-8 Encoding`: 
    UTF8_StringEncoding, "UTF8-StringEncoding",
    /// `Unit`: A unit of measure, or unit, is a particular quantity value that has been chosen as a scale for measuring other quantities the same kind (more generally of equivalent dimension). For example, the meter is a quantity of length that has been rigorously defined and standardized by the BIPM (International Board of Weights and Measures). Any measurement of the length can be expressed as a number multiplied by the unit meter. More formally, the value of a physical quantity Q with respect to a unit (U) is expressed as the scalar multiple of a real number (n) and U, as  \(Q = nU\).
    Unit, "Unit",
    /// `Unordered`: 
    Unordered, "Unordered",
    /// `Pub unpublished type`: 
    Unpublished, "Unpublished",
    /// `Unsigned Big Integer Type`: 
    UnsignedBigIntegerType, "UnsignedBigIntegerType",
    /// `UnsignedBigIntegerType-abbreviation`: 
    UnsignedBigIntegerType_abbreviation, "UnsignedBigIntegerType-abbreviation",
    /// `UnsignedBigIntegerType-maxInclusive`: 
    UnsignedBigIntegerType_maxInclusive, "UnsignedBigIntegerType-maxInclusive",
    /// `UnsignedBigIntegerType-minInclusive`: 
    UnsignedBigIntegerType_minInclusive, "UnsignedBigIntegerType-minInclusive",
    /// `Unsigned Integer Encoding`: 
    UnsignedIntegerEncoding, "UnsignedIntegerEncoding",
    /// `Unsigned Integer Type`: 
    UnsignedIntegerType, "UnsignedIntegerType",
    /// `Unsigned Long Integer Type`: 
    UnsignedLongIntegerType, "UnsignedLongIntegerType",
    /// `UnsignedLongIntegerType-literal`: 
    UnsignedLongIntegerType_literal, "UnsignedLongIntegerType-literal",
    /// `UnsignedLongIntegerType-maxInclusive`: 
    UnsignedLongIntegerType_maxInclusive, "UnsignedLongIntegerType-maxInclusive",
    /// `UnsignedLongIntegerType-minInclusive`: 
    UnsignedLongIntegerType_minInclusive, "UnsignedLongIntegerType-minInclusive",
    /// `Signed Integer Type`: 
    UnsignedMediumIntegerType, "UnsignedMediumIntegerType",
    /// `Unsigned Short Integer Type`: 
    UnsignedShortIntegerType, "UnsignedShortIntegerType",
    /// `UnsignedShortIntegerType-abbreviation`: 
    UnsignedShortIntegerType_abbreviation, "UnsignedShortIntegerType-abbreviation",
    /// `UnsignedShortIntegerType-maxInclusive`: 
    UnsignedShortIntegerType_maxInclusive, "UnsignedShortIntegerType-maxInclusive",
    /// `UnsignedShortIntegerType-minInclusive`: 
    UnsignedShortIntegerType_minInclusive, "UnsignedShortIntegerType-minInclusive",
    /// `Unsigned Type`: 
    UnsignedType, "UnsignedType",
    /// `UnsignedType-signedness`: 
    UnsignedType_signedness, "UnsignedType-signedness",
    /// `Unsigned Variable Length Integer Type`: 
    UnsignedVariableLengthIntegerType, "UnsignedVariableLengthIntegerType",
    /// `User modifiable parameter`: Parameter is modifiable by a user.
    UserModifiableParameter, "UserModifiableParameter",
    /// `User Quantity Kind`: 
    UserQuantityKind, "UserQuantityKind",
    /// `VECTOR`: 
    VECTOR, "VECTOR",
    /// `VECTOR 3D`: 
    VECTOR_3D, "VECTOR_3D",
    /// `Variable Interval Time Series Array Type`: 
    VariableIntervalTimeSeriesArray, "VariableIntervalTimeSeriesArray",
    /// `Variable Interval Time Series Array Type`: A Variable Interval Time Series Array Type is a data type that specifies the properties of arrays that hold time series data that has been sampled over non-uniformly spaced time intervals. A time series is a sequence of data points, measured typically at successive times, spaced at uniform or non-uniform time intervals. For variable interval time series, the successive time intervals may follow a repeating pattern, or may be completely random.
    VariableIntervalTimeSeriesArrayType, "VariableIntervalTimeSeriesArrayType",
    /// `Variable Length Integer Type`: 
    VariableLengthIntegerType, "VariableLengthIntegerType",
    /// `VariableLengthIntegerType-maxBits`: 
    VariableLengthIntegerType_maxBits, "VariableLengthIntegerType-maxBits",
    /// `VariableLengthIntegerType-minBits`: 
    VariableLengthIntegerType_minBits, "VariableLengthIntegerType-minBits",
    /// `Vector Type`: 
    Vector, "Vector",
    /// `Vector-dimension`: 
    Vector_dimension, "Vector-dimension",
    /// `Vector-dimensionality`: 
    Vector_dimensionality, "Vector-dimensionality",
    /// `Vector-referenceFrame`: 
    Vector_referenceFrame, "Vector-referenceFrame",
    /// `Vector Array Type`: 
    VectorArray, "VectorArray",
    /// `VectorArray-vector`: 
    VectorArray_vector, "VectorArray-vector",
    /// `Vector Array Type`: 
    VectorArrayType, "VectorArrayType",
    /// `Vector Type`: 
    VectorType, "VectorType",
    /// `Vehicle coordinate system`: 
    VehicleCoordinateSystem, "VehicleCoordinateSystem",
    /// `VehicleCoordinateSystem-pitchRotationDefinition`: 
    VehicleCoordinateSystem_pitchRotationDefinition, "VehicleCoordinateSystem-pitchRotationDefinition",
    /// `VehicleCoordinateSystem-rollRotationDefinition`: 
    VehicleCoordinateSystem_rollRotationDefinition, "VehicleCoordinateSystem-rollRotationDefinition",
    /// `VehicleCoordinateSystem-yawRotationDefinition`: 
    VehicleCoordinateSystem_yawRotationDefinition, "VehicleCoordinateSystem-yawRotationDefinition",
    /// `Verifiable`: An aspect class that holds properties that provide external knowledge and specifications of a given resource.
    Verifiable, "Verifiable",
    /// `Visual Cue`: 
    VisualCue, "VisualCue",
    /// `VisualCueEnumeration-defaultValue`: 
    VisualCueEnumeration_defaultValue, "VisualCueEnumeration-defaultValue",
    /// `Dry`: 
    WDST_DRY, "WDST_DRY",
    /// `Wet`: 
    WDST_WET, "WDST_WET",
    /// `Wet dry state type`: 
    WetDryStateType, "WetDryStateType",
    /// `Wikipedia`: 
    Wikipedia, "Wikipedia",
    /// `Word Aligned`: 
    WordAligned, "WordAligned",
    /// `Year Day Time`: 
    YDT, "YDT",
    /// `Yes`: 
    Yes, "Yes",
    /// `Yes no type`: 
    YesNoType, "YesNoType",
    /// `abbreviation`: 
    abbreviation, "abbreviation",
    /// `accuracy`: 
    accuracy, "accuracy",
    /// `acronym`: 
    acronym, "acronym",
    /// `alignment`: 
    alignment, "alignment",
    /// `allowed pattern`: 
    allowedPattern, "allowedPattern",
    /// `allowed unit of system`: 
    allowedUnitOfSystem, "allowedUnitOfSystem",
    /// `ANSI SQL Name`: 
    ansiSQLName, "ansiSQLName",
    /// `applicable CGS unit`: 
    applicableCGSUnit, "applicableCGSUnit",
    /// `applicable ISO unit`: 
    applicableISOUnit, "applicableISOUnit",
    /// `applicable Imperial unit`: 
    applicableImperialUnit, "applicableImperialUnit",
    /// `applicable physical constant`: 
    applicablePhysicalConstant, "applicablePhysicalConstant",
    /// `applicable Planck unit`: 
    applicablePlanckUnit, "applicablePlanckUnit",
    /// `applicable SI unit`: 
    applicableSIUnit, "applicableSIUnit",
    /// `applicable US Customary unit`: 
    applicableUSCustomaryUnit, "applicableUSCustomaryUnit",
    /// `applicable unit`: 
    applicableUnit, "applicableUnit",
    /// `arg1Type`: 
    arg1Type, "arg1Type",
    /// `arg2Type`: 
    arg2Type, "arg2Type",
    /// `arg3Type`: 
    arg3Type, "arg3Type",
    /// `argType`: 
    argType, "argType",
    /// `aural cue`: 
    auralCue, "auralCue",
    /// `aural cue enumeration`: 
    auralCueEnumeration, "auralCueEnumeration",
    /// `base`: 
    base, "base",
    /// `base CGS unit dimensions`: 
    baseCGSUnitDimensions, "baseCGSUnitDimensions",
    /// `base dimension enumeration`: 
    baseDimensionEnumeration, "baseDimensionEnumeration",
    /// `base ISO unit dimensions`: 
    baseISOUnitDimensions, "baseISOUnitDimensions",
    /// `base Imperial unit dimensions`: 
    baseImperialUnitDimensions, "baseImperialUnitDimensions",
    /// `base SI unit dimensions`: 
    baseSIUnitDimensions, "baseSIUnitDimensions",
    /// `base US Customary unit dimensions`: 
    baseUSCustomaryUnitDimensions, "baseUSCustomaryUnitDimensions",
    /// `base unit dimensions`: 
    baseUnitDimensions, "baseUnitDimensions",
    /// `is base unit of system`: 
    baseUnitOfSystem, "baseUnitOfSystem",
    /// `basis`: 
    basis, "basis",
    /// `belongs to system of quantities`: 
    belongsToSystemOfQuantities, "belongsToSystemOfQuantities",
    /// `bit order`: 
    bitOrder, "bitOrder",
    /// `bits`: 
    bits, "bits",
    /// `bounded`: 
    bounded, "bounded",
    /// `by row`: 
    byRow, "byRow",
    /// `byte order`: 
    byteOrder, "byteOrder",
    /// `bytes`: 
    bytes, "bytes",
    /// `C Language name`: Datatype name in the C programming language
    cName, "cName",
    /// `cardinality`: 
    cardinality, "cardinality",
    /// `categorized as`: 
    categorizedAs, "categorizedAs",
    /// `citation`: 
    citation, "citation",
    /// `code`: 
    code, "code",
    /// `is coherent unit of system`: 
    coherentUnitOfSystem, "coherentUnitOfSystem",
    /// `coherent unit system`: 
    coherentUnitSystem, "coherentUnitSystem",
    /// `columns`: 
    columns, "columns",
    /// `conversion coefficient`: 
    conversionCoefficient, "conversionCoefficient",
    /// `conversion multiplier`: 
    conversionMultiplier, "conversionMultiplier",
    /// `conversion offset`: 
    conversionOffset, "conversionOffset",
    /// `coordinate center`: 
    coordinateCenter, "coordinateCenter",
    /// `coordinate system`: 
    coordinateSystem, "coordinateSystem",
    /// `coordinate system frame`: 
    coordinateSystemFrame, "coordinateSystemFrame",
    /// `currency exponent`: 
    currencyExponent, "currencyExponent",
    /// `data encoding`: 
    dataEncoding, "dataEncoding",
    /// `data order`: 
    dataOrder, "dataOrder",
    /// `data structure`: 
    dataStructure, "dataStructure",
    /// `datatype`: 
    dataType, "dataType",
    /// `dbpedia match`: 
    dbpediaMatch, "dbpediaMatch",
    /// `default`: 
    default, "default",
    /// `Default Value`: 
    defaultValue, "defaultValue",
    /// `defined unit of system`: 
    definedUnitOfSystem, "definedUnitOfSystem",
    /// `denominator dimension vector`: 
    denominatorDimensionVector, "denominatorDimensionVector",
    /// `is coherent derived unit of system`: 
    derivedCoherentUnitOfSystem, "derivedCoherentUnitOfSystem",
    /// `is non-coherent derived unit of system`: 
    derivedNonCoherentUnitOfSystem, "derivedNonCoherentUnitOfSystem",
    /// `derived quantity kind of system`: 
    derivedQuantityKindOfSystem, "derivedQuantityKindOfSystem",
    /// `is derived unit of system`: 
    derivedUnitOfSystem, "derivedUnitOfSystem",
    /// `description`: 
    description, "description",
    /// `descriptor`: 
    descriptor, "descriptor",
    /// `dimension`: 
    dimension, "dimension",
    /// `dimension exponent`: 
    dimensionExponent, "dimensionExponent",
    /// `dimension exponent for amount of substance`: 
    dimensionExponentForAmountOfSubstance, "dimensionExponentForAmountOfSubstance",
    /// `dimension exponent for electric current`: 
    dimensionExponentForElectricCurrent, "dimensionExponentForElectricCurrent",
    /// `dimension exponent for length`: 
    dimensionExponentForLength, "dimensionExponentForLength",
    /// `dimension exponent for luminous intensity`: 
    dimensionExponentForLuminousIntensity, "dimensionExponentForLuminousIntensity",
    /// `dimension exponent for mass`: 
    dimensionExponentForMass, "dimensionExponentForMass",
    /// `dimension exponent for thermodynamic temperature`: 
    dimensionExponentForThermodynamicTemperature, "dimensionExponentForThermodynamicTemperature",
    /// `dimension exponent for time`: 
    dimensionExponentForTime, "dimensionExponentForTime",
    /// `dimension inverse`: 
    dimensionInverse, "dimensionInverse",
    /// `dimension vector`: 
    dimensionVector, "dimensionVector",
    /// `dimension vector for SI`: 
    dimensionVectorForSI, "dimensionVectorForSI",
    /// `dimensionality`: 
    dimensionality, "dimensionality",
    /// `dimensionless exponent`: 
    dimensionlessExponent, "dimensionlessExponent",
    /// `duration`: 
    duration, "duration",
    /// `element`: 
    element, "element",
    /// `element kind`: 
    elementKind, "elementKind",
    /// `element label`: 
    elementLabel, "elementLabel",
    /// `element name`: 
    elementName, "elementName",
    /// `element type`: This property is used to relate a structured data type with the data type of the structured type's elements. It is used for homogeneous structured data types, that is, those whose elements that are all of the same type.
    elementType, "elementType",
    /// `element type count`: 
    elementTypeCount, "elementTypeCount",
    /// `element type list`: 
    elementTypeList, "elementTypeList",
    /// `element unit`: 
    elementUnit, "elementUnit",
    /// `encoded value`: 
    encodedValue, "encodedValue",
    /// `encoding`: 
    encoding, "encoding",
    /// `encoding description`: 
    encodingDescription, "encodingDescription",
    /// `enumeration`: 
    enumeration, "enumeration",
    /// `exact constant`: 
    exactConstant, "exactConstant",
    /// `exact match`: 
    exactMatch, "exactMatch",
    /// `example`: The 'qudt:example' property is used to annotate an instance of a class with a reference to a concept that is an example. The type of this property is 'rdf:Property'. This allows both scalar and object ranges.
    example, "example",
    /// `exponent`: 
    exponent, "exponent",
    /// `expression`: 
    expression, "expression",
    /// `field`: 
    field, "field",
    /// `field code`: 
    fieldCode, "fieldCode",
    /// `fieldLabel`: 
    fieldLabel, "fieldLabel",
    /// `field labels`: 
    fieldLabels, "fieldLabels",
    /// `field name`: 
    fieldName, "fieldName",
    /// `fieldType`: 
    fieldType, "fieldType",
    /// `figure`: 
    figure, "figure",
    /// `figure caption`: 
    figureCaption, "figureCaption",
    /// `figure label`: 
    figureLabel, "figureLabel",
    /// `float percentage`: 
    floatPercentage, "floatPercentage",
    /// `float X`: 
    float_X, "float_X",
    /// `float Y`: 
    float_Y, "float_Y",
    /// `float Z`: 
    float_Z, "float_Z",
    /// `frame type`: 
    frameType, "frameType",
    /// `function`: 
    function, "function",
    /// `function arity`: 
    functionArity, "functionArity",
    /// `generalization`: 
    generalization, "generalization",
    /// `guidance`: 
    guidance, "guidance",
    /// `allowed unit`: 
    hasAllowedUnit, "hasAllowedUnit",
    /// `has base quantity kind`: 
    hasBaseQuantityKind, "hasBaseQuantityKind",
    /// `base unit`: 
    hasBaseUnit, "hasBaseUnit",
    /// `coherent unit`: 
    hasCoherentUnit, "hasCoherentUnit",
    /// `defined unit`: 
    hasDefinedUnit, "hasDefinedUnit",
    /// `has quantity kind dimension vector denominator part`: 
    hasDenominatorPart, "hasDenominatorPart",
    /// `derived coherent unit`: 
    hasDerivedCoherentUnit, "hasDerivedCoherentUnit",
    /// `has coherent derived unit`: 
    hasDerivedNonCoherentUnit, "hasDerivedNonCoherentUnit",
    /// `derived unit`: 
    hasDerivedUnit, "hasDerivedUnit",
    /// `has dimension`: 
    hasDimension, "hasDimension",
    /// `dimension expression`: 
    hasDimensionExpression, "hasDimensionExpression",
    /// `has dimension vector`: 
    hasDimensionVector, "hasDimensionVector",
    /// `has non-coherent unit`: 
    hasNonCoherentUnit, "hasNonCoherentUnit",
    /// `has quantity kind dimension vector numerator part`: 
    hasNumeratorPart, "hasNumeratorPart",
    /// `prefix unit`: 
    hasPrefixUnit, "hasPrefixUnit",
    /// `has quantity`: 
    hasQuantity, "hasQuantity",
    /// `has quantity kind`: 
    hasQuantityKind, "hasQuantityKind",
    /// `has reference quantity kind`: 
    hasReferenceQuantityKind, "hasReferenceQuantityKind",
    /// `has rule`: 
    hasRule, "hasRule",
    /// `has unit`: 
    hasUnit, "hasUnit",
    /// `has unit system`: 
    hasUnitSystem, "hasUnitSystem",
    /// `has vocabulary`: 
    hasVocabulary, "hasVocabulary",
    /// `height`: 
    height, "height",
    /// `hexbinary`: 
    hexbinary, "hexbinary",
    /// `iconic cue`: 
    iconicCue, "iconicCue",
    /// `iconic cue enumeration`: 
    iconicCueEnumeration, "iconicCueEnumeration",
    /// `id`: 
    id, "id",
    /// `iec-61360 code`: 
    iec61360Code, "iec61360Code",
    /// `image`: 
    image, "image",
    /// `image location`: 
    imageLocation, "imageLocation",
    /// `incrementDatatype`: 
    incrementDatatype, "incrementDatatype",
    /// `index`: 
    index, "index",
    /// `informative reference`: 
    informativeReference, "informativeReference",
    /// `integer 1..12`: 
    integer1to12, "integer1to12",
    /// `integer 1..31`: 
    integer1to31, "integer1to31",
    /// `integer percentage`: 
    integerPercentage, "integerPercentage",
    /// `inverted`: 
    inverted, "inverted",
    /// `is base quantity kind of system`: 
    isBaseQuantityKindOfSystem, "isBaseQuantityKindOfSystem",
    /// `is byte string`: 
    isByteString, "isByteString",
    /// `isComprisedOf`: 
    isComprisedOf, "isComprisedOf",
    /// `is Delta Quantity`: This property is used to identify a Quantity instance that is a measure of a change, or interval, of some property, rather than a measure of its absolute value. This is important for measurements such as temperature differences where the conversion among units would be calculated differently because of offsets.
    isDeltaQuantity, "isDeltaQuantity",
    /// `is dimension in system`: 
    isDimensionInSystem, "isDimensionInSystem",
    /// `is metric unit`: 
    isMetricUnit, "isMetricUnit",
    /// `is quantity kind of`: 
    isQuantityKindOf, "isQuantityKindOf",
    /// `is scaling of`: 
    isScalingOf, "isScalingOf",
    /// `normative reference (ISO)`: 
    isoNormativeReference, "isoNormativeReference",
    /// `java name`: 
    javaName, "javaName",
    /// `Javascript name`: 
    jsName, "jsName",
    /// `kinesthetic cue`: 
    kinestheticCue, "kinestheticCue",
    /// `kinesthetic cue enumeration`: 
    kinestheticCueEnumeration, "kinestheticCueEnumeration",
    /// `landscape`: 
    landscape, "landscape",
    /// `latex definition`: 
    latexDefinition, "latexDefinition",
    /// `latex symbol`: 
    latexSymbol, "latexSymbol",
    /// `length`: 
    length, "length",
    /// `literal`: 
    literal, "literal",
    /// `lower bound`: 
    lowerBound, "lowerBound",
    /// `mantissa`: In scientific notation, the mantissa of a real number is the integer coefficient preceding the base raised to the exponent.
    mantissa, "mantissa",
    /// `math definition`: 
    mathDefinition, "mathDefinition",
    /// `mathML definition`: 
    mathMLdefinition, "mathMLdefinition",
    /// `matlab name`: 
    matlabName, "matlabName",
    /// `maximum bits`: 
    maxBits, "maxBits",
    /// `max depth`: 
    maxDepth, "maxDepth",
    /// `max exclusive`: 
    maxExclusive, "maxExclusive",
    /// `max exponent`: 
    maxExponent, "maxExponent",
    /// `max inclusive`: 
    maxInclusive, "maxInclusive",
    /// `max length`: 
    maxLength, "maxLength",
    /// `max mantissa`: 
    maxMantissa, "maxMantissa",
    /// `Microsoft SQL Server name`: 
    microsoftSQLServerName, "microsoftSQLServerName",
    /// `minimum bits`: 
    minBits, "minBits",
    /// `min exclusive`: 
    minExclusive, "minExclusive",
    /// `min inclusive`: 
    minInclusive, "minInclusive",
    /// `min length`: 
    minLength, "minLength",
    /// `min mantissa`: 
    minMantissa, "minMantissa",
    /// `minimum value`: 
    minValue, "minValue",
    /// `modal cue`: 
    modalCue, "modalCue",
    /// `modal cue enumeration`: 
    modalCueEnumeration, "modalCueEnumeration",
    /// `modifiability`: Reference to one in a list of enumerated elements that indicates whether data (e.g. variable or parameter) can be changed.
    modifiability, "modifiability",
    /// `MySQL name`: 
    mySQLName, "mySQLName",
    /// `negative`: 
    negative, "negative",
    /// `negative delta limit`: 
    negativeDeltaLimit, "negativeDeltaLimit",
    /// `next`: 
    next, "next",
    /// `node`: 
    node, "node",
    /// `normative reference`: 
    normativeReference, "normativeReference",
    /// `numerator dimension vector`: 
    numeratorDimensionVector, "numeratorDimensionVector",
    /// `numeric value`: 
    numericValue, "numericValue",
    /// `objectValue`: 
    objectValue, "objectValue",
    /// `octets`: 
    octets, "octets",
    /// `ODBC name`: 
    odbcName, "odbcName",
    /// `OLE DB name`: 
    oleDBName, "oleDBName",
    /// `om unit`: 
    omUnit, "omUnit",
    /// `online reference`: 
    onlineReference, "onlineReference",
    /// `optional`: 
    optional, "optional",
    /// `ORACLE SQL name`: 
    oracleSQLName, "oracleSQLName",
    /// `order`: 
    order, "order",
    /// `order in structure`: 
    orderInStructure, "orderInStructure",
    /// `ordered type`: 
    orderedType, "orderedType",
    /// `Ordering Relation`: 
    orderingRelation, "orderingRelation",
    /// `origin definition`: 
    originDefinition, "originDefinition",
    /// `out of scope`: 
    outOfScope, "outOfScope",
    /// `padding`: 
    padding, "padding",
    /// `pattern`: 
    pattern, "pattern",
    /// `permissible maths`: 
    permissibleMaths, "permissibleMaths",
    /// `permissible transformation`: 
    permissibleTransformation, "permissibleTransformation",
    /// `pitch rotation definition`: 
    pitchRotationDefinition, "pitchRotationDefinition",
    /// `description (plain text)`: 
    plainTextDescription, "plainTextDescription",
    /// `positive`: 
    positive, "positive",
    /// `Positive delta limit`: 
    positiveDeltaLimit, "positiveDeltaLimit",
    /// `precision`: 
    precision, "precision",
    /// `prefix`: Associates a unit with the appropriate prefix, if any.
    prefix, "prefix",
    /// `prefix multiplier`: 
    prefixMultiplier, "prefixMultiplier",
    /// `previous`: 
    previous, "previous",
    /// `protocol buffers name`: 
    protocolBuffersName, "protocolBuffersName",
    /// `python name`: 
    pythonName, "pythonName",
    /// `denominator dimension vector`: 
    qkdvDenominator, "qkdvDenominator",
    /// `numerator dimension vector`: 
    qkdvNumerator, "qkdvNumerator",
    /// `quantity`: 
    quantity, "quantity",
    /// `quantity value`: 
    quantityValue, "quantityValue",
    /// `rationale`: 
    rationale, "rationale",
    /// `rdfs datatype`: 
    rdfsDatatype, "rdfsDatatype",
    /// `realization`: 
    realization, "realization",
    /// `reference`: 
    reference, "reference",
    /// `reference frame`: 
    referenceFrame, "referenceFrame",
    /// `reference frame type`: 
    referenceFrameType, "referenceFrameType",
    /// `reference unit`: 
    referenceUnit, "referenceUnit",
    /// `relative standard uncertainty`: 
    relativeStandardUncertainty, "relativeStandardUncertainty",
    /// `relevant quantity kind`: 
    relevantQuantityKind, "relevantQuantityKind",
    /// `Relevant Unit`: This property is used for qudt:Discipline instances to identify the Unit instances that are used within a given discipline.
    relevantUnit, "relevantUnit",
    /// `return type`: 
    returnType, "returnType",
    /// `rgb code`: 
    rgbCode, "rgbCode",
    /// `roll rotation definition`: 
    rollRotationDefinition, "rollRotationDefinition",
    /// `rotation definition`: 
    rotationDefinition, "rotationDefinition",
    /// `rows`: 
    rows, "rows",
    /// `rule type`: 
    ruleType, "ruleType",
    /// `scalarValue`: 
    scalarValue, "scalarValue",
    /// `scale type`: 
    scaleType, "scaleType",
    /// `si units expression`: 
    siUnitsExpression, "siUnitsExpression",
    /// `signedness`: 
    signedness, "signedness",
    /// `significant digits`: 
    significantDigits, "significantDigits",
    /// `sound`: The intended use of the sound property is to be associated with modal enumerations
    sound, "sound",
    /// `specialization`: 
    specialization, "specialization",
    /// `standard uncertainty`: 
    standardUncertainty, "standardUncertainty",
    /// `string value`: 
    stringValue, "stringValue",
    /// `symbol`: 
    symbol, "symbol",
    /// `system definition`: 
    systemDefinition, "systemDefinition",
    /// `system derived quantity kind`: 
    systemDerivedQuantityKind, "systemDerivedQuantityKind",
    /// `system dimension`: 
    systemDimension, "systemDimension",
    /// `time datatype`: 
    timeDatatype, "timeDatatype",
    /// `total digits`: 
    totalDigits, "totalDigits",
    /// `type`: A reference to the specification of the data type of a variable or constant.
    type_, "type",
    /// `type matrix`: 
    typeMatrix, "typeMatrix",
    /// `type vector`: 
    typeVector, "typeVector",
    /// `ucum case-insensitive code`: 
    ucumCaseInsensitiveCode, "ucumCaseInsensitiveCode",
    /// `ucum case-sensitive code`: 
    ucumCaseSensitiveCode, "ucumCaseSensitiveCode",
    /// `ucum code`: 
    ucumCode, "ucumCode",
    /// `unece common code`: 
    uneceCommonCode, "uneceCommonCode",
    /// `unit`: 
    unit, "unit",
    /// `unit for`: 
    unitFor, "unitFor",
    /// `is unit of system`: 
    unitOfSystem, "unitOfSystem",
    /// `upper bound`: 
    upperBound, "upperBound",
    /// `url`: 
    url, "url",
    /// `value`: This property identifies the value associated with a data structure. The value may be a scalar or a reference to another object. The disjoint sub-properties of this data:value distinguish between scalar values and object references.
    value, "value",
    /// `value for quantity`: 
    valueQuantity, "valueQuantity",
    /// `value range`: 
    valueRange, "valueRange",
    /// `value type`: 
    valueType, "valueType",
    /// `value union`: 
    valueUnion, "valueUnion",
    /// `value vector`: A list of the values of elements in a Partial Array.
    valueVector, "valueVector",
    /// `Visual Basic name`: 
    vbName, "vbName",
    /// `vector`: 
    vector, "vector",
    /// `vector magnitude`: 
    vectorMagnitude, "vectorMagnitude",
    /// `visual cue`: 
    visualCue, "visualCue",
    /// `visual cue enumeration`: 
    visualCueEnumeration, "visualCueEnumeration",
    /// `width`: 
    width, "width",
    /// `X-Axis Definition`: 
    xAxisDefinition, "xAxisDefinition",
    /// `X-Coordinate definition`: 
    xCoordinateDefinition, "xCoordinateDefinition",
    /// `Y-Axis definition`: 
    yAxisDefinition, "yAxisDefinition",
    /// `Y-Coordinate definition`: 
    yCoordinateDefinition, "yCoordinateDefinition",
    /// `Yaw rotation definition`: 
    yawRotationDefinition, "yawRotationDefinition",
    /// `Z-Axis definition`: 
    zAxisDefinition, "zAxisDefinition",
    /// `Z-Coordinate definition`: 
    zCoordinateDefinition, "zCoordinateDefinition"
);
