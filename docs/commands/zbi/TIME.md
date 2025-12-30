# TIME




ZBI Commands


This function gets the current time as a number.


**Format**
```
   TIME
```

**Parameters**
N/A

**Returns**
This function returns the time past midnight (2400h) in seconds. If the Real-Time Clock is not
installed, 0 is returned.

**Example**
This is an example of how to use the TIME command [assuming the time is one minute past
midnight]:

```
   10 PRINT TIME
   RUN
   60

```

582


ZBI Commands