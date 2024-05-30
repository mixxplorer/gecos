#![warn(missing_docs)]
#![deny(warnings)]
#![deny(clippy::all)]
#![doc = include_str!("../Readme.md")]

use thiserror::Error;

/// Error type for gecos errors. All public facing Results will carry this error type.
#[derive(Error, Debug)]
pub enum GecosError {
    /// Illegal character for passwd representations
    #[error("String contains invalid char, which is not allowed inside a Gecos field! (Chars ',', ':', '=', '\\', '\"', '\\n' are not allowed)")]
    IllegalPasswdChar(char),
}

/// The raw Gecos struct.
///
/// See [the man page](https://man.freebsd.org/cgi/man.cgi?query=passwd&sektion=5) for an introduction of the format.
///
/// To set most of the fields, you need to assign a [GecosSanitizedString] in order to ensure the strings do not contain ',', ':', '=', '\', '"', '\n'
/// This is done in compatibility with [chfn](https://github.com/util-linux/util-linux/blob/0284eb3a8a6505dd9745b042089046ad368bfe74/login-utils/chfn.c#L121C6-L121C26).
///
/// ## Usage
///
/// You can create new gecos object and also parse existing ones from their string representation.
///
/// ## Creating gecos
///
/// Every field except other can be None, if there is no value. When creating an object, you can do this explicitly:
///
/// ```rust
/// # use gecos::Gecos;
/// #
/// let gecos = Gecos {
///     full_name: None,
///     room: None,
///     work_phone: None,
///     home_phone: None,
///     other: vec![],
/// };
///
/// // the most simple outcome, everything is just empty,
/// // therefore the `to_gecos_string` function produces only ','.
/// assert_eq!(gecos.to_gecos_string(), ",,,,")
/// ```
///
/// Of course, you can also set some data, which looks like this. Please note the special case of the `other` field.
/// As some implementations allow multiple "other" fields, you can pass a vector. It is your responsibility to ensure compatibility here.
///
/// ```rust
/// # use std::convert::TryFrom;
/// # use gecos::Gecos;
/// #
/// let gecos = Gecos {
///     full_name: Some("Test Name".to_string().try_into().unwrap()),
///     room: None,
///     work_phone: None,
///     home_phone: None,
///     other: vec![
///         "Some info".to_string().try_into().unwrap(),
///         "More info".to_string().try_into().unwrap()
///     ],
/// };
///
/// assert_eq!(gecos.to_gecos_string(), "Test Name,,,,Some info,More info")
/// ```
///
/// Utilizing [`Gecos::to_gecos_string`] allows converting a [`Gecos`] object to a gecos string like you find it in the passwd database.
///
/// ## Parse gecos
///
/// You can parse gecos objects by calling the [`Gecos::from_gecos_string`] static function of this object. A simple example might look like this:
///
/// ```rust
/// # use gecos::Gecos;
/// #
/// let gecos = Gecos::from_gecos_string("Some Person,,,,").unwrap();
///
/// assert_eq!(gecos.full_name.unwrap().to_string(), "Some Person")
/// ```
///
#[derive(Clone, Debug)]
pub struct Gecos {
    /// like Guest, can be None if empty.
    pub full_name: Option<GecosSanitizedString>,
    /// like H-1.13, can be None if empty.
    pub room: Option<GecosSanitizedString>,
    /// like 574, can be None if empty.
    pub work_phone: Option<GecosSanitizedString>,
    /// like +491606799999, can be None if empty.
    pub home_phone: Option<GecosSanitizedString>,
    /// like a mail address or other important information, vector can be empty if there is no data.
    pub other: Vec<GecosSanitizedString>,
}

/// A struct to ensure the string has none of [',', ':', '=', '\', '"', '\n'] in it, as this would break the gecos string object.
///
/// You can create a new object by converting a String into a GecosSanitizesString like this:
/// ```rust
/// # use std::convert::TryFrom;
/// # use gecos::GecosSanitizedString;
/// #
/// // simple example (type is typically inferred)
/// let name_gecos: GecosSanitizedString = "Another name".to_string().try_into().unwrap();
/// // or more explicitly
/// let room_gecos = GecosSanitizedString::new("St. 9".to_string()).unwrap();
///
/// // converting to standard String
/// let name_string = name_gecos.to_string();
/// ```
#[derive(Clone, Debug)]
pub struct GecosSanitizedString {
    str: String,
}

impl GecosSanitizedString {
    /// Returns a new [GecosSanitizedString] object.
    pub fn new(value: String) -> Result<Self, GecosError> {
        const INVALID_CHARS: [&char; 6] = [&',', &':', &'=', &'\\', &'\"', &'\n'];

        for character in INVALID_CHARS {
            if value.contains(*character) {
                return Err(GecosError::IllegalPasswdChar(*character));
            }
        }
        Ok(Self { str: value })
    }
}

