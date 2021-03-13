# Tidy

Tidy renames files matching a source pattern to a new name matching the target
pattern. The purpose is to homogenize the filenames of documents from various
sources.

Example:

The command

```bash
tidy -s 20030201_%0_%1 -t 2003-03-01_%1_%0 -d .
```

renames files such as _202011015_foo_bar.txt_ to _2020-11-15_bar_foo.txt_ in the
current folder and all its subfolders.

Type `tidy --help` to see all options.


## Patterns

Patterns are formed of identifiers and literals. Identifiers are dates (year,
month, day) and fragments (%0, %1, ..). A valid pattern must contain at least
one identifier or literal. Source patterns must contain at least all the
identifiers that occur in the target pattern. In other words, the identifiers of
the target pattern are a subset of the identifiers in the source pattern.

### Dates

Date patterns always refer to the same reference date: __Feb 01, 2003__. Tidy
supports various date formats which can be expressed by writing the reference
date in different ways.

Supported formats:

- year: 2003, 03
- month: 02, FEB, feb, FEBRUARY, february
- day: 01

Examples:

| format | example |
|:------------------|:------------|
| 01.02.2003 | 04.05.2020 |
| feb_01_2003 | nov_06_2020 |
| 2003-FEBRUARY-01 | 2018-MAY-07 |
| 2003-february-01 | 2018-may-07 |

### Fragments

Tidy allows to assign parts of a filename to variables which can then be moved
around in the target pattern. For example, given a filename `20150903_foo-bar`
and the pattern `20030201_%0-%1`, `%0` corresponds to "foo" and `%1`corresponds
to "bar". A target pattern might, e.g., reverse the order `20030201_%1-%0`.

Fragments have to be numbered continiously starting at 0.

### Literals

Literals help to delineate date patterns and fragments. Literals in the source
formats have to match the filename otherwise processing will return an error.

In the format `20030201_%0-%1`, `_` and `-` are both literals.

## Limitations

Names of months are all in English. However it is easy to add support for more
lanugages.

## Install

Tidy can currently only be installed from source.
