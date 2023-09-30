# Configuration files
## Overview
A _configuration_ is a list of _records_ contained in _sections_. Each record is characterized by a _key_ and a _value_ which are both UTF-8 encoded strings. 
Keys should contain only alphanumeric characters (`A-Z`, `a-z`, `0-9`) and undescores (`_`), but different configuration formats may allow other characters.
Keys can't be empty, instead values might be empty and can potentially contain any valid Unicode character, however the chosen configuration style usually imposes different limitations.

Every record should be declared inside a _section_, which is just a named container of records and other (sub)sections. 
Section names should follow the same rules of record keys. 
Inside a section can't coexist multiple records with the same key and multiple subsections with the same name.
Every configuration file has a unique _root section_ that should not be defined explicitly but automatically contains any
record or section not defined in any other section.

Therefore, a configuration file can always be represented as a _tree_ where the leaves are records (or empty sections), 
nodes that are not leaves are sections and the root node is the root section.

## Configuration file formats
When you want to write a configuration file for your project you should first choose a valid _configuration file format_ in order 
to serialize a valid configuration into a file. There are several different formats you can use, here we'll list some of them:

### Windows .INI / Linux .desktop / Systemd units
This is not an unique configuration file formats, because every application added new limitation or features from the original format represented by the classic MS-DOS/Windows `.INI` files. Actually we can determine some common rules which are shared between these styles.

Each section and record specification should lie on their own line, however many formats allows to split a record in multiple lines by using escape characters.

Sections are defined between two square bracket `[`, `]` enclosing the section name, and any record immediately following a section specification are inserted in that section. A new section declaration will start a new section, therefore you can't define new sections inside other section that are not the root one (which can't be explicitly declared).

Records can be defined with the following syntax:

    key=value

where `key` is the key and `value` is the value of the record. Leading and trailing spaces (including tabs) in `key` and `value` are automatically trimmed, therefore you can add spaces and tabs to improve readability. Usually `value` can't contain the equal sign `=` unless `value` it's entirely enclosed by quotation marks `"`.

Records declared before any section definition are automatically added to the root section.

For example, these records are always equivalent in any configuration

    key=val
    key = val
        key    =       val

instead the following one may or may not be equivalent to the preceding ones

    key = "val"

whereas this is definitively not equivalent

    key = " val "

#### Example

    [Section1]
    key1 = val1
    key2  =  val2

    [Section2]

    key1 = val 3
    key3 = " val = 4 "

Here the root section contains sections `Section1` and `Section2`. Moreover, `Section1` contains the records (`key1`, `val1`), (`key2`, `val2`) and `Section2` contains (`key1`, `val 3`), (`key3`, `" val = 4 "`).
Notice that the two records with key `key1` are not incompatible since they belong to different sections, moreover only leading and trailing spaces of ` val 3` are trimmed, thus the effective value is `val 3` and not `val3`.

Record (`key3`, `" val = 4 "`) should be accepted by everywhere even if the value field contains the equal sign `=` since it is enclosed by quotation marks.

## JSON
The JSON format was introduce by JavaScript to serialize objects and other entities, but now ithas been adopted by many desktop applications and used to store configuration information.

Here we analyze only a tiny subset of the JSON language that can be used to write configuration files.

All the content of a JSON configuration file should be enclosed between a pair of curly bracket `{`, `}` representing the root section. A section can be defined in the following way:

    "name " : {
        ...
      },

(notice the final comma) where in place of `...` you can put the contained records and subsections. Records are defined as

    "key" : json_ent,

where `json_ent` can be one of the following:

- a string enclosed by quotation marks:

      "key" : "val",
  
- a number:

      "key" : num,

- an array (comma-list) of `json_ent`:

      "key" : [ json_ent1, json_ent2, ..., json_entN ],

#### Example

    {
      "Section1" : {
        "key1" : "val1",
        "key2" : 5,
        "Section1b" : {
          "key2" : "IO",
        },
      },
      
      "Section2" : {
        "key3" : " ASDF ",
      },
    }

Notice that inside section `Section1` we have defined the subsection `Section1b`.
