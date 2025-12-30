# ^FB




ZPL Commands


The `^FB` command allows you to print text into a defined **block type** format. This command formats an `^FD`
or `^SN` string into a block of text using the origin, font, and rotation specified for the text string. The `^FB`
command also contains an automatic word-wrap function.


**Field Block**

**Format:** `^FBa,b,c,d,e`












|Parameters|Details|
|---|---|
|`a =` width of text block<br>line (in dots)|**Values:**<br>`0` to the width of the label<br>**Default:** `0` If the value is less than the font width or not specified, the text<br>does not print.|
|`b =` maximum number of<br>lines in the text block|**Values:** `1` to`9999`<br>**Default:** `1` Text exceeding the maximum number of lines overwrites the last<br>line. Changing the font size automatically increases or decreases the size of<br>the block.|
|`c =` add or delete space<br>between lines (in dots)|**Values:** `-9999` to`9999`<br>**Default:** `0` Numbers are considered to be positive unless preceded by a<br>minus sign. Positive values add space; negative values delete space.|
|`d =` text justification|**Values:**<br>`L =` left<br>`C =` center<br>`R =` right<br>`J =` justified<br>**Default:** `L` If`J` is used, the last line is left-justified.|
|`e =` hanging indent (in<br>dots) of the second and<br>remaining lines|**Values:** `0` to`9999`**Default:** `0`|



**Example:** These are examples of how the `^FB` command affects field data.


186


ZPL Commands


**Comments:**


This scheme can be used to facilitate special functions:

`\& =` carriage return/line feed

`\(*) =` soft hyphen (word break with a dash)

`\\ =` backslash (\)

**Item 1:** `^CI13` must be selected to print a backslash (\).

**Item 2:** If a soft hyphen escape sequence is placed near the end of a line, the hyphen is printed. If it is not
placed near the end of the line, it is ignored.


(*) = any alphanumeric character


- If a word is too long to print on one line by itself (and no soft hyphen is specified), a hyphen is
automatically placed in the word at the right edge of the block. The remainder of the word is on the next
line. The position of the hyphen depends on word length, not a syllable boundary. Use a soft hyphen
within a word to control where the hyphenation occurs.


- Maximum data-string length is 3K, including control characters, carriage returns, and line feeds.


- Normal carriage returns, line feeds, and **word spaces** at line breaks are discarded.

- When using `^FT` (Field Typeset), `^FT` uses the baseline origin of the last possible line of text. Increasing
the font size causes the text block to increase in size from bottom to top. This could cause a label to
print past its top margin.

- When using `^FO` (Field Origin), increasing the font size causes the text block to increase in size from top
to bottom.

- `^FS` terminates an `^FB` command. Each block requires its own `^FB` command.


While the `^FB` command has a text justification parameter that defines the justification of the text within the
block, it also interacts with the justification of `^FO` and `^FT` that define the justification of the origin.

The `^FB` command does not support soft hyphens as a potential line breakpoint. However, soft hyphen
characters are always printed as if they were a hyphen.


187


ZPL Commands


The `^FB` command does not support complex text. For complex text support, use `^TB` .


188