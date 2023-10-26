use konfig::WithDocs;
use serde::Serialize;
use std::collections::BTreeMap;
use std::net::{Ipv4Addr, Ipv6Addr};

/// A complex data structure that contains substructures, enums, and fields with types from
/// `std::net` and all primitive types, as well as structure and unit enum variant types.
#[derive(Serialize, WithDocs)]
struct ComplexData {
    /// A substructure that contains a list of items.
    items: ItemList,

    /// An enum that represents different types of values.
    values: ValueEnum,

    /// A substructure that contains a map of properties.
    properties: PropertyMap,

    /// A boolean value.
    boolean: bool,

    /// An integer value.
    integer: i32,

    /// A list of numbers.
    numbers: Vec<usize>,

    /// A floating-point value.
    float: f64,

    /// A string value.
    string: String,

    /// A character value.
    character: char,

    /// An IPv4 address.
    ipv4_address: Ipv4Addr,

    /// An IPv6 address.
    ipv6_address: Ipv6Addr,

    /// A structure variant with two fields.
    structure_variant: StructureVariant,

    /// A unit variant.
    unit_variant: UnitVariant,
}

/// A list of items.
#[derive(Serialize, WithDocs)]
struct ItemList {
    /// The items in the list.
    items: Vec<Item>,
}

/// An item in the list.
#[derive(Serialize, WithDocs)]
struct Item {
    /// The name of the item.
    name: String,

    /// The value of the item.
    value: u32,
}

/// An enum that represents different types of values.
#[derive(Serialize, WithDocs)]
enum ValueEnum {
    /// A boolean value.
    Boolean(bool),

    /// An integer value.
    Integer(i32),

    /// A floating-point value.
    Float(f64),

    /// A string value.
    String(String),

    /// An array of values.
    Array(Vec<ValueEnum>),
}

/// An enum that represents a structure variant with two fields.
#[derive(Serialize, WithDocs)]
enum StructureVariant {
    /// The variant with two fields.
    Variant(StructVariantFields),
}

#[derive(Serialize, WithDocs)]
struct StructVariantFields {
    /// The name of the variant.
    name: String,

    /// The value of the variant.
    value: i32,
}

/// An enum that represents a unit variant.
#[derive(Serialize, WithDocs)]
enum UnitVariant {
    /// The unit variant.
    Variant,
}

/// A map of integer values.
#[derive(Serialize, WithDocs)]
struct IntMap {
    /// The map of integer values.
    map: BTreeMap<String, i32>,
}

/// A map of string values.
#[derive(Serialize, WithDocs)]
struct StringMap {
    /// The map of string values.
    map: BTreeMap<String, String>,
}

/// A substructure that contains a map of properties.
#[derive(Serialize, WithDocs)]
struct PropertyMap {
    /// The map of properties.
    properties: BTreeMap<String, Property>,
}

/// A property.
#[derive(Serialize, WithDocs)]
struct Property {
    /// The name of the property.
    name: String,

    /// The value of the property.
    value: ValueEnum,
}

#[test]
fn serialize_complex_structure() {
    // Define the complex data structure.
    let val = ComplexData {
        items: ItemList {
            items: vec![
                Item {
                    name: "item1".to_string(),
                    value: 10,
                },
                Item {
                    name: "item2".to_string(),
                    value: 20,
                },
            ],
        },
        values: ValueEnum::Array(vec![
            ValueEnum::Integer(1),
            ValueEnum::String("two".to_string()),
            ValueEnum::Float(3.14),
        ]),
        properties: PropertyMap {
            properties: {
                let mut map = BTreeMap::new();
                map.insert(
                    "prop1".to_string(),
                    Property {
                        name: "prop1".to_string(),
                        value: ValueEnum::Boolean(true),
                    },
                );
                map.insert(
                    "prop2".to_string(),
                    Property {
                        name: "prop2".to_string(),
                        value: ValueEnum::Integer(42),
                    },
                );
                map.insert(
                    "prop3".to_string(),
                    Property {
                        name: "prop3".to_string(),
                        value: ValueEnum::String("hello".to_string()),
                    },
                );
                map
            },
        },
        boolean: true,
        integer: 123,
        numbers: vec![1, 2, 3],
        float: 3.1415,
        string: "rust is awesome".to_string(),
        character: 'c',
        ipv4_address: Ipv4Addr::new(192, 168, 0, 1),
        ipv6_address: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
        structure_variant: StructureVariant::Variant(StructVariantFields {
            name: "variant".to_string(),
            value: 42,
        }),
        unit_variant: UnitVariant::Variant,
    };

    let serialized = konfig::to_string(&val).unwrap();
    let expected = include_str!("./data/expected/complex_structure.konfig.md");

    println!("{}", konfig::to_string_with_docs(&val).unwrap());

    assert_eq!(
        serialized,
        include_str!("./data/expected/complex_structure.konfig.md"),
        "expected:\n\n{}\n\n but got:\n\n{}\n\n",
        expected,
        serialized
    )
}
