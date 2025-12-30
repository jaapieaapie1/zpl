# ^CT ~CT




ZPL Commands


The `^CT` and `~CT` commands are used to change the control command prefix. The default prefix is the tilde
(~).


**Change Tilde**


**Format:** `^CTa or ~CTa`






|Parameters|Details|
|---|---|
|`a =` change control<br>command character|**Values:** any ASCII character<br>**Default:** a parameter is required. If a parameter is not entered, the next<br>character received is the new control command character.|



**Example:** This is an example of how to change the control command prefix from a `^` to a `+` :

```
^XA
^CT+
^XZ
+HS

```

166