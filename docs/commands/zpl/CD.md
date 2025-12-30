# ^CD ~CD




ZPL Commands


The `^CD` and `~CD` commands are used to change the delimiter character. This character is used to
separate parameter values associated with several ZPL II commands. The default delimiter is a comma (,).


**Change Delimiter**

**Format:** `^CDa or ~CDa`






|Parameters|Details|
|---|---|
|`a =` delimiter character<br>change|**Values:** any ASCII character<br>**Default:** a parameter is required. If a parameter is not entered, the next<br>character received is the new prefix character.|



**Example:** This shows how to change the character delimiter to a semi-colon ( `;)` :

```
^XA
^FO10,10
^GB10,10,3
^XZ
^XA
^CD;
^FO10;10
^GB10;10;3
^XZ

```

To save, the `JUS` command is required. Here is an example using `JUS` :

```
~CD;
^XA^JUS^XZ

```

153