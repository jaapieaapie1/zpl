# AUXPORT_STEALPIN



This function will take control of a pin and allow ZBI to perform other actions on the pin.


**Format**
AUXPORT_STEALPIN `(X)`

**Parameters**
`X` = perform action on this applicator port pin.

**Returns**
This function returns -1 upon failure and 0 upon success.

**Example**
This is an example of the AUXPORT_STEALPIN command:

```
          1 REM Demo applicator to show control of applicator pins
          1 REM on the printer
          1 REM The application is to create a light pole with an
          1 REM external feed button
          AUTONUM 1,1
          LET RED = 9
          LET YELLOW = 10
          LET GREEN = 11
          LET BUTTON = 4
          LET FEED_KEY = 3
          LET TMP = AUXPORT_STEALPIN(RED)
          LET TMP = AUXPORT_STEALPIN(YELLOW)
          LET TMP = AUXPORT_STEALPIN(GREEN)
          LET TMP = AUXPORT_STEALPIN(BUTTON)
          DO WHILE 1 = 1
          SLEEP 1
          IF ISERROR = 1 THEN
          LET TMP = AUXPORT_SETPIN(RED,1)
          LET TMP = AUXPORT_SETPIN(YELLOW,0)
          LET TMP = AUXPORT_SETPIN(GREEN,0)
          ELSE IF ISWARNING = 1 THEN
          LET TMP = AUXPORT_SETPIN(RED,0)
          LET TMP = AUXPORT_SETPIN(YELLOW,1)
          LET TMP = AUXPORT_SETPIN(GREEN,0)
          ELSE
          LET TMP = AUXPORT_SETPIN(RED,0)
          LET TMP = AUXPORT_SETPIN(YELLOW,0)
          LET TMP = AUXPORT_SETPIN(GREEN,1)
          END IF
          IF AUXPORT_GETPIN(BUTTON) = 1 THEN
          LET A = TRIGGEREVENT(FEED_KEY)
          END IF
          LOOP

```

**Comments**
If this pin is not controlled via ZBI (power pin), this function will return -1.


538


ZBI Commands