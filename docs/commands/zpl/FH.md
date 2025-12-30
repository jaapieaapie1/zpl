# ^FH




ZPL Commands


The `^FH` command allows you to enter the hexadecimal value for any character directly into the `^FD`
statement. The `^FH` command must precede each `^FD` command that uses hexadecimal in its field.


**Field Hexadecimal Indicator**

Within the `^FD` statement, the hexadecimal indicator must precede each hexadecimal value. The default
hexadecimal indicator is _ (underscore). There must be a minimum of two characters designated to follow
the underscore. The `a` parameter can be added when a different hexadecimal indicator is needed.

This command can be used with any of the commands that have field data (that is `^FD`, `^FV` (Field
Variable), and `^SN` (Serialized Data)).

Valid hexadecimal characters are:
```
0 1 2 3 4 5 6 7 8 9 A B C D E F a b c d e f
```

**Format:** `^FHa`






|Parameters|Details|
|---|---|
|`a =` hexadecimal<br>indicator|**Values:** any character except current format and control prefix (^ and ~ by<br>default)<br>**Default:** _ (underscore)|



**Example:** This is an example of how to enter a hexadecimal value directly into a `^FD` statement: This is an
example of ASCII data using `^CI0` .


**Example:** These are examples of how `^FH` works with UTF-8 and UTF-16BE:

- UTF-8


193


- UTF-16BE



ZPL Commands


194