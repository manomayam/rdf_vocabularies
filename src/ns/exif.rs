// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-exif")))]
//! This module provides terms for `Exif data description vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Exif data description vocabulary|
//! |**Prefix**|exif|
//! |**Namespace base IRI**|<http://www.w3.org/2003/12/exif/ns#>|
//! |**Description**|Vocabulary to describe an Exif format picture data. All Exif 2.2 tags are defined as RDF properties, as well as several terms to help this schema.|
//! |**Is defined by**|<http://www.w3.org/2003/12/exif/ns#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2003/12/exif/ns#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `IFD`: An Image File Directory
    IFD, "IFD",
    /// `Unknown tag`: An Exif tag whose meaning is not known
    _unknown, "_unknown",
    /// `ApertureValue`: The lens aperture. The unit is the APEX value.
    apertureValue, "apertureValue",
    /// `Artist`: Person who created the image
    artist, "artist",
    /// `BitsPerSample`: The number of bits per image component. In this standard each component of the image is 8 bits, so the value for this tag is 8. See also SamplesPerPixel. In JPEG compressed data a JPEG marker is used instead of this tag.
    bitsPerSample, "bitsPerSample",
    /// `BrightnessValue`: The value of brightness. The unit is the APEX value. Ordinarily it is given in the range of -99.99 to 99.99. Note that if the numerator of the recorded value is FFFFFFFF.H, Unknown shall be indicated.
    brightnessValue, "brightnessValue",
    /// `CFAPattern`: The color filter array (CFA) geometric pattern of the image sensor when a one-chip color area sensor is used. It does not apply to all sensing methods.
    cfaPattern, "cfaPattern",
    /// `ColorSpace`: The color space information tag (ColorSpace) is always recorded as the color space specifier. Normally sRGB (=1) is used to define the color space based on the PC monitor conditions and environment.
    colorSpace, "colorSpace",
    /// `ComponentsConfiguration`: Information specific to compressed data. The channels of each component are arranged in order from the 1st component to the 4th. For uncompressed data the data arrangement is given in the PhotometricInterpretation tag. However, since PhotometricInterpretation can only express the order of Y,Cb and Cr, this tag is provided for cases when compressed data uses components other than Y, Cb, and Cr and to enable support of other sequences.
    componentsConfiguration, "componentsConfiguration",
    /// `CompressedBitsPerPixel`: Information specific to compressed data. The compression mode used for a compressed image is indicated in unit bits per pixel.
    compressedBitsPerPixel, "compressedBitsPerPixel",
    /// `Compression`: The compression scheme used for the image data. When a primary image is JPEG compressed, this designation is not necessary and is omitted. When thumbnails use JPEG compression, this tag value is set to 6.
    compression, "compression",
    /// `Contrast`: The direction of contrast processing applied by the camera when the image was shot.
    contrast, "contrast",
    /// `Copyright`: Copyright information. In this standard the tag is used to indicate both the photographer and editor copyrights. It is the copyright notice of the person or organization claiming rights to the image.
    copyright, "copyright",
    /// `CustomRendered`: The use of special processing on image data, such as rendering geared to output. When special processing is performed, the reader is expected to disable or minimize any further processing.
    customRendered, "customRendered",
    /// `Data Type`: The Exif field data type, such as ascii, byte, short etc.
    datatype, "datatype",
    /// `Date`: a date information. Usually saved as YYYY:MM:DD (HH:MM:SS) format in Exif data, but represented here as W3C-DTF format
    date, "date",
    /// `Date and/or Time`: An attribute relating to Date and/or Time
    dateAndOrTime, "dateAndOrTime",
    /// `DateTime`: The date and time of image creation. In this standard it is the date and time the file was changed.
    dateTime, "dateTime",
    /// `DateTimeDigitized`: The date and time when the image was stored as digital data. If, for example, an image was captured by DSC and at the same time the file was recorded, then the DateTimeOriginal and DateTimeDigitized will have the same contents.
    dateTimeDigitized, "dateTimeDigitized",
    /// `DateTimeOriginal`: The date and time when the original image data was generated. For a DSC the date and time the picture was taken are recorded.
    dateTimeOriginal, "dateTimeOriginal",
    /// `DeviceSettingDescription`: Information on the picture-taking conditions of a particular camera model. The tag is used only to indicate the picture-taking conditions in the reader.
    deviceSettingDescription, "deviceSettingDescription",
    /// `DigitalZoomRatio`: The digital zoom ratio when the image was shot. If the numerator of the recorded value is 0, this indicates that digital zoom was not used.
    digitalZoomRatio, "digitalZoomRatio",
    /// `Exif Attribute`: A property that connects an IFD to one of its entries. Super property which integrates all Exif tags.
    exifAttribute, "exifAttribute",
    /// `ExifVersion`: Exif Version
    exifVersion, "exifVersion",
    /// `Exif IFD Pointer`: A pointer to the Exif IFD, which is a set of tags for recording Exif-specific attribute information.
    exif_IFD_Pointer, "exif_IFD_Pointer",
    /// `Exif data`: An Exif IFD data entry
    exifdata, "exifdata",
    /// `ExposureBiasValue`: The exposure bias. The unit is the APEX value. Ordinarily it is given in the range of -99.99 to 99.99.
    exposureBiasValue, "exposureBiasValue",
    /// `ExposureIndex`: The exposure index selected on the camera or input device at the time the image is captured.
    exposureIndex, "exposureIndex",
    /// `ExposureMode`: the exposure mode set when the image was shot. In auto-bracketing mode, the camera shoots a series of frames of the same scene at different exposure settings.
    exposureMode, "exposureMode",
    /// `ExposureProgram`: The class of the program used by the camera to set exposure when the picture is taken.
    exposureProgram, "exposureProgram",
    /// `ExposureTime`: Exposure time, given in seconds (sec).
    exposureTime, "exposureTime",
    /// `FNumber`: F number
    fNumber, "fNumber",
    /// `FileSource`: The image source. If a DSC recorded the image, this tag value of this tag always be set to 3, indicating that the image was recorded on a DSC.
    fileSource, "fileSource",
    /// `Flash`: The status of flash when the image was shot.
    flash, "flash",
    /// `FlashEnergy`: The strobe energy at the time the image is captured, as measured in Beam Candle Power Seconds (BCPS).
    flashEnergy, "flashEnergy",
    /// `FlashpixVersion`: The Flashpix format version supported by a FPXR file. If the FPXR function supports Flashpix format Ver. 1.0, this is indicated similarly to ExifVersion by recording "0100" as 4-byte ASCII.
    flashpixVersion, "flashpixVersion",
    /// `FocalLength`: The actual focal length of the lens, in mm. Conversion is not made to the focal length of a 35 mm film camera.
    focalLength, "focalLength",
    /// `FocalLengthIn35mmFilm`: The equivalent focal length assuming a 35mm film camera, in mm. A value of 0 means the focal length is unknown. Note that this tag differs from the FocalLength tag.
    focalLengthIn35mmFilm, "focalLengthIn35mmFilm",
    /// `FocalPlaneResolutionUnit`: The unit for measuring FocalPlaneXResolution and FocalPlaneYResolution. This value is the same as the ResolutionUnit.
    focalPlaneResolutionUnit, "focalPlaneResolutionUnit",
    /// `FocalPlaneXResolution`: The number of pixels in the image width (X) direction per FocalPlaneResolutionUnit on the camera focal plane.
    focalPlaneXResolution, "focalPlaneXResolution",
    /// `FocalPlaneYResolution`: The number of pixels in the image height (Y) direction per FocalPlaneResolutionUnit on the camera focal plane.
    focalPlaneYResolution, "focalPlaneYResolution",
    /// `GainControl`: The degree of overall image gain adjustment.
    gainControl, "gainControl",
    /// `Geometric data`: Geometric data such as latitude, longitude and altitude. Usually saved as rational number.
    geo, "geo",
    /// `GPSAltitude`: The altitude based on the reference in GPSAltitudeRef. Altitude is expressed as one RATIONAL value. The reference unit is meters.
    gpsAltitude, "gpsAltitude",
    /// `GPSAltitudeRef`: Indicates the altitude used as the reference altitude. If the reference is sea level and the altitude is above sea level, 0 is given. If the altitude is below sea level, a value of 1 is given and the altitude is indicated as an absolute value in the GPSAltitude tag. The reference unit is meters.
    gpsAltitudeRef, "gpsAltitudeRef",
    /// `GPSAreaInformation`: A character string recording the name of the GPS area. The first byte indicates the character code used, and this is followed by the name of the GPS area.
    gpsAreaInformation, "gpsAreaInformation",
    /// `GPSDOP`: The GPS DOP (data degree of precision). An HDOP value is written during two-dimensional measurement, and PDOP during three-dimensional measurement.
    gpsDOP, "gpsDOP",
    /// `GPSDateStamp`: date and time information relative to UTC (Coordinated Universal Time). The record format is "YYYY:MM:DD" while converted to W3C-DTF to use in RDF
    gpsDateStamp, "gpsDateStamp",
    /// `GPSDestBearing`: The bearing to the destination point. The range of values is from 0.00 to 359.99.
    gpsDestBearing, "gpsDestBearing",
    /// `GPSDestBearingRef`: Indicates the reference used for giving the bearing to the destination point. 'T' denotes true direction and 'M' is magnetic direction.
    gpsDestBearingRef, "gpsDestBearingRef",
    /// `GPSDestDistance`: The distance to the destination point.
    gpsDestDistance, "gpsDestDistance",
    /// `GPSDestDistanceRef`: Indicates the unit used to express the distance to the destination point. 'K', 'M' and 'N' represent kilometers, miles and knots.
    gpsDestDistanceRef, "gpsDestDistanceRef",
    /// `GPSDestLatitude`: Latitude of destination, expressed as three values giving the degrees, minutes, and seconds, respectively.
    gpsDestLatitude, "gpsDestLatitude",
    /// `GPSDestLatitudeRef`: Reference for latitude of destination
    gpsDestLatitudeRef, "gpsDestLatitudeRef",
    /// `GPSDestLongitude`: Longitude of destination, expressed as three values giving the degrees, minutes, and seconds, respectively.
    gpsDestLongitude, "gpsDestLongitude",
    /// `GPSDestLongitudeRef`: Reference for longitude of destination
    gpsDestLongitudeRef, "gpsDestLongitudeRef",
    /// `GPSDifferential`: Indicates whether differential correction is applied to the GPS receiver.
    gpsDifferential, "gpsDifferential",
    /// `GPSImgDirection`: The direction of the image when it was captured. The range of values is from 0.00 to 359.99.
    gpsImgDirection, "gpsImgDirection",
    /// `GPSImgDirectionRef`: The reference for giving the direction of the image when it is captured. 'T' denotes true direction and 'M' is magnetic direction.
    gpsImgDirectionRef, "gpsImgDirectionRef",
    /// `GPS Info`: An attribute relating to GPS information
    gpsInfo, "gpsInfo",
    /// `GPSInfo IFD Pointer`: A pointer to the GPS IFD, which is a set of tags for recording GPS information.
    gpsInfo_IFD_Pointer, "gpsInfo_IFD_Pointer",
    /// `GPSLatitude`: The latitude, expressed as three values giving the degrees, minutes, and seconds, respectively.
    gpsLatitude, "gpsLatitude",
    /// `GPSLatitudeRef`: Indicates whether the latitude is north or south latitude. The ASCII value 'N' indicates north latitude, and 'S' is south latitude.
    gpsLatitudeRef, "gpsLatitudeRef",
    /// `GPSLongitude`: The longitude, expressed as three values giving the degrees, minutes, and seconds, respectively.
    gpsLongitude, "gpsLongitude",
    /// `GPSLongitudeRef`: Indicates whether the longitude is east or west longitude. ASCII 'E' indicates east longitude, and 'W' is west longitude.
    gpsLongitudeRef, "gpsLongitudeRef",
    /// `GPSMapDatum`: The geodetic survey data used by the GPS receiver. If the survey data is restricted to Japan, the value of this tag is 'TOKYO' or 'WGS-84'. If a GPS Info tag is recorded, it is strongly recommended that this tag be recorded.
    gpsMapDatum, "gpsMapDatum",
    /// `GPSMeasureMode`: The GPS measurement mode. '2' means two-dimensional measurement and '3' means three-dimensional measurement is in progress.
    gpsMeasureMode, "gpsMeasureMode",
    /// `GPSProcessingMethod`: A character string recording the name of the method used for location finding. The first byte indicates the character code used, and this is followed by the name of the method.
    gpsProcessingMethod, "gpsProcessingMethod",
    /// `GPSSatellites`: The GPS satellites used for measurements. This tag can be used to describe the number of satellites, their ID number, angle of elevation, azimuth, SNR and other information in ASCII notation. The format is not specified. If the GPS receiver is incapable of taking measurements, value of the tag shall be set to NULL.
    gpsSatellites, "gpsSatellites",
    /// `GPSSpeed`: The speed of GPS receiver movement.
    gpsSpeed, "gpsSpeed",
    /// `GPSSpeedRef`: The unit used to express the GPS receiver speed of movement. 'K' 'M' and 'N' represents kilometers per hour, miles per hour, and knots.
    gpsSpeedRef, "gpsSpeedRef",
    /// `GPSStatus`: The status of the GPS receiver when the image is recorded. 'A' means measurement is in progress, and 'V' means the measurement is Interoperability.
    gpsStatus, "gpsStatus",
    /// `GPSTimeStamp`: The time as UTC (Coordinated Universal Time). TimeStamp is expressed as three RATIONAL values giving the hour, minute, and second.
    gpsTimeStamp, "gpsTimeStamp",
    /// `GPSTrack`: The direction of GPS receiver movement. The range of values is from 0.00 to 359.99.
    gpsTrack, "gpsTrack",
    /// `GPSTrackRef`: The reference for giving the direction of GPS receiver movement. 'T' denotes true direction and 'M' is magnetic direction.
    gpsTrackRef, "gpsTrackRef",
    /// `GPSVersionID`: The version of GPSInfoIFD. The version is given as 2.2.0.0. This tag is mandatory when GPSInfo tag is present.
    gpsVersionID, "gpsVersionID",
    /// `Height`: Height of an object
    height, "height",
    /// `IFD Pointer`: A tag that refers a child IFD
    ifdPointer, "ifdPointer",
    /// `Image Config`: An attribute relating to Image Configuration
    imageConfig, "imageConfig",
    /// `Image Data Character`: An attribute relating to image data characteristics
    imageDataCharacter, "imageDataCharacter",
    /// `Image Data Structure`: An attribute relating to image data structure
    imageDataStruct, "imageDataStruct",
    /// `ImageDescription`: A character string giving the title of the image. It may be a comment such as "1988 company picnic" or the like. Two-byte character codes cannot be used. When a 2-byte code is necessary, the Exif Private tag UserComment is to be used.
    imageDescription, "imageDescription",
    /// `ImageLength`: Image height. The number of rows of image data. In JPEG compressed data a JPEG marker is used.
    imageLength, "imageLength",
    /// `ImageUniqueID`: An identifier assigned uniquely to each image. It is recorded as an ASCII string equivalent to hexadecimal notation and 128-bit fixed length.
    imageUniqueID, "imageUniqueID",
    /// `ImageWidth`: Image width. The number of columns of image data, equal to the number of pixels per row. In JPEG compressed data a JPEG marker is used instead of this tag.
    imageWidth, "imageWidth",
    /// `Interoperability Info`: An attribute relating to Interoperability. Tags stored in Interoperability IFD may be defined dependently to each Interoperability rule.
    interopInfo, "interopInfo",
    /// `InteroperabilityIndex`: Indicates the identification of the Interoperability rule. 'R98' = conforming to R98 file specification of Recommended Exif Interoperability Rules (ExifR98) or to DCF basic file stipulated by Design Rule for Camera File System. 'THM' = conforming to DCF thumbnail file stipulated by Design rule for Camera File System.
    interoperabilityIndex, "interoperabilityIndex",
    /// `InteroperabilityVersion`: Interoperability Version
    interoperabilityVersion, "interoperabilityVersion",
    /// `Interoperability IFD Pointer`: A pointer to the Interoperability IFD, which is composed of tags storing the information to ensure the Interoperability
    interoperability_IFD_Pointer, "interoperability_IFD_Pointer",
    /// `ISOSpeedRatings`: Indicates the ISO Speed and ISO Latitude of the camera or input device as specified in ISO 12232.
    isoSpeedRatings, "isoSpeedRatings",
    /// `JPEGInterchangeFormat`: The offset to the start byte (SOI) of JPEG compressed thumbnail data. This is not used for primary image JPEG data.
    jpegInterchangeFormat, "jpegInterchangeFormat",
    /// `JPEGInterchangeFormatLength`: The number of bytes of JPEG compressed thumbnail data. This is not used for primary image JPEG data.
    jpegInterchangeFormatLength, "jpegInterchangeFormatLength",
    /// `Length`: Length of an object. Could be a subProperty of other general schema.
    length, "length",
    /// `LightSource`: Light source such as Daylight, Tungsten, Flash etc.
    lightSource, "lightSource",
    /// `Make`: Manufacturer of image input equipment
    make, "make",
    /// `MakerNote`: Manufacturer notes
    makerNote, "makerNote",
    /// `MaxApertureValue`: The smallest F number of the lens. The unit is the APEX value. Ordinarily it is given in the range of 00.00 to 99.99, but it is not limited to this range.
    maxApertureValue, "maxApertureValue",
    /// `Meter`: A length with unit of meter
    meter, "meter",
    /// `MeteringMode`: Metering mode, such as CenterWeightedAverage, Spot, MultiSpot,Pattern, Partial etc.
    meteringMode, "meteringMode",
    /// `Milimeter`: A length with unit of mm
    mm, "mm",
    /// `Model`: Model of image input equipment
    model, "model",
    /// `OECF`: Indicates the Opto-Electric Conversion Function (OECF) specified in ISO 14524. OECF is the relationship between the camera optical input and the image values.
    oecf, "oecf",
    /// `Orientation`: The image orientation viewed in terms of rows and columns.
    orientation, "orientation",
    /// `PhotometricInterpretation`: Pixel composition. In JPEG compressed data a JPEG marker is used instead of this tag.
    photometricInterpretation, "photometricInterpretation",
    /// `PictTaking`: An attribute relating to Picture-Taking Conditions
    pictTaking, "pictTaking",
    /// `PrintIM Brightness`: Brightness info for print image matching
    pimBrightness, "pimBrightness",
    /// `PrintIM ColorBalance`: ColorBalance info for print image matching
    pimColorBalance, "pimColorBalance",
    /// `PrintIM Contrast`: Contrast info for print image matching
    pimContrast, "pimContrast",
    /// `PIM Info`: An attribute relating to print image matching
    pimInfo, "pimInfo",
    /// `PrintIM Saturation`: Saturation info for print image matching
    pimSaturation, "pimSaturation",
    /// `PrintIM Sharpness`: Sharpness info for print image matching
    pimSharpness, "pimSharpness",
    /// `PixelXDimension`: Information specific to compressed data. When a compressed file is recorded, the valid width of the meaningful image shall be recorded in this tag, whether or not there is padding data or a restart marker. This tag should not exist in an uncompressed file.
    pixelXDimension, "pixelXDimension",
    /// `PixelYDimension`: Information specific to compressed data. When a compressed file is recorded, the valid height of the meaningful image shall be recorded in this tag, whether or not there is padding data or a restart marker. This tag should not exist in an uncompressed file. Since data padding is unnecessary in the vertical direction, the number of lines recorded in this valid image height tag will in fact be the same as that recorded in the SOF.
    pixelYDimension, "pixelYDimension",
    /// `PlanarConfiguration`: Indicates whether pixel components are recorded in chunky or planar format. In JPEG compressed files a JPEG marker is used instead of this tag. If this field does not exist, the TIFF default of 1 (chunky) is assumed.
    planarConfiguration, "planarConfiguration",
    /// `PrimaryChromaticities`: The chromaticity of the three primary colors of the image. Normally this tag is not necessary, since color space is specified in the color space information tag (ColorSpace).
    primaryChromaticities, "primaryChromaticities",
    /// `PrintImageMatching IFD Pointer`: A pointer to the print image matching IFD
    printImageMatching_IFD_Pointer, "printImageMatching_IFD_Pointer",
    /// `Recording Offset`: An attribute relating to recording offset
    recOffset, "recOffset",
    /// `ReferenceBlackWhite`: The reference black point value and reference white point value. The color space is declared in a color space information tag, with the default being the value that gives the optimal image characteristics Interoperability these conditions.
    referenceBlackWhite, "referenceBlackWhite",
    /// `Related File`: Tag Relating to Related File Information
    relatedFile, "relatedFile",
    /// `RelatedImageFileFormat`: Related image file format
    relatedImageFileFormat, "relatedImageFileFormat",
    /// `RelatedImageLength`: Related image length
    relatedImageLength, "relatedImageLength",
    /// `RelatedImageWidth`: Related image width
    relatedImageWidth, "relatedImageWidth",
    /// `RelatedSoundFile`: Related audio file
    relatedSoundFile, "relatedSoundFile",
    /// `Resolution`: a rational number representing a resolution. Could be a subProperty of other general schema.
    resolution, "resolution",
    /// `ResolutionUnit`: The unit for measuring XResolution and YResolution. The same unit is used for both XResolution and YResolution. If the image resolution in unknown, 2 (inches) is designated.
    resolutionUnit, "resolutionUnit",
    /// `RowsPerStrip`: The number of rows per strip. This is the number of rows in the image of one strip when an image is divided into strips. With JPEG compressed data this designation is not needed and is omitted.
    rowsPerStrip, "rowsPerStrip",
    /// `SamplesPerPixel`: The number of components per pixel. Since this standard applies to RGB and YCbCr images, the value set for this tag is 3. In JPEG compressed data a JPEG marker is used instead of this tag.
    samplesPerPixel, "samplesPerPixel",
    /// `Saturation`: The direction of saturation processing applied by the camera when the image was shot.
    saturation, "saturation",
    /// `SceneCaptureType`: The type of scene that was shot. It can also be used to record the mode in which the image was shot, such as Landscape, Portrait etc. Note that this differs from the scene type (SceneType) tag.
    sceneCaptureType, "sceneCaptureType",
    /// `SceneType`: The type of scene. If a DSC recorded the image, this tag value shall always be set to 1, indicating that the image was directly photographed.
    sceneType, "sceneType",
    /// `Seconds`: a mesurement of time length with unit of second
    seconds, "seconds",
    /// `SensingMethod`: The image sensor type on the camera or input device, such as One-chip color area sensor etc.
    sensingMethod, "sensingMethod",
    /// `Sharpness`: The direction of sharpness processing applied by the camera when the image was shot.
    sharpness, "sharpness",
    /// `ShutterSpeedValue`: Shutter speed. The unit is the APEX (Additive System of Photographic Exposure) setting
    shutterSpeedValue, "shutterSpeedValue",
    /// `Software`: The name and version of the software or firmware of the camera or image input device used to generate the image.
    software, "software",
    /// `SpatialFrequencyResponse`: This tag records the camera or input device spatial frequency table and SFR values in the direction of image width, image height, and diagonal direction, as specified in ISO 12233.
    spatialFrequencyResponse, "spatialFrequencyResponse",
    /// `SpectralSensitivity`: Indicates the spectral sensitivity of each channel of the camera used. The tag value is an ASCII string compatible with the standard developed by the ASTM Technical committee.
    spectralSensitivity, "spectralSensitivity",
    /// `StripByteCounts`: The total number of bytes in each strip. With JPEG compressed data this designation is not needed and is omitted.
    stripByteCounts, "stripByteCounts",
    /// `StripOffsets`: For each strip, the byte offset of that strip. With JPEG compressed data this designation is not needed and is omitted.
    stripOffsets, "stripOffsets",
    /// `SubSecTime`: DateTime subseconds
    subSecTime, "subSecTime",
    /// `SubSecTimeDigitized`: DateTimeDigitized subseconds
    subSecTimeDigitized, "subSecTimeDigitized",
    /// `SubSecTimeOriginal`: DateTimeOriginal subseconds
    subSecTimeOriginal, "subSecTimeOriginal",
    /// `SubjectArea`: The location and area of the main subject in the overall scene.
    subjectArea, "subjectArea",
    /// `SubjectDistance`: The distance to the subject, given in meters. Note that if the numerator of the recorded value is FFFFFFFF.H, Infinity shall be indicated; and if the numerator is 0, Distance unknown shall be indicated.
    subjectDistance, "subjectDistance",
    /// `SubjectDistanceRange`: The distance to the subject, such as Macro, Close View or Distant View.
    subjectDistanceRange, "subjectDistanceRange",
    /// `SubjectLocation`: The location of the main subject in the scene. The value of this tag represents the pixel at the center of the main subject relative to the left edge, prior to rotation processing as per the Rotation tag. The first value indicates the X column number and second indicates the Y row number.
    subjectLocation, "subjectLocation",
    /// `subsecond`: 
    subsecond, "subsecond",
    /// `Subseconds`: A tag used to record fractions of seconds for a date property
    subseconds, "subseconds",
    /// `tagNumber`: 
    tagNumber, "tagNumber",
    /// `Tag number`: The Exif tag number
    tag_number, "tag_number",
    /// `Tag ID`: The Exif tag number with context prefix, such as IFD type or maker name
    tagid, "tagid",
    /// `TransferFunction`: A transfer function for the image, described in tabular style. Normally this tag is not necessary, since color space is specified in the color space information tag (ColorSpace).
    transferFunction, "transferFunction",
    /// `UserComment`: A tag for Exif users to write keywords or comments on the image besides those in ImageDescription, and without the character code limitations of the ImageDescription tag. The character code used in the UserComment tag is identified based on an ID code in a fixed 8-byte area at the start of the tag data area.
    userComment, "userComment",
    /// `User Info`: An attribute relating to User Information
    userInfo, "userInfo",
    /// `Version Info`: An attribute relating to Version
    versionInfo, "versionInfo",
    /// `WhiteBalance`: The white balance mode set when the image was shot.
    whiteBalance, "whiteBalance",
    /// `WhitePoint`: The chromaticity of the white point of the image. Normally this tag is not necessary, since color space is specified in the color space information tag (ColorSpace).
    whitePoint, "whitePoint",
    /// `Width`: Width of an object
    width, "width",
    /// `XResolution`: The number of pixels per ResolutionUnit in the ImageWidth direction. When the image resolution is unknown, 72 [dpi] is designated.
    xResolution, "xResolution",
    /// `YCbCrCoefficients`: The matrix coefficients for transformation from RGB to YCbCr image data.
    yCbCrCoefficients, "yCbCrCoefficients",
    /// `YCbCrPositioning`: The position of chrominance components in relation to the luminance component. This field is designated only for JPEG compressed data or uncompressed YCbCr data.
    yCbCrPositioning, "yCbCrPositioning",
    /// `YCbCrSubSampling`: The sampling ratio of chrominance components in relation to the luminance component. In JPEG compressed data a JPEG marker is used instead of this tag.
    yCbCrSubSampling, "yCbCrSubSampling",
    /// `YResolution`: The number of pixels per ResolutionUnit in the ImageLength direction. The same value as XResolution is designated.
    yResolution, "yResolution"
);
