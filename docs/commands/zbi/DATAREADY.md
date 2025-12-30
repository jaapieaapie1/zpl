# DATAREADY



This function is used to determine if there is data received on a specified port.


**Format**
```
          DATAREADY (A)
```

**Parameters**
A = the port to check

**Returns**
1 if there is data, 0 if there is no data.

**Example**
This is an example of how to check if there is a data on a port:

```
          10 PRINT DATAREADY(0)
          RUN

```

The result, assuming no data is waiting, is:

```
          0

```

**Comments**
If this command follows the `INPUT` command, it may return 1 if the line received was ended with a
CRLF. In this case, `INBYTE` can be used to take the LF out of the buffer.


488


ZBI Commands