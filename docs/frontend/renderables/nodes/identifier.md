# Identifier ([Frontend](../../frontend.md))

A data type used to represent identifiers for [nodes](./node_settings.md) and [types](./type.md) in memory. Effectively a wrapper around an array of heap-allocated strings. The first sub-string in an identifier is reserved for the [unique project identifier hash](../../user_interface/project/project.md), which can not be defined by a user. The last sub-string in an identifier represents the *name* of the element to be identified. Any sub-strings between first and last are considered "modules" for the purpose of namespacing.

Any identifier sub-string besides the project hash (the character set is tbd.) is strictly limited to *lowercase latin* characters (a-z) and the *hyphen* (-) character. The hyphen is included strictly for multi-word naming capabilities. It can not be placed at the beginning or end of an identifier sub-string.

The frontend should capture any hyphen misuses and prevent invalid strings from being created. Additonally, the usage of spaces, underscores and other delimiting characters should also be captured and automatically converted into *hyphens*. Uppercase letters may also be captured and converted to their lowercase variants.

A print function should be included for logging an identifier during development. The colon (:) character should be the default delimiter between identifier sub-strings - however other delimiter options can also be provided.

## Requires

*Nothing*

## Required By

- [Type](./type.md)
- [Node Settings](./node_settings.md)
