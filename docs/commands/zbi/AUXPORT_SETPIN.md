# AUXPORT_SETPIN



This function sets the output level on an applicator pin.


**Format**
AUXPORT_SETPIN `(X,Y)`

**Parameters**
`X` = perform action on this applicator port pin.

`Y` = The value to set on the pin (1 = high, 0 = low).

**Returns**
This function returns -1 upon failure and 0 upon success.

**Example**


See AUXPORT_STEALPIN on page 538.


**Comments**
If this pin is not controlled via ZBI (power pin), this function will return -1. See AUXPORT_STEALPIN
on page 538.


539


ZBI Commands