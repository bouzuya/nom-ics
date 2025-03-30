use std::collections::BTreeMap;

struct CalendarObject(Vec<Property>, Vec<Component>);

struct Component(ComponentName, Vec<Property>, Vec<Component>);

struct ComponentName(String);

#[derive(Debug, PartialEq)]
pub struct Property(
    pub(crate) PropertyName,
    pub(crate) BTreeMap<ParameterName, ParameterValue>,
    pub(crate) PropertyValue,
);

#[derive(Debug, PartialEq)]
pub struct PropertyName(pub(crate) String);

#[derive(Debug, PartialEq)]
pub struct ParameterName(String);

#[derive(Debug, PartialEq)]
pub struct ParameterValue(Vec<String>);

#[derive(Debug, PartialEq)]
pub enum PropertyValue {
    Binary(Vec<u8>),
    Boolean(bool),
    CalAddress(String),
    Date(String),
    DateTime(String),
    Duration(String),
    Float(f64),
    Integer(i32),
    Period(String),
    Recur(String),
    Text(String),
    Time(String),
    Uri(String),
    UtcOffset(String),
    XType(String),
}
