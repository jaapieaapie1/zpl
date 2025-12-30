# DATE$




ZBI Commands


This function returns the date as a string.


**Format**
```
   DATE$
```

**Parameters**
N/A

**Returns**
The current date in string form YYYYMMDD. If the Real-Time Clock is not installed, an empty string
is returned.

**Example**
This is an example of how to use the DATE$ command:

```
   10 PRINT DATE$
   RUN

```

The result, assuming the date is January 1, 2003 is:

```
   20030101

```

**Example**
This is another example of the DATE$ command used with the sub-string operator to get the day of
the month:

```
   10 LET A$=DATE$(7:8)
   20 IF A$ <> DATE$(7:8)
   30 LET A$=DATE$(7:8)
   40 IF A$="01"
   50 PRINT "IT IS THE FIRST OF THE MONTH"
   60 END IF
   70 END IF
   80 SLEEP 100
   90 GOTO 20

```

**Comments**
None


579