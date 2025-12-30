# capture.channel1.delimiter



This command stores the delimiter used to partition data received on the port specified
by `capture.channel1.port` and stored in `capture.channel1.data.raw` and
`capture.channel1.data.mime` .

This will be reported in the data capture section of the `HZA` response.


**Setvar**

To set the delimiter used to partition data received on the `capture.channel1.port` :

```
       ! U1 setvar "capture.channel1.delimiter" "delimiter"

```

**Values**


Any character set up to a maximum of 64 characters in length.


**NOTE:** Binary data can be used in the delimiter. To do this enter a '\' and then the 3 digit
octal value of the character. `"\\"` = `'\'` in some tools, so to get `\002` you may need to
enter `“\\002”` . Escaped octal characters count as a single character and not 4 (e.g. a
delimiter of `“\001\000\002”` is 3 characters, not 12)

**Default**
```
          "\012"

```

**Getvar**


To retrieve the delimiter:

```
       ! U1 getvar "capture.channel1.delimiter"

```

**Example**

Binary data can be used in the delimiter. To do this enter a `'\'` and then the 3 digit octal value of the
character. Note: `"\\"` = `'\'` in some tools, so to get `\002` you may need to enter `"\\002"` .

```
       "\000" = NULL (single character)
       "end\015\012\000" = 'e'+'n'+'d'+'\r'+'\n'+ NULL (total of 6 characters

```

645


SGD Printer Commands