# AUXPORT_RELEASEPIN



This function returns a pin controlled by ZBI to normal printer operation.


**Format**
AUXPORT_RELEASEPIN `(X)`

**Parameters**
`X` = perform action on this applicator port pin.

**Returns**
This function returns -1 upon failure and 0 upon success.

**Example**
This is an example of the AUXPORT_RELEASEPIN command:

```
          90 LET TMP = AUXPORT_RELEASEPIN(X)

```

**Comments**
If this pin is not controlled via ZBI (power pin), this function will return -1. See AUXPORT_STEALPIN
on page 538.


541


ZBI Commands