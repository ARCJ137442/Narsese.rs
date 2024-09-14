# Narsese.rs

|[简体中文](README.md) | **English**|
|:-:|:-:|

🕒 Last updated time: [2024-09-14 10:19:55]

![License](https://img.shields.io/crates/l/narsese?style=for-the-badge&color=ff7043)
![Code Size](https://img.shields.io/github/languages/code-size/ARCJ137442/Narsese.rs?style=for-the-badge&color=ff7043)
![Lines of Code](https://www.aschey.tech/tokei/github.com/ARCJ137442/Narsese.rs?style=for-the-badge&color=ff7043)
[![Language](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=ff7043)](https://www.rust-lang.org)

<!-- Towards Users -->

Cargo Status:

[![crates.io](https://img.shields.io/crates/v/narsese?style=for-the-badge)](https://crates.io/crates/narsese)
[![docs.rs](https://img.shields.io/docsrs/narsese?style=for-the-badge)](https://docs.rs/narsese)
![Crate Size](https://img.shields.io/crates/size/narsese?style=for-the-badge)

![Recent Downloads](https://img.shields.io/crates/dr/narsese?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/narsese?style=for-the-badge)
![Crates.io Dependents](https://img.shields.io/crates/dependents/narsese?style=for-the-badge)

<!-- Towards Developers -->

Development Status:

[![CI status](https://img.shields.io/github/actions/workflow/status/ARCJ137442/Narsese.rs/ci.yml?style=for-the-badge)](https://github.com/ARCJ137442/Narsese.rs/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-2.0.0-%23FE5196?style=for-the-badge)](https://conventionalcommits.org)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/ARCJ137442/Narsese.rs/latest?style=for-the-badge)

![Created At](https://img.shields.io/github/created-at/ARCJ137442/Narsese.rs?style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/ARCJ137442/Narsese.rs?style=for-the-badge)

## Introduction

This project uses [Semantic Versioning 2.0.0](https://semver.org/) for version number management.

The [**Rust**](https://www.rust-lang.org) implementation of **Narsese**

- ✨Implemented based on the `enum` type of [**Enum Narsese**](#enum-narsese)
- ✨Implemented based on the "nested string lexical tree" of [**Lexical Narsese**](#lexical-narsese)
- 🏗️ A **Narsese API** for developing NARS in Rust (In developing)

## Installation

(Last updated: 2024-04-10)

As a Rust library published on [crates.io](https://crates.io/), you can directly add dependencies in `Cargo.toml`:

```toml
[dependencies.narsese]
version = "0" # Please choose the latest version as needed, and refer to the corresponding sidebar on crates.io if necessary
features = ["bundled"] # Enable all features, including "Enum Narsese" and "Lexical Narsese"
```

List of library features:

- `bundled`: Enable all features
- `enum_narsese`: Enable "Enum Narsese"
- `lexical_narsese`: Enable "Lexical Narsese"

## Usage

(Last updated: 2024-03-29)

### Using / Enum Narsese

```rust
use narsese::enum_nse;

fn main() {
    // Use the quick macro to parse and create Narsese (needed to ensure correct syntax, otherwise panic) //
    let term = enum_term!(<A --> B>);
    let sentence = enum_sentence!("<SELF {-] good>!");
    let task = enum_task!($0.8;0.8;0.8$ <robin --> bird>?);

    // Output / Verification //
    println!("{term:?}");
    assert_eq!(term, enum_nse!("<A --> B>").try_into_term().unwrap()); // The parsing result in string form is equal to it, and use `try_into_term` for downcasting
    println!("{sentence:?}");
    println!("{task:?}");
}
```

### Using / Lexical Narsese

```rust
use narsese::lexical_nse;

fn main() {
    // Use the quick macro to parse and create Narsese (needed to ensure correct syntax, otherwise panic) //
    let term = lexical_term!(<A --> B>);
    let sentence = lexical_sentence!("<SELF {-] good>!");
    let task = lexical_task!($0.8;0.8;0.8$ <robin --> bird>?);

    // Output / Verification //
    println!("{term:?}");
    println!("{sentence:?}");
    println!("{task:?}");
}
```

(More examples can be found in the project's unit tests)

🚧【2024-03-29 00:43:13】WIP: More specific examples

## Usage / Narsese API

🚧【2024-04-05 00:09:03】WIP: Basic functionality

## Concept

### Enum Narsese

✨Implemented based on Rust's native `enum` feature

- ✅Semantic precision
  - Set semantics: The order of elements is irrelevant in unordered compound terms
  - Truth/Budget semantics: Store specific, fixed-range floating-point values
- ✅Structural usability
  - Narsese completeness: All data structures of CommonNarsese are implemented

### Lexical Narsese

✨Implemented based on the "nested string lexical tree"

- ✅Structural completeness
  - Lexical completeness: All lexical structures of CommonNarsese are implemented, such as "compound terms," "statements," "sentences," "tasks"
- ✅Data flexibility
  - Non-category specific: Does not specifically limit the scope of "atomic term prefixes," "compound term connectors," "statement verbs"
  - Extensible: Allows further transformation to specific, dedicated data structures through the "lexical folding" mechanism
    - ✅A "lexical folding" mechanism has been provided for [Enum Narsese](#enum-narsese)

### NarseseAPI

(Last updated: 2024-04-05)

- 🎯 Scalability
  - Through `trait`s, uniformly express and define abstract features that are "independent of specific structures"
- 🎯 Compatibility
  - 📌 Based on CommonNarsese features: Extracts the **semantic** features required by CommonNarsese in the model of "atomic/compound/statement"
  - 📄 Such as: Narsese values, term categories, term capacity, internal term acquisition, and other concepts

#### Standard ASCII Lexicon

(Last updated: 2024-04-05)

It can be used as a CommonNarsese lexical parser (also used for downstream NAVM, BabelNAR) "Standard ASCII Lexicon"

This definition is compatible with the [OpenNARS wiki's Narsese syntax](https://github.com/opennars/opennars/wiki/Narsese-Grammar-(Input-Output-Format)), and some differences and design principles are:

- 📌 **One form only has one expression method**: Terms with the same semantics must have the same expression form (the focus is on limiting the expression form of "compound terms")
- 📌 Not "content-specific": "ASCII Lexicon Narsese" leaves enough space for custom terms (atomic/compound/statement) by "not being specific to enumeration"
- 📌 Task → Statement univocity: For "tasks with default budget values," the default of the budget value will always cause the parser to recognize it as a "statement," without any ambiguity

Defined with the following [PEG grammar](https://en.wikipedia.org/wiki/Parsing_expression_grammar):

```pest
/// Whitespace | All Unicode whitespace symbols, ignored before parsing
WHITESPACE = _{ WHITE_SPACE }

/// Main entry: Lexical Narsese | Priority: Task > Sentence > Term
narsese = {
    task
  | sentence
  | term
}

/// Task: Sentence with budget
task = {
    budget ~ sentence
}

/// Budget value | Does not include the "empty budget" implied by the "empty string"
budget = {
    "$" ~ budget_content ~ "$"
}
/// Content of the budget value
budget_content = {
    (truth_budget_term ~ (";" ~ truth_budget_term)* ~ ";"*)
  | "" // Empty budget (but with parentheses)
}
/// Common items for truth and budget values | Used as internal values, no range constraints
truth_budget_term = @{(ASCII_DIGIT|".")+}

/// Sentence = Term Punctuation Stamp? Truth?
sentence = {
    term ~ punctuation ~ stamp? ~ truth?
}

/// Term = Statement | Compound | Atom
term = {
    statement | compound | atom
}

/// Statement = <Term Predicate Term>
statement = {
    "<" ~ term ~ copula ~ term ~ ">"
}

/// Predicate of the statement
copula = @{
    (punct_sym ~ "-" ~ punct_sym) // Inheritance/Similarity/Instance/Property/Instance-Property
  | (punct_sym ~ "=" ~ punct_sym) // Implication/Equivalence
  | ("=" ~ punct_sym ~ ">") // Temporal Implication
  | ("<" ~ punct_sym ~ ">") // Temporal Equivalence
}

/// Punctuation symbol | Used for "atomic term prefix", "compound term connector", and "statement predicate"
punct_sym = { (PUNCTUATION | SYMBOL) }

/// Compound = (Connector, Terms...) | {Extension Set...} | [Intension Set...]
compound = {
      ("(" ~ connecter ~ "," ~ term ~ ("," ~ term)* ~ ")") // Based on the connector
    | ("{" ~ term ~ ("," ~ term)* ~ "}") // Extension set
    | ("[" ~ term ~ ("," ~ term)* ~ "]") // Intension set
}

/// Compound term connector
connecter = @{ punct_sym ~ (!"," ~ punct_sym)* }

/// Atom = Prefix(optional) Content
atom = {
      "_" ~ // Placeholder
    | (atom_prefix ~ atom_content) // Variable/Interval/Operation...
    | atom_content // Word
}
/// Prefix of the atomic term
atom_prefix = @{ punct_sym+ }
/// Content of the atomic term | To avoid conflict with "compound term connector"
atom_content = @{ atom_char ~ (!copula ~ atom_char)* }
/// Characters that can be used as "content of the atomic term"
atom_char = { LETTER | NUMBER | "_" | "-" }

/// Punctuation
punctuation = { (PUNCTUATION | SYMBOL) }

/// Stamp | An empty timestamp will be directly omitted in the "sentence"
stamp = {
    ":" ~ (!":" ~ ANY)+ ~ ":"
}

/// Truth-value | An empty truth will be directly omitted in the "sentence"
truth = {
  "%" ~ (truth_budget_term ~ (";" ~ truth_budget_term)* ~ ";"*) ~ "%"
}
```

🔗For more details, please refer to the section on "CommonNarsese" in [**JuNarsese.jl**](https://github.com/ARCJ137442/JuNarsese.jl/blob/main/README-en.md#commonnarsese).

This definition can be directly loaded through the Rust library [**pest.rs**](https://pest.rs/) and has been used in BabelNAR for the parsing of the NARS dialect.

## Open Source License

Like most Rust projects, this project is released under a dual license of [MIT](https://choosealicense.com/licenses/mit/) and [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/).

- You can choose either of the licenses for redistribution, but you must retain the license files.
