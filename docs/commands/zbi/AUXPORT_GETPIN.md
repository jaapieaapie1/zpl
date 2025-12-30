# AUXPORT_GETPIN



This function will retrieve the state of the applicator pin.


**Format**
AUXPORT_GETPIN `(X)`

**Parameters**
`X` = perform action on this applicator port pin.

**Returns**
This function returns 1 if pin is in high state, 0 in low state, and -1 upon failure.

**Example**


See AUXPORT_STEALPIN on page 538.


**Comments**
If this pin is not controlled via ZBI (power pin), this function will return -1. See AUXPORT_STEALPIN
on page 538.


540




ZBI Commands