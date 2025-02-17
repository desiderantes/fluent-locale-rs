# Changelog

## Unreleased

  - …

## fluent-locale 0.10.0 (October 3, 2019)

  - Update to `unic-langid` 0.6.

## fluent-locale 0.9.0 (October 1, 2019)

  - Use AsRef as bounds in negotiation.
  - Support unic-langid with full CLDR backed likelysubtags behind "cldr" feature.

## fluent-locale 0.8.0 (September 10, 2019)

  - Update to `unic-langid` 0.5.

## fluent-locale 0.7.0 (July 30, 2019)

  - Update `unic-langid` to 0.4.
  - Switch benchmark to criterion.
  - Update helper functions to be more generic.

## fluent-locale 0.6.0 (July 24, 2019)

  - Switch to use `unic-langid` (but allow for `unic-locale`).
  - Refactor the API to handle fallible lists.

## fluent-locale 0.5.0 (June 16, 2019)

  - Separate out `unic-langid` and `unic-locale` into new crates.
  - Switch from BCP47 conformance to Unicode Locale Identifier.
  - Update to Rust 2018.

## fluent-locale 0.4.1 (August 6, 2018)

  - Separate out requested from available to allow for different mixes of Vec and &[].

## fluent-locale 0.4.0 (August 6, 2018)

  - Ergonomics improvement - `negotiate_languages` now accepts &[&str], &[String], Vec<&str> and Vec<string>

## fluent-locale 0.3.2 (July 31, 2018)

  - Make Locale::matches reject matches if privateuse is not empty

## fluent-locale 0.3.1 (February 12, 2018)

  - Make fluent-locale compliant with rust stable (from 1.23)

