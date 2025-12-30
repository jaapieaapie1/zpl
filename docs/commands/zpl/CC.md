# ^CC ~CC




ZPL Commands


The `^CC` command is used to change the format command prefix. The default prefix is the caret ( `^` ).


**Change Caret**


**Format:** `^CCx` or `~CCx`






|Parameters|Details|
|---|---|
|`x =` caret character<br>change|**Values:** any ASCII character<br>**Default:** a parameter is required. If a parameter is not entered, the next<br>character received is the new prefix character.|



**Example:** This is an example of how to change the format prefix to / from a ::

```
^XA
^CC/
/XZ

```

The forward slash (/) is set at the new prefix. Note the `/XZ` ending tag uses the new designated prefix
character (/).

**Example:** This is an example of how to change the format prefix from `~` to a `/` :

```
~CC/
/XA/JUS/XZ

```

152