# weblink.logging.clear



This command clears the weblink log. Setting this value to anything will clear it, including an empty string.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To clear the weblink log entries:

```
       ! U1 setvar "weblink.logging.clear" "value"

```

**Values**


Any string value, including an empty string.


**Default**
NA


**Do**


To clear the weblink log entries:

```
       ! U1 do "weblink.logging.clear" "value"

```

**Values**


Any string value, including an empty string.


**Default**
NA


**Example**


This example clears the weblink log entries with an empty string value.

```
       ! U1 setvar "weblink.logging.clear" ""

```

1348


SGD Network Commands