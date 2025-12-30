# DATE




ZBI Commands


This function gets the current date as a number.


**Format**
```
   DATE
```

**Parameters**
N/A

**Returns**
This function returns the current date in `YYYYDDD` format, where `YYYY` is the year and `DDD` is
the number of days since the beginning of the year. If the Real-Time Clock is not installed, `0` is
returned.

**Example**
This example assumes the current date is January 1, 2003:

```
   10 PRINT DATE
   RUN
   2003001

```

581