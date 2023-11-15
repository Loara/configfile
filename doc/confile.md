# Configuration files
## Overview
A _configuration_ is a collection of textual entries usually held in one or more text files in order to allow the user to customize the behaviour of an application/service by just modifying the configuration. There're two kind of entries in a configuration:
 - _flags_, which are characterized by a _name_;
 - _records_, which instead are characterized by a _key_ and a _value_.

Records in a configuration can be organized in _sections_. Every section has a _section name_ and contains zero or more entries or other sections. Notice that at this point _names_, _keys_, _values_ and _section names_ are arbitrary nonempty unescaped Unicode strings, however many backends usually introduce encoding assumptions (usually UTF-8) and character limitations in names and/or values.

In a configuration every entry/section should be contained in a _parent section_, the only section that is not contained in any other section is the _root_ section. The root section doesn't have a section name, can't be directly accessed and in configuration files it's never specified.

Inside a section flags are identified by their names, therefore you can't have two or more flags with the same name in the same section. However, records aren't identified by their key/value, so in a single section you can have multiple records with the same key and/or value. When you want to retrieve a record by key from a section you usually won't get a single value, buy an _ordered_ list of zero or more values, each of them associated to a record in that section with the specified key.

A configuration can always be represented as a _tree_ where the leaves are entries (or empty sections), 
nodes that are not leaves are sections and the root node is the root section.

## Configuration file formats
When you want to write a configuration to a file for your project you should first choose a valid _configuration file format_ in order 
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

## XML
Even XML files can be seen as configuration files, for example by identifying sections with tags:

    <section flag1 key1="value1" key2="value2" flag3 ... />

and subsections with child tags:

    <parent_section ... >
        <subsection1 />
        <subsection2 flag1>
            <subsubsection1 key1="value1" />
        </subsection2>
        ...
    </parent_section>

and so on.
    
