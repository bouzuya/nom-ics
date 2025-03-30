use std::collections::BTreeMap;

struct CalendarObject(Vec<Property>, Vec<Component>);

struct Component(ComponentName, Vec<Property>, Vec<Component>);

struct ComponentName(String);

struct Property(
    PropertyName,
    BTreeMap<ParameterName, ParameterValue>,
    PropertyValue,
);

struct PropertyName(String);

struct ParameterName(String);

struct ParameterValue(Vec<String>);

enum PropertyValue {
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
