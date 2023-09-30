# Zata file format
Zata is a custom configuration file format that you should use to define new configuration styles for your files.

## Records
A record in Zata can be defined in the following way:

    key val

Field `val` can be empty. 

## Sections
Sections are defined with the following syntax:

    > name

Like Python and YAML Zata uses indentation (two leading spaces) to determine which items are in the current section.
For example in the following file

    > Sect1
      key1 val1
      key2 val2
      > Sect2
        key3 val 3
    > Sect3
      key4 val 4

section `Sect1`, `Sect3` are in the rott section whereas `Sect2` is in `Sect1`.
