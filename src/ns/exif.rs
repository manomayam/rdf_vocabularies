// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Exif data description vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Exif data description vocabulary|
//! |**Prefix**|exif|
//! |**Namespace base IRI**|[http://www.w3.org/2003/12/exif/ns#](http://www.w3.org/2003/12/exif/ns#)|
//! |**Description**|Vocabulary to describe an Exif format picture data. All Exif 2.2 tags are defined as RDF properties, as well as several terms to help this schema.|
//! |**Is defined by**|[http://www.w3.org/2003/12/exif/ns#](http://www.w3.org/2003/12/exif/ns#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2003/12/exif/ns#",;
    /// `IFD`: An Image File Directory
    IFD, "IFD",
    /// `Unknown tag`: An Exif tag whose meaning is not known
    _unknown, "_unknown",
    /// `ApertureValue`: レンズ絞り値（APEX値）。この値のルート2のベキ乗を取ると、通常の絞り値となる。例えば'5'の場合だと√2^5=F5.6
    apertureValue, "apertureValue",
    /// `Artist`: Person who created the image
    artist, "artist",
    /// `BitsPerSample`: The number of bits per image component. In this standard each component of the image is 8 bits, so the value for this tag is 8. See also SamplesPerPixel. In JPEG compressed data a JPEG marker is used instead of this tag.
    bitsPerSample, "bitsPerSample",
    /// `BrightnessValue`: 画像撮影時の撮影対象物の明るさ（APEX値）。EVにするにはISO感度の値を足す必要があり、Ev:Exposure, Bv:BrightnessValue, Sv:SensitivityValueとしてEv=Bv+Sv Sv=log^2(ISOSpeedRating/3.125)という計算式を使う。ISO感度100の場合Sv=5、ISO200の場合Sv=6、ISO125ではSv=5.32
    brightnessValue, "brightnessValue",
    /// `CFAPattern`: CCDに付いているColor filter array(CFA)のパターン。例えば普通のRGBフィルターだと、CFAPatternのデータは 02 02 00 01 01 02
    cfaPattern, "cfaPattern",
    /// `ColorSpace`: 使われる色空間。通常sRGB
    colorSpace, "colorSpace",
    /// `ComponentsConfiguration`: Information specific to compressed data. The channels of each component are arranged in order from the 1st component to the 4th. For uncompressed data the data arrangement is given in the PhotometricInterpretation tag. However, since PhotometricInterpretation can only express the order of Y,Cb and Cr, this tag is provided for cases when compressed data uses components other than Y, Cb, and Cr and to enable support of other sequences.
    componentsConfiguration, "componentsConfiguration",
    /// `CompressedBitsPerPixel`: 画像の圧縮率
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
    /// `DateTime`: この画像が作成（あるいは最後に変更された）日時。通常はDateTimeOriginaと同じ値
    dateTime, "dateTime",
    /// `DateTimeDigitized`: 画像がデジタル化された日時。デジカメ画像なら通常DateTimeOriginalと同じ値
    dateTimeDigitized, "dateTimeDigitized",
    /// `DateTimeOriginal`: The date and time when the original image data was generated. For a DSC the date and time the picture was taken are recorded.
    dateTimeOriginal, "dateTimeOriginal",
    /// `DeviceSettingDescription`: Information on the picture-taking conditions of a particular camera model. The tag is used only to indicate the picture-taking conditions in the reader.
    deviceSettingDescription, "deviceSettingDescription",
    /// `DigitalZoomRatio`: デジタルズームが使われた場合、その比率
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
    /// `ExposureMode`: 露光モード
    exposureMode, "exposureMode",
    /// `ExposureProgram`: The class of the program used by the camera to set exposure when the picture is taken.
    exposureProgram, "exposureProgram",
    /// `ExposureTime`: Exposure time, given in seconds (sec).
    exposureTime, "exposureTime",
    /// `FNumber`: F number
    fNumber, "fNumber",
    /// `FileSource`: 画像がどういうデバイスから得られたか。通常3=DSC
    fileSource, "fileSource",
    /// `Flash`: フラッシュ発光の状態
    flash, "flash",
    /// `FlashEnergy`: The strobe energy at the time the image is captured, as measured in Beam Candle Power Seconds (BCPS).
    flashEnergy, "flashEnergy",
    /// `FlashpixVersion`: The Flashpix format version supported by a FPXR file. If the FPXR function supports Flashpix format Ver. 1.0, this is indicated similarly to ExifVersion by recording "0100" as 4-byte ASCII.
    flashpixVersion, "flashpixVersion",
    /// `FocalLength`: レンズの焦点距離。単位はmm
    focalLength, "focalLength",
    /// `FocalLengthIn35mmFilm`: 35mm換算した焦点距離
    focalLengthIn35mmFilm, "focalLengthIn35mmFilm",
    /// `FocalPlaneResolutionUnit`: The unit for measuring FocalPlaneXResolution and FocalPlaneYResolution. This value is the same as the ResolutionUnit.
    focalPlaneResolutionUnit, "focalPlaneResolutionUnit",
    /// `FocalPlaneXResolution`: 撮影画像のCCD位置での水平解像度。この値とFocalPlaneYResolution、FocalLengthを使うと、レンズ焦点距離の35mmカメラ換算値が計算できる。例えば200万画素機を使いVGAモードの画像を撮ったような場合はこの値はVGAの解像度でリサンプルされた値になっており、CCDの画素ピッチそのままの値ではないので注意が必要
    focalPlaneXResolution, "focalPlaneXResolution",
    /// `FocalPlaneYResolution`: 撮影画像のCCD位置での垂直解像度
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
    /// `GPSLatitude`: 緯度
    gpsLatitude, "gpsLatitude",
    /// `GPSLatitudeRef`: 緯度の北緯もしくは南緯
    gpsLatitudeRef, "gpsLatitudeRef",
    /// `GPSLongitude`: The longitude, expressed as three values giving the degrees, minutes, and seconds, respectively.
    gpsLongitude, "gpsLongitude",
    /// `GPSLongitudeRef`: 経度の東経もしくは西経
    gpsLongitudeRef, "gpsLongitudeRef",
    /// `GPSMapDatum`: 測地系。日本なら'TOKYO'もしくは'WGS-84'
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
    /// `Interoperability Info`: An attribute relating to Interoperability. Tags stored in<br>Interoperability IFD may be defined dependently to each Interoperability rule.
    interopInfo, "interopInfo",
    /// `InteroperabilityIndex`: メイン画像のInteroperability IFDで、データの内容がExifR98 v1.0準拠の場合は、'R98'の文字列。サムネィル画像のInteroperability IFDの場合は、'THM'の文字列
    interoperabilityIndex, "interoperabilityIndex",
    /// `InteroperabilityVersion`: データの内容がExifR98 v1.0準拠の場合は、'0100'の文字列。
    interoperabilityVersion, "interoperabilityVersion",
    /// `Interoperability IFD Pointer`: A pointer to the Interoperability IFD, which is composed of tags storing the information to ensure the Interoperability
    interoperability_IFD_Pointer, "interoperability_IFD_Pointer",
    /// `ISOSpeedRatings`: CCD感度の銀塩フィルム換算値
    isoSpeedRatings, "isoSpeedRatings",
    /// `JPEGInterchangeFormat`: The offset to the start byte (SOI) of JPEG compressed thumbnail data. This is not used for primary image JPEG data.
    jpegInterchangeFormat, "jpegInterchangeFormat",
    /// `JPEGInterchangeFormatLength`: The number of bytes of JPEG compressed thumbnail data. This is not used for primary image JPEG data.
    jpegInterchangeFormatLength, "jpegInterchangeFormatLength",
    /// `Length`: Length of an object. Could be a subProperty of other general schema.
    length, "length",
    /// `LightSource`: Light source such as Daylight, Tungsten, Flash etc.
    lightSource, "lightSource",
    /// `Make`: カメラのメーカー名。DCFでは必須
    make, "make",
    /// `MakerNote`: カメラの内部情報等、メーカー依存データ
    makerNote, "makerNote",
    /// `MaxApertureValue`: The smallest F number of the lens. The unit is the APEX value. Ordinarily it is given in the range of 00.00 to 99.99, but it is not limited to this range.
    maxApertureValue, "maxApertureValue",
    /// `Meter`: A length with unit of meter
    meter, "meter",
    /// `MeteringMode`: 自動露出の測光モード
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
    /// `PrimaryChromaticities`: 原色の色度。CCIR REcommendation 709 primariesを使っている場合は、'640/1000,330/1000,300/1000,600/1000,150/1000,60/1000'という値。通常はColorSpaceタグがあるので不要
    primaryChromaticities, "primaryChromaticities",
    /// `PrintImageMatching IFD Pointer`: A pointer to the print image matching IFD
    printImageMatching_IFD_Pointer, "printImageMatching_IFD_Pointer",
    /// `Recording Offset`: An attribute relating to recording offset
    recOffset, "recOffset",
    /// `ReferenceBlackWhite`: 画像情報の黒点・白点値。YCbCr形式の場合、最初の２つの値がY(輝度)の黒点・白点、次がCb、最後がCrとなり、デフォルト値は'0,255,0,128,0,128'。RGB形式の場合はR、G、Bの順で黒点・白点値が並んでおり、デフォルト値は'0,255,0,255,0,255'。
    referenceBlackWhite, "referenceBlackWhite",
    /// `Related File`: Tag Relating to Related File Information
    relatedFile, "relatedFile",
    /// `RelatedImageFileFormat`: Related image file format
    relatedImageFileFormat, "relatedImageFileFormat",
    /// `RelatedImageLength`: 画像高。通常はサムネイル画像のInteroperability IFDに使う
    relatedImageLength, "relatedImageLength",
    /// `RelatedImageWidth`: 画像幅。通常はサムネイル画像のInteroperability IFDに使う
    relatedImageWidth, "relatedImageWidth",
    /// `RelatedSoundFile`: 画像と一緒に音声録音できる機種の場合に、音声ファイルの名前
    relatedSoundFile, "relatedSoundFile",
    /// `Resolution`: a rational number representing a resolution. Could be a subProperty of other general schema.
    resolution, "resolution",
    /// `ResolutionUnit`: XResolution/YResolutionの解像度の単位
    resolutionUnit, "resolutionUnit",
    /// `RowsPerStrip`: The number of rows per strip. This is the number of rows in the image of one strip when an image is divided into strips. With JPEG compressed data this designation is not needed and is omitted.
    rowsPerStrip, "rowsPerStrip",
    /// `SamplesPerPixel`: The number of components per pixel. Since this standard applies to RGB and YCbCr images, the value set for this tag is 3. In JPEG compressed data a JPEG marker is used instead of this tag.
    samplesPerPixel, "samplesPerPixel",
    /// `Saturation`: The direction of saturation processing applied by the camera when the image was shot.
    saturation, "saturation",
    /// `SceneCaptureType`: The type of scene that was shot. It can also be used to record the mode in which the image was shot, such as Landscape, Portrait etc. Note that this differs from the scene type (SceneType) tag.
    sceneCaptureType, "sceneCaptureType",
    /// `SceneType`: 画像がどうやって撮られたか。デジカメの場合は通常1=A directly photographed image
    sceneType, "sceneType",
    /// `Seconds`: a mesurement of time length with unit of second
    seconds, "seconds",
    /// `SensingMethod`: The image sensor type on the camera or input device, such as One-chip color area sensor etc.
    sensingMethod, "sensingMethod",
    /// `Sharpness`: The direction of sharpness processing applied by the camera when the image was shot.
    sharpness, "sharpness",
    /// `ShutterSpeedValue`: シャッター速度（APEX値）。この値の2のベキ乗をとって逆数にすると、通常のシャッター速度表示になる。例えば'4'の場合だと1/(2^4)=1/16秒
    shutterSpeedValue, "shutterSpeedValue",
    /// `Software`: デジカメ（もしくは入力機器）の内蔵ソフトウェア/ファームウェアの名称とバージョン
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
    /// `SubjectArea`: 画面中の主たる被写体の位置。2値によるXY座標、3値による円、4値による長方形座標の3通りの示し方がある
    subjectArea, "subjectArea",
    /// `SubjectDistance`: 被写体までの距離。単位はm
    subjectDistance, "subjectDistance",
    /// `SubjectDistanceRange`: The distance to the subject, such as Macro, Close View or Distant View.
    subjectDistanceRange, "subjectDistanceRange",
    /// `SubjectLocation`: The location of the main subject in the scene. The value of this tag represents the pixel at the center of the main subject relative to the left edge, prior to rotation processing as per the Rotation tag. The first value indicates the X column number and second indicates the Y row number.
    subjectLocation, "subjectLocation",
    /// ``: 
    subsecond, "subsecond",
    /// `Subseconds`: A tag used to record fractions of seconds for a date property
    subseconds, "subseconds",
    /// `Tag number`: The Exif tag number
    tag_number, "tag_number",
    /// `Tag ID`: The Exif tag number with context prefix, such as IFD type or maker name
    tagid, "tagid",
    /// `TransferFunction`: A transfer function for the image, described in tabular style. Normally this tag is not necessary, since color space is specified in the color space information tag (ColorSpace).
    transferFunction, "transferFunction",
    /// `UserComment`: ユーザーコメント。ImageDescriptionタグと違って、こちらはJIS2バイトコード、Unicode等での記述が許されており、最初の8バイトが文字コード
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
    /// `XResolution`: 画像の表示・印刷時の水平解像度
    xResolution, "xResolution",
    /// `YCbCrCoefficients`: The matrix coefficients for transformation from RGB to YCbCr image data.
    yCbCrCoefficients, "yCbCrCoefficients",
    /// `YCbCrPositioning`: 色情報のサンプリングを間引きしている場合に、色情報のサンプルポイントがどこになるか。1='centered'なら点集合の中央、2＝'co-sited'なら点集合の原点
    yCbCrPositioning, "yCbCrPositioning",
    /// `YCbCrSubSampling`: The sampling ratio of chrominance components in relation to the luminance component. In JPEG compressed data a JPEG marker is used instead of this tag.
    yCbCrSubSampling, "yCbCrSubSampling",
    /// `YResolution`: The number of pixels per ResolutionUnit in the ImageLength direction. The same value as XResolution is designated.
    yResolution, "yResolution"
);
