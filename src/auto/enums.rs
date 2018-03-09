// This file was generated by gir (https://github.com/gtk-rs/gir @ 6704a1c)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e)
// DO NOT EDIT

use error::ErrorDomain;
use ffi;
use ffi as glib_ffi;
use translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChecksumType {
    Md5,
    Sha1,
    Sha256,
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    Sha512,
    #[cfg(any(feature = "v2_52", feature = "dox"))]
    Sha384,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ChecksumType {
    type GlibType = ffi::GChecksumType;

    fn to_glib(&self) -> ffi::GChecksumType {
        match *self {
            ChecksumType::Md5 => ffi::G_CHECKSUM_MD5,
            ChecksumType::Sha1 => ffi::G_CHECKSUM_SHA1,
            ChecksumType::Sha256 => ffi::G_CHECKSUM_SHA256,
            #[cfg(any(feature = "v2_36", feature = "dox"))]
            ChecksumType::Sha512 => ffi::G_CHECKSUM_SHA512,
            #[cfg(any(feature = "v2_52", feature = "dox"))]
            ChecksumType::Sha384 => ffi::G_CHECKSUM_SHA384,
            ChecksumType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GChecksumType> for ChecksumType {
    fn from_glib(value: ffi::GChecksumType) -> Self {
        match value {
            0 => ChecksumType::Md5,
            1 => ChecksumType::Sha1,
            2 => ChecksumType::Sha256,
            #[cfg(any(feature = "v2_36", feature = "dox"))]
            3 => ChecksumType::Sha512,
            #[cfg(any(feature = "v2_52", feature = "dox"))]
            4 => ChecksumType::Sha384,
            value => ChecksumType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DateMonth {
    BadMonth,
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for DateMonth {
    type GlibType = ffi::GDateMonth;

    fn to_glib(&self) -> ffi::GDateMonth {
        match *self {
            DateMonth::BadMonth => ffi::G_DATE_BAD_MONTH,
            DateMonth::January => ffi::G_DATE_JANUARY,
            DateMonth::February => ffi::G_DATE_FEBRUARY,
            DateMonth::March => ffi::G_DATE_MARCH,
            DateMonth::April => ffi::G_DATE_APRIL,
            DateMonth::May => ffi::G_DATE_MAY,
            DateMonth::June => ffi::G_DATE_JUNE,
            DateMonth::July => ffi::G_DATE_JULY,
            DateMonth::August => ffi::G_DATE_AUGUST,
            DateMonth::September => ffi::G_DATE_SEPTEMBER,
            DateMonth::October => ffi::G_DATE_OCTOBER,
            DateMonth::November => ffi::G_DATE_NOVEMBER,
            DateMonth::December => ffi::G_DATE_DECEMBER,
            DateMonth::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GDateMonth> for DateMonth {
    fn from_glib(value: ffi::GDateMonth) -> Self {
        match value {
            0 => DateMonth::BadMonth,
            1 => DateMonth::January,
            2 => DateMonth::February,
            3 => DateMonth::March,
            4 => DateMonth::April,
            5 => DateMonth::May,
            6 => DateMonth::June,
            7 => DateMonth::July,
            8 => DateMonth::August,
            9 => DateMonth::September,
            10 => DateMonth::October,
            11 => DateMonth::November,
            12 => DateMonth::December,
            value => DateMonth::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DateWeekday {
    BadWeekday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for DateWeekday {
    type GlibType = ffi::GDateWeekday;

    fn to_glib(&self) -> ffi::GDateWeekday {
        match *self {
            DateWeekday::BadWeekday => ffi::G_DATE_BAD_WEEKDAY,
            DateWeekday::Monday => ffi::G_DATE_MONDAY,
            DateWeekday::Tuesday => ffi::G_DATE_TUESDAY,
            DateWeekday::Wednesday => ffi::G_DATE_WEDNESDAY,
            DateWeekday::Thursday => ffi::G_DATE_THURSDAY,
            DateWeekday::Friday => ffi::G_DATE_FRIDAY,
            DateWeekday::Saturday => ffi::G_DATE_SATURDAY,
            DateWeekday::Sunday => ffi::G_DATE_SUNDAY,
            DateWeekday::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GDateWeekday> for DateWeekday {
    fn from_glib(value: ffi::GDateWeekday) -> Self {
        match value {
            0 => DateWeekday::BadWeekday,
            1 => DateWeekday::Monday,
            2 => DateWeekday::Tuesday,
            3 => DateWeekday::Wednesday,
            4 => DateWeekday::Thursday,
            5 => DateWeekday::Friday,
            6 => DateWeekday::Saturday,
            7 => DateWeekday::Sunday,
            value => DateWeekday::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum KeyFileError {
    UnknownEncoding,
    Parse,
    NotFound,
    KeyNotFound,
    GroupNotFound,
    InvalidValue,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for KeyFileError {
    type GlibType = ffi::GKeyFileError;

    fn to_glib(&self) -> ffi::GKeyFileError {
        match *self {
            KeyFileError::UnknownEncoding => ffi::G_KEY_FILE_ERROR_UNKNOWN_ENCODING,
            KeyFileError::Parse => ffi::G_KEY_FILE_ERROR_PARSE,
            KeyFileError::NotFound => ffi::G_KEY_FILE_ERROR_NOT_FOUND,
            KeyFileError::KeyNotFound => ffi::G_KEY_FILE_ERROR_KEY_NOT_FOUND,
            KeyFileError::GroupNotFound => ffi::G_KEY_FILE_ERROR_GROUP_NOT_FOUND,
            KeyFileError::InvalidValue => ffi::G_KEY_FILE_ERROR_INVALID_VALUE,
            KeyFileError::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileError> for KeyFileError {
    fn from_glib(value: ffi::GKeyFileError) -> Self {
        match value {
            0 => KeyFileError::UnknownEncoding,
            1 => KeyFileError::Parse,
            2 => KeyFileError::NotFound,
            3 => KeyFileError::KeyNotFound,
            4 => KeyFileError::GroupNotFound,
            5 => KeyFileError::InvalidValue,
            value => KeyFileError::__Unknown(value),
        }
    }
}

impl ErrorDomain for KeyFileError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::g_key_file_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(KeyFileError::UnknownEncoding),
            1 => Some(KeyFileError::Parse),
            2 => Some(KeyFileError::NotFound),
            3 => Some(KeyFileError::KeyNotFound),
            4 => Some(KeyFileError::GroupNotFound),
            5 => Some(KeyFileError::InvalidValue),
            value => Some(KeyFileError::__Unknown(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SeekType {
    Cur,
    Set,
    End,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for SeekType {
    type GlibType = ffi::GSeekType;

    fn to_glib(&self) -> ffi::GSeekType {
        match *self {
            SeekType::Cur => ffi::G_SEEK_CUR,
            SeekType::Set => ffi::G_SEEK_SET,
            SeekType::End => ffi::G_SEEK_END,
            SeekType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSeekType> for SeekType {
    fn from_glib(value: ffi::GSeekType) -> Self {
        match value {
            0 => SeekType::Cur,
            1 => SeekType::Set,
            2 => SeekType::End,
            value => SeekType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TimeType {
    Standard,
    Daylight,
    Universal,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for TimeType {
    type GlibType = ffi::GTimeType;

    fn to_glib(&self) -> ffi::GTimeType {
        match *self {
            TimeType::Standard => ffi::G_TIME_TYPE_STANDARD,
            TimeType::Daylight => ffi::G_TIME_TYPE_DAYLIGHT,
            TimeType::Universal => ffi::G_TIME_TYPE_UNIVERSAL,
            TimeType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GTimeType> for TimeType {
    fn from_glib(value: ffi::GTimeType) -> Self {
        match value {
            0 => TimeType::Standard,
            1 => TimeType::Daylight,
            2 => TimeType::Universal,
            value => TimeType::__Unknown(value),
        }
    }
}

