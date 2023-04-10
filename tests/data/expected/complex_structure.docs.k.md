* An enum that represents different types of values.
> values = `Array`

* A boolean value.
> boolean = true

* An integer value.
> integer = 123

* A list of numbers.
> numbers = [1, 2, 3]

* A floating-point value.
> float = 3.1414999961853027

* A string value.
> string = "rust is awesome"

* A character value.
> character = "c"

* An IPv4 address.
> ipv4_address = "192.168.0.1"

* An IPv6 address.
> ipv6_address = "::1"

* A structure variant with two fields.
> structure_variant = `Variant`

* A unit variant.
> unit_variant = `Variant`


# An enum that represents different types of values.
### An array of values.
## [0]
----------------------------------------------------

> values > `Array` > [0] > `Integer` = 1


# An enum that represents different types of values.
### An array of values.
## [1]
----------------------------------------------------

> values > `Array` > [1] > `String` = "two"


# An enum that represents different types of values.
### An array of values.
## [2]
----------------------------------------------------

> values > `Array` > [2] > `Float` = 3.140000104904175


# A structure variant with two fields.
## The variant with two fields.
--------------------------------------

* The name of the variant.
> structure_variant > `Variant` > name = "variant"

* The value of the variant.
> structure_variant > `Variant` > value = 42


# A substructure that contains a list of items.
## The items in the list.
### [0]
-----------------------------------------------

* The name of the item.
> items > items > [0] > name = "item1"

* The value of the item.
> items > items > [0] > value = 10


# A substructure that contains a list of items.
## The items in the list.
### [1]
-----------------------------------------------

* The name of the item.
> items > items > [1] > name = "item2"

* The value of the item.
> items > items > [1] > value = 20


# A substructure that contains a map of properties.
## The map of properties.
### [prop1]
---------------------------------------------------

* The name of the property.
> properties > properties > [prop1] > name = "prop1"

* The value of the property.
> properties > properties > [prop1] > value > `Boolean` = true


# A substructure that contains a map of properties.
## The map of properties.
### [prop2]
---------------------------------------------------

* The name of the property.
> properties > properties > [prop2] > name = "prop2"

* The value of the property.
> properties > properties > [prop2] > value > `Integer` = 42


# A substructure that contains a map of properties.
## The map of properties.
### [prop3]
---------------------------------------------------

* The name of the property.
> properties > properties > [prop3] > name = "prop3"

* The value of the property.
> properties > properties > [prop3] > value > `String` = "hello"