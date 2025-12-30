# EXTRACT$



This function searches for a string based on a starting and ending string. When these two strings are found,
the string between them is returned.


**NOTE:** If the EXTRACT$ command encounters a carriage return line feed before encountering
the beginning character or the ending character, it returns null.

**Format**
```
          EXTRACT$ (CHANNEL, START$, STOP$)
          EXTRACT$ (A$, START$, STOP$)
```

**Parameters**

              - `CHANNEL` = extracts data from this channel

              - `A$` = the source string

              - `START$` = Once this string is found, the extract pulls characters immediately following.

              - `STOP$` = the extraction stops when this string is found

**Example**


This example shows how to extract the word Technologies from this string:
Zebra,Technologies,Corporation.


This is what the program looks like to accomplish this:

```
          10 LET A$ = "Zebra,Technologies,Corporation,"
          20 LET DATA$ = EXTRACT$(A$,",",",")

```

**Example**


This example shows how the EXTRACT$ command works from an open port:

```
          10 OPEN #1: NAME "SER"
          20 LET DATA$ = EXTRACT$(1,",",",")

```

Notice how the quotes are used to show a literal character, in this case a comma.


**Example**


This example shows how the start and stop points are variable; a variable name is used instead of
the literal:

```
          10 LET B$ = ","
          20 LET A$ = "Zebra,Technologies,Corporation"
          30 LET DATA$ = EXTRACT$(A$,B$,B$)
          40 PRINT DATA$
          RUN
          Technologies

```

554


ZBI Commands


**Example**


This example shows how an empty string can be used to extract from the start of the input string to
the end string:

```
 10 LET IN$ = "BLAH BLAH <END>"
 20 LET B$ = EXTRACT$(IN$, "", "<END>")
 30 PRINT B$
 RUN
 BLAH BLAH

```

**Example**


This example will use an empty string to extract to the end of a line:

```
 10 LET IN$ = "BLAH <START> THE DATA"
 20 LET B$ = EXTRACT$(IN$, "<START>", "")
 30 PRINT B$
 RUN
 THE DATA

```

**Comments**
EXTRACT$ reads in and discards data until the start criteria is met. Then, all data is returned up to
the stop criteria.


555