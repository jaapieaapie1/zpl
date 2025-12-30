# odometer.label_dot_length



This command returns the length of the last label printed or fed (in dots).


**Getvar**


To return the length of the last label printed or fed (in dots):

```
       ! U1 getvar "odometer.label_dot_length"

```

**Example**

This is an example of how to reset the length using the `^LL` command and how to use the `getvar` to
confirm the change. For the `^LL` command to work, the printer must be in continuous mode.

To change the odometer label dot length, type:

```
       ^XA
       ^LL500
       ^XZ

```

To get the current odometer label dot length, type:

```
       ! U1 getvar "odometer.label_dot_length"

```

Something similar to this is shown:

```
       "500"

```

920




SGD Printer Commands