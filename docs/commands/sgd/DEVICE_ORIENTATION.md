# device.orientation



This printer setting determines the installation orientation of the KR403 printer, either horizontal or vertical.
It is intended for use only by the system integrator. Modification by an end user can result in unexpected
printer behaviour.


**Setvar**


To instruct the printer to change the presenter loop length:

```
       ! U1 setvar "device.orientation" "value"

```

**Values**

              - `"0"` printer is installed horizontally

              - `"1"` is installed vertically

**Default**

`"0"` Printer is installed horizontally (original factory default only, value will not change when
defaulting the printer with `^JUF` ).


**Getvar**


To instruct the printer to respond with the currently set presenter loop length:

```
       ! U1 getvar "device.orientation"

```

724


SGD Printer Commands