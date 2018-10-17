extern crate named_type;
#[macro_use]
extern crate named_type_derive;

use named_type::NamedType;

#[allow(dead_code)]
#[derive(NamedType)]
struct MyStruct {}

#[test]
fn test_struct() {
    assert_eq!(MyStruct::type_name(), concat!(module_path!(), "::MyStruct"));
    assert_eq!(MyStruct::short_type_name(), "MyStruct");
}

#[allow(dead_code)]
#[derive(NamedType)]
enum MyEnum {}

#[test]
fn test_enum() {
    assert_eq!(MyEnum::type_name(), concat!(module_path!(), "::MyEnum"));
    assert_eq!(MyEnum::short_type_name(), "MyEnum");
}

#[allow(dead_code)]
#[derive(NamedType)]
#[named_type(short_prefix = "Pre")]
enum Prefixed {}

#[test]
fn test_prefix() {
    assert_eq!(Prefixed::type_name(), concat!(module_path!(), "::Prefixed"));
    assert_eq!(Prefixed::short_type_name(), "PrePrefixed");
}

#[allow(dead_code)]
#[derive(NamedType)]
#[named_type(short_suffix = "_suffix")]
struct Suffixed {}

#[test]
fn test_suffix() {
    assert_eq!(Suffixed::type_name(), concat!(module_path!(), "::Suffixed"));
    assert_eq!(Suffixed::short_type_name(), "Suffixed_suffix");
}
