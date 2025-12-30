# TIME$




ZBI Commands


This function returns the current time in a string.


**Format**
```
   TIME$
```

**Parameters**
N/A

**Returns**
This function returns the time of day in format HH:MM:SS (hours:minutes:seconds). If the Real-Time
Clock is not installed, an empty string is returned.

**Example**
This is an example of how to use the TIME$ command:

```
   10 PRINT TIME$
   RUN
   10:00:00

```

580