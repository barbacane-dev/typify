# Typify (barbacane-dev fork)

Typify compiles JSON Schema documents into Rust types. This is the
[barbacane-dev fork](https://github.com/barbacane-dev/typify), an
independently maintained line of development based on
[oxidecomputer/typify 0.6.1](https://github.com/oxidecomputer/typify) and
extended with broader JSON Schema support, more conformant code generation,
and a handful of long-standing bug fixes. See
[What's new vs upstream](#whats-new-vs-upstream) for the change set.

There are three ways to use typify:

- the `cargo typify` command (see [`cargo-typify/README.md`](./cargo-typify/README.md))
- the `import_types!("types.json")` macro to generate Rust types directly in your program
- the `TypeSpace` builder API for use in `build.rs`, `xtask`, or stand-alone generators

**If generation fails, doesn't compile, or is generally lousy**: please
file an issue and include the JSON Schema and Rust output (if there is
any). Use the `cargo typify` command to generate code from the command
line. It is even more helpful if you can articulate the output you'd
ideally like to see.

## Install

This fork is **git-only** — there is no crates.io publication. Depend
on a tagged release directly in `Cargo.toml`:

```toml
[dependencies]
typify = { git = "https://github.com/barbacane-dev/typify", tag = "v1.0.0" }
```

Or, for the macro / builder use case:

```toml
[dependencies]
typify = { git = "https://github.com/barbacane-dev/typify", tag = "v1.0.0" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

For `cargo typify`, install the binary from the same tag:

```console
$ cargo install --git https://github.com/barbacane-dev/typify --tag v1.0.0 cargo-typify
```

## Quick start

### Macro

```rust
use typify::import_types;

import_types!(schema = "schema.json");
```

### Builder

```rust
use typify::{TypeSpace, TypeSpaceSettings};

let content = std::fs::read_to_string("schema.json")?;
let schema = serde_json::from_str(&content)?;

let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
type_space.add_schema_from_value(schema)?;

// `to_stream()` returns a `TokenStream` — pair with prettyplease or rustfmt:
let code = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);
std::fs::write("generated.rs", code)?;
```

### CLI

```console
$ cargo typify schema.json                 # writes schema.rs next to schema.json
$ cargo typify schema.json --output -      # to stdout
$ cargo typify schema.json --schema-dir ./schemas/    # bundle external $refs
```

## What's new vs upstream

This fork carries every change in 1.0.0 on top of upstream 0.6.1. The
full list lives in [`CHANGELOG.adoc`](./CHANGELOG.adoc); the highlights:

**New JSON Schema features**

- JSON Schema **2020-12 and 2019-09** support via automatic
  normalization. `$defs`, `prefixItems`, `dependentRequired`,
  `dependentSchemas`, `unevaluatedProperties`, and `$dynamicRef` are
  transparently lowered to draft-07 before processing.
- **External `$ref`** support: a schema can reference types in other
  files. Use `TypeSpace::add_schema_with_externals()`, or pass
  `--schema-dir` to `cargo typify`.
- **Non-`$defs` internal `$ref`** support: any JSON Pointer path
  (`#/properties/foo`) resolves correctly, not just
  `#/definitions/...` / `#/$defs/...`.
- **`if` / `then` / `else`** support via transformation to `oneOf`.

**More conformant code generation**

- Untagged enum variants are ordered Integer-before-Number, so JSON
  numbers deserialize to the narrower variant when both fit.
- Bounded integer newtypes get `TryFrom` (enforcing min/max) instead of `From`.
- Bounded integer ranges like `[1..32]` use the smallest fitting type
  (`NonZeroU8`) instead of `NonZeroU64`.
- 2020-12 `prefixItems` + `items: false` generates a Rust tuple.
- `anyOf` overhauled: the previous broken flattened-struct output is
  replaced with proper `#[serde(untagged)]` enum generation, including
  for primitive variants and mixed object/primitive schemas.
- `not` schemas no longer panic; complex `not` falls back to
  `serde_json::Value`, enumerated `not` becomes a deny-list newtype.
- `patternProperties` and `propertyNames` no longer panic on edge cases.

**Fixes**

- Multi-type `type: [a, b, c]` unions combined with `oneOf`/`anyOf`/`allOf`/`not`
  no longer silently drop variants.
- `required` fields with intrinsic-default Rust types (`Vec`, `HashMap`, `Option<T>`)
  now honour the schema's wire contract — see
  [Migration notes from 0.6.1](#migration-notes-from-061).
- Special characters in enum variant names (`=`, `>`, `≥`, etc.) no
  longer panic.
- Integer `minimum`/`maximum` render as integers (not floats) in doc comments.
- The bundler correctly pulls in and rewrites sibling refs inside
  bundled external schemas.
- All known panics in `ref_key()`, `convert_reference()`, etc. are
  replaced with proper error returns.

**Quality of life**

- `TypeSpace::add_schema_from_value()` — auto-detects and normalizes
  any JSON Schema draft.
- `TypeSpace::add_schema_with_externals()` — handles multi-file schema bundles.
- `cargo typify`: auto-discovers external schemas from the input
  directory; `--schema-dir` for explicit control; transparent 2020-12
  normalization (no flag required).

## Migration notes from 0.6.1

The 1.0.0 release contains one **wire-format** change. Schema-level
behaviour and Rust API are otherwise additive.

### Required fields with intrinsic Rust defaults

In 0.6.1, when the schema marked a field `required` but the Rust type
had an intrinsic default (`Vec`, `HashMap`, `Option<T>`), the
generated code applied both `#[serde(default)]` and
`skip_serializing_if`. Deserialize was lenient, but serialize silently
**omitted** the field when it was empty — violating the schema's
`required` contract.

1.0.0 promotes those fields to a new internal state that emits
`#[serde(default)]` alone. Deserialize stays lenient, but serialize
**always** renders the field.

For schema `{ required: ["tags"], properties: { tags: { type: "array" } } }`
and value `Foo { tags: vec![] }`:

| | 0.6.1 | 1.0.0 |
|---|---|---|
| Deserialize `{}` | OK | OK |
| Serialize `Foo { tags: vec![] }` | `{}` | `{"tags":[]}` |

**What you need to do**: if any downstream consumer compares
serialized output byte-for-byte (snapshot tests, diff-based audit
tools, mock servers), expect previously-omitted required fields to
appear as `[]`, `{}`, or `null`. Payload size grows accordingly. Round
trips are now lossless.

## JSON Schema → Rust types

JSON Schema is a constraint language designed for validation. As a
result, it is not well-suited — and is often seemingly hostile — to
translation into constructive type systems. It allows for expressions
of arbitrary complexity with an infinity of ways to articulate a given
set of constraints. Typify does its best to discern an appropriate
interpretation, but it is far from perfect.

### Built-in types

Integers, floating-point numbers, strings, etc. all have
straightforward representations in Rust. The only significant nuance
is selecting the appropriate built-in type based on type attributes:
for example, a JSON Schema with `minimum`/`maximum` indicates the
appropriate integral type to use, and a `format` like `"uuid"` maps to
`uuid::Uuid` (requiring the `uuid` crate as a dependency).

### Arrays

JSON Schema arrays turn into one of three Rust types: `Vec<T>`,
`HashSet<T>`, or a tuple. An array may have a fixed length that
matches a fixed list of item types (via `prefixItems` + `items: false`
in 2020-12, or the legacy `items` array form); this becomes a Rust
tuple. `uniqueItems: true` produces `HashSet<T>`; otherwise `Vec<T>`.

### Objects

In general, objects turn into Rust structs. If the schema defines no
properties, typify emits a `HashMap<String, T>` (where `T` comes from
`additionalProperties`) or `HashMap<String, serde_json::Value>` otherwise.

Properties not in `required` are typically `Option<T>` with
`#[serde(default)]`. Non-required properties whose Rust type already
has a default value (such as `Vec<T>`) keep their type and get
`#[serde(default, skip_serializing_if = ...)]` (so you don't see
`Option<Vec<T>>`).

Required properties whose Rust type has an intrinsic default get
`#[serde(default)]` **alone** — see
[Migration notes from 0.6.1](#migration-notes-from-061).

#### Alternate Map types

By default, typify uses `std::collections::HashMap`. To use
`std::collections::BTreeMap` or a third-party map (e.g.
`indexmap::IndexMap`), call `with_map_type` on `TypeSpaceSettings`
with the full path to the type. See
`TypeSpaceSettings::with_map_type` for the trait requirements on
custom map types.

### OneOf

`oneOf` maps to a Rust enum. Typify selects the appropriate
[serde enum representation](https://serde.rs/enum-representations.html)
(external, internal, adjacent, or untagged).

### AllOf

`allOf` is handled by merging schemas. Typify tries to preserve and
share type names where it can, but you may end up with replicated
fields in some cases.

### AnyOf

`anyOf` maps to a `#[serde(untagged)]` enum. Typify runs the same enum
detection pipeline (external, internal, adjacent, untagged) as `oneOf`.
This handles `anyOf` with primitive types (strings, integers), mixed
object/primitive schemas, and overlapping object properties.

### AdditionalProperties

`additionalProperties: false` becomes `#[serde(deny_unknown_fields)]`.
A missing `additionalProperties` (or `true`) means any other property
is permitted and silently ignored. When `additionalProperties` is a
schema, the struct gets a `#[serde(flatten)]` map field of the
appropriate value type.

### If / Then / Else

`if`/`then`/`else` is transformed into a `oneOf`: the `then` branch
becomes `allOf(if, then)`, and the `else` branch becomes
`allOf(not(if), else)`. The result is a Rust enum with one variant per
branch.

### Not

`not` with enumerated values produces a deny-list newtype that rejects
those values at construction. More complex `not` schemas (e.g.,
`not: { type: "object" }`) fall back to `serde_json::Value`.

### JSON Schema 2020-12 / 2019-09

Typify normalizes these drafts to draft-07 before processing:

| 2020-12 keyword | Lowered to |
|-----------------|-----------|
| `$defs` | `definitions` |
| `prefixItems` + `items` | `items` (array) + `additionalItems` |
| `$ref` alongside other keywords | `allOf` wrapping |
| `dependentRequired` | `dependencies` (array form) |
| `dependentSchemas` | `dependencies` (schema form) |
| `unevaluatedProperties` | `additionalProperties` (best-effort) |
| `$dynamicRef` | `$ref` (best-effort) |

Use `add_schema_from_value()` to enable auto-detection and
normalization. `cargo typify` does this automatically.

### External references

`$ref` can point to external files (e.g.
`"other-file.json#/definitions/Foo"`). Use
`add_schema_with_externals()` to provide a map of external schemas:

```rust
let mut externals = BTreeMap::new();
externals.insert("types.json".to_string(), types_json_value);
type_space.add_schema_with_externals(main_schema_value, externals)?;
```

Non-standard internal references (e.g.
`$ref: "#/properties/foo"`) are also resolved automatically.

## Rust → Schema → Rust

Schemas derived from Rust types may include an `x-rust-type`
extension that provides information about the original type:

```json
{
  "type": "object",
  "properties": { },
  "x-rust-type": {
    "crate": "crate-o-types",
    "version": "1.0.0",
    "path": "crate_o_types::some_mod::SomeType"
  }
}
```

The extension includes the crate name, a Cargo-style version
requirement, and the full path (which must start with the
ident-converted crate name).

Each typify entry point lets you specify a list of crates and
versions. If the user specifies `crate-o-types@1.0.1`, typify uses the
specified `SomeType` instead of generating one from the schema.

### Using types from other crates

Each mode of using typify has a method for controlling the use of
types with `x-rust-type` annotations. The default is to ignore them.
The recommended method is to specify each crate and version you
intend to use. You can also supply `*` as the version (which may
result in incompatibilities), or define a policy to allow the use of
all "unknown" crates (which may require adding dependencies for those
crates).

CLI:
```console
$ cargo typify --unknown-crates allow --crate oxnet@1.0.0 schema.json
```

Builder:
```rust
let mut settings = typify::TypeSpaceSettings::default();
settings
    .with_unknown_crates(typify::UnknownPolicy::Allow)
    .with_crate("oxnet", typify::CrateVers::Version("1.0.0".parse().unwrap()), None);
```

Macro:
```rust
typify::import_types!(
    schema = "schema.json",
    unknown_crates = Allow,
    crates = {
        "oxnet" = "1.0.0",
    },
);
```

### Version requirements

The `version` field follows the Cargo version requirements spec. If
the extension specifies `0.1.0` and the user is using `0.1.1`, the
type is used; if the extension specifies `0.2.2` and the user is on
`0.2.0`, the type is not used.

Crate authors can commit to greater-than-semver stability with
explicit ranges. `>=0.1.0, <1.0.0` means the type stays
schema-compatible across all releases until 1.0.0. Authors should
populate `version` carefully to uphold type availability.

### Type parameters

`x-rust-type` may specify type parameters:

```json
{
  "$defs": {
    "Sprocket": {
      "type": "object",
      "properties": { },
      "x-rust-type": {
        "crate": "util",
        "version": "0.1.0",
        "path": "util::Sprocket",
        "parameters": [
          { "$ref": "#/$defs/Gizmo" }
        ]
      }
    },
    "Gizmo": {
      "type": "object",
      "properties": { },
      "x-rust-type": {
        "crate": "util",
        "version": "0.1.0",
        "path": "util::Gizmo"
      }
    }
  }
}
```

With `util@0.1.0` specified, references to `#/$defs/Sprocket` use the
type `util::Sprocket<util::Gizmo>`. Parameters may be inline or
referenced schemas.

### Including `x-rust-type` in your library

The schema for the extension value:

```json
{
  "description": "schema for the x-rust-type extension",
  "type": "object",
  "properties": {
    "crate":   { "type": "string", "pattern": "^[a-zA-Z0-9_-]+$" },
    "version": { "type": "string", "description": "semver requirement per Cargo.toml" },
    "path":    { "type": "string", "pattern": "^[a-zA-Z0-9_]+(::[a-zA-Z0-9+]+)*$" },
    "parameters": {
      "type": "array",
      "items": { "$ref": "#/definitions/Schema" }
    }
  },
  "required": ["crate", "path", "version"]
}
```

## Formatting

Pair `TypeSpace::to_stream()` with `prettyplease` (zero external
dependencies, suitable for `build.rs`) or `rustfmt-wrapper`
(idiomatic output, requires `rustfmt`):

```rust
// prettyplease — best for build.rs
prettyplease::unparse(&syn::parse2::<syn::File>(typespace.to_stream())?)
```

```rust
// rustfmt — best for checked-in generation
rustfmt_wrapper::rustfmt(typespace.to_stream().to_string())?
```

```rust
// no formatting — only if no human will ever read it
typespace.to_stream().to_string()
```

## License

Apache-2.0. Original work © Oxide Computer Company; modifications ©
barbacane-dev contributors. See [`LICENSE`](./LICENSE).

## Acknowledgements

This project is a fork of
[oxidecomputer/typify](https://github.com/oxidecomputer/typify) at
the 0.6.1 baseline. All credit for the original architecture and
substantial portions of the code goes to its authors. The fork exists
to ship JSON Schema 2020-12 coverage, external `$ref` support, and a
catalogue of long-standing bug fixes on a schedule independent of
upstream.
