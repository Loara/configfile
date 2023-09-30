# Style file syntax
## ROOT section

### FORMAT record
Can contain one of the following values:
- `zata` the Zata configuration file format

## RECORDS and ROOT_RECORDS section
ROOT_RECORDS defines records in root section, RECORDS instead in any other record. RECORDS is __floating__.
### custom sections
Each section here represents a record specification.

#### USEONLY_RECORD record
Its `value` should be a comma list with at least one element:

    record, val1, val2, ...

- `record` is the address of a record (respect to the current section or the root section)
- `val1`, `val2`, ... are strings

If only `record` is specified then you can use this record only if `record` is present. If at least one of `val1`, `val2`, ... is specified then `record`
should be also equal to at least one of `val1`, `val2`, ...

This record can be __multiple__: multiple declaration provide additional restrictions.

#### STYLE record
Can be one of the following:

- `raw`: an arbitrary string terminated by a newline character;
- `num`: a positive integer;
- `comma`: a comma separated list of strings;
- `choice`: the record can assume only a finite number of values listed in LIST record;
- `flag`: the value should be always empty, used to define flag records.

#### LIST record
A comma separated list of possible values a `choice` record can assume.

#### REQUIRED and OPTIONAL flag records
A REQUIRED record should be always specified in a configuration file, Optional may be omitted.

#### MULTIPLE flag records
MULTIPLE records instead can be defined multiple times.

## SECTIONS and ROOT_SECTIONS sections
ROOT_SECTIONS defines sections in root sections, SECTIONS in any other section. SECTIONS is __floating__.

### INCLUDE record
Each element in the comma list is a FLOATING section that can the user can define inside the specified section.
### custom section
Each custom section defines a section. This section can contain a RECORDS and/or a SECTIONS section 
to define records and subsections.

#### USEONLY_RECORD record
#### OPTIONAL flag record
#### MULTIPLE flag record
As before.

#### CUSTOM_SUBSECTIONS flag record
If this section is flagged with CUSTOM_SUBSECTIONS then an user can define any subsection inside it.
Each of these subsections will have the same attributes defined in CUSTOMS subsection
##### CUSTOMS section
Uses the same item of custom section. These properties will be inherited by any subsection defined by the user.

#### FLOATING records
Can be defined only for sections defined in ROOT_SECTIONS, not in SECTIONS. A FLOATING section can _only_ be used in any section that includes it with the INCLUDE record and not only the one in which it is defined.
