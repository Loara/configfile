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
