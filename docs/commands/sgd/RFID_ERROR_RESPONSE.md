# rfid.error.response



This command can be used to retrieve the RFID status, including any error codes or messages.


**Getvar**


To retrieve any active RFID error messages:

```
       ! U1 getvar "rfid.error.response"

```

**Example**

This `getvar` example shows responses that you may get in different situations:

```
       ! U1 getvar "rfid.error.response"

```

If no RFID tag is present, you get the following response:

```
       "NO TAG FOUND"

```

If an RFID tag is present and there are no errors, you get the following response:

```
       "RFID OK"

```

1512


SGD RFID Commands