impl TryFrom<String> for GecosSanitizedString {
    type Error = GecosError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a> From<&'a GecosSanitizedString> for &'a String {
    fn from(value: &'a GecosSanitizedString) -> Self {
        &value.str
    }
}

impl std::fmt::Display for GecosSanitizedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl PartialEq for GecosSanitizedString {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl Gecos {
    /// Converts a [Gecos] object to a gecos string like in the passwd database.
    ///
    /// ```rust
    /// # use std::convert::TryFrom;
    /// # use gecos::Gecos;
    /// #
    /// let gecos = Gecos {
    ///     full_name: Some("Test Name".to_string().try_into().unwrap()),
    ///     room: None,
    ///     work_phone: None,
    ///     home_phone: None,
    ///     other: vec![],
    /// };
    ///
    /// assert_eq!(gecos.to_gecos_string(), "Test Name,,,,")
    /// ```
    pub fn to_gecos_string(&self) -> String {
        macro_rules! gecos_element_to_string {
            ($sts:expr) => {
                $sts.as_ref().unwrap_or(&"".to_string().try_into().unwrap())
            };
        }

        format!(
            "{},{},{},{},{}",
            gecos_element_to_string!(self.full_name),
            gecos_element_to_string!(self.room),
            gecos_element_to_string!(self.work_phone),
            gecos_element_to_string!(self.home_phone),
            self.other
                .iter()
                .map(|val| val.into())
                .cloned()
                .collect::<Vec<String>>()
                .join(","),
        )
    }

    /// Converts a gecos string like in passwd database into a [Gecos] object.
    ///
    /// ```rust
    /// # use gecos::Gecos;
    /// #
    /// let gecos = Gecos::from_gecos_string("Some Person,Room,Work phone,Home phone,Other 1,Other 2").unwrap();
    ///
    /// assert_eq!(gecos.full_name.unwrap().to_string(), "Some Person");
    /// assert_eq!(gecos.room.unwrap().to_string(), "Room");
    /// assert_eq!(gecos.work_phone.unwrap().to_string(), "Work phone");
    /// assert_eq!(gecos.home_phone.unwrap().to_string(), "Home phone");
    /// assert_eq!(gecos.other.iter().map(|val| val.to_string()).collect::<Vec<String>>(), ["Other 1", "Other 2"]);
    /// ```
    ///
    /// Also support parsing strings, which do not have all fields populated:
    ///
    ///```rust
    /// # use gecos::Gecos;
    /// #
    /// let gecos = Gecos::from_gecos_string("Some Person,,,Home phone,Other").unwrap();
    ///
    /// assert_eq!(gecos.full_name.unwrap().to_string(), "Some Person");
    /// assert!(gecos.room.is_none());
    /// assert!(gecos.work_phone.is_none());
    /// assert_eq!(gecos.home_phone.unwrap().to_string(), "Home phone");
    /// assert_eq!(gecos.other.iter().map(|val| val.to_string()).collect::<Vec<String>>(), ["Other"]);
    /// ```
    ///
    /// or even incomplete
    ///
    /// ```rust
    /// # use gecos::{Gecos, GecosSanitizedString};
    /// #
    /// let gecos = Gecos::from_gecos_string("Some Person").unwrap();
    ///
    /// assert_eq!(gecos.full_name.unwrap().to_string(), "Some Person");
    /// assert!(gecos.room.is_none());
    /// assert!(gecos.work_phone.is_none());
    /// assert!(gecos.home_phone.is_none());
    /// assert_eq!(gecos.other, Vec::<GecosSanitizedString>::new());
    /// ```
    pub fn from_gecos_string(input: &str) -> Result<Self, GecosError> {
        let mut splitted = input
            .split(',')
            .map(|val| -> Result<GecosSanitizedString, GecosError> { val.to_string().try_into() });

        macro_rules! gecos_string_element_to_gecos_object_element {
            ($sts:expr) => {
                match $sts {
                    Some(option_val) => {
                        match option_val {
                            Ok(val) => {
                                // map empty string to None as well
                                if val.to_string() == "" {
                                    None
                                } else {
                                    Some(val)
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    }
                    None => None,
                }
            };
        }

        Ok(Self {
            full_name: gecos_string_element_to_gecos_object_element!(splitted.next()),
            room: gecos_string_element_to_gecos_object_element!(splitted.next()),
            work_phone: gecos_string_element_to_gecos_object_element!(splitted.next()),
            home_phone: gecos_string_element_to_gecos_object_element!(splitted.next()),
            other: splitted.collect::<Result<Vec<GecosSanitizedString>, GecosError>>()?,
        })
    }
}
