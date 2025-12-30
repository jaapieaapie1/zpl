# device.user_vars.create



This command creates a user variable with the specified name, type, range, and default value. The root
SGD location for user variables is `"device.user_vars"` .


**Setvar**


To instruct the printer to create a user variable with the specified parameters:

```
       ! U1 setvar "device.user_vars.create" "name:type:range:defaultValue"

```

**Values**

              - `name` is the name of the SGD to appear in `device.user_vars` . The name can be anything
from 1 to 64 printable ASCII characters. Any '.' within the name will be replaced with '_'.
(e.g. `"john.doe"` will be changed to `"john_doe”` . The name must be unique in the
`device.user_vars` branch or it will not be created. The name will be converted to lower
case.

              - `type` is one of STRING, INTEGER, DOUBLE, CHOICES, UPDOWNCHOICES, UPDOWNINTEGER,
UPDOWNDOUBLE. The type must be one of these types or the variable will not be created.


                - STRING - For strings, the range indicates the min/max length of the data that can be stored.
If the range is left blank, the default range is a string length of 0-1024. There is no maximum
string length, however, if large data is placed into the variables the user should be aware
that system memory and performance will be affected. Strings larger than available system
memory will not be stored. Values should attempt to stay around, or below, 5K.


                - INTEGER/UPDOWNINTEGER - For integers the range can be any number expressed by a 32bit integer, signed or unsigned. If the range is left blank then a range of -32768 to 32767 will
be used.


                - DOUBLE/UPDOWNDOUBLE - A double can be any value within the range of +/–1.7e308. If
the range is left blank then a range of -32768.0 to 32767.0 will be used.


                - CHOICES/UPDOWNCHOICES - Choices must be specified in a comma delimited list. The
range cannot be blank if the type is CHOICES or UPDOWNCHOICES.

              - `range` is expressed as x-y. The range of a variable depends upon the type. Some types will
create default ranges, while others will fail to be created if the range is invalid or not specified.

              - `default` is the default value for the variable. The value must fall within the range specified
or the variable will not be created. If the type is INTEGER, UPDOWNINTEGER, DOUBLE,
UPDOWNDOUBLE the default value will be 0 if not specified. For STRING the default value will
be an empty string if it is not specified. CHOICES and UPDOWNCHOICES must have a default
value and it must be one of the choices within the specified range.


All four parts of the creation string must be present (some can be empty) meaning that there
must be three delimiter characters (':') present. There is no error shown or indicated otherwise
when the variable is not created for some reason. If the variable is not created one of the rules
listed above has been violated.


Any user variables will be deleted from the device.user_vars branch on a power cycle (they
won't be recreated on the next power up).


Defaulting the user_vars branch will restore the consumers back to their defaulted values and
will not remove them from the user_vars branch.


787


SGD Printer Commands


**Example**

To create a user variable named `userVar1` that is an integer with a minimum of 1, a maximum of 10, and a
default/initial value of 5, issue this command:

```
! U1 setvar "device.user_vars.create" "userVar1:INTEGER:1-10:5"

```

After issuing the above “create” command the `device.user_vars.userVar1` SGD will be present in an
ALLCV response.

After issuing the above “create” command the `device.user_vars.userVar1` SGD may be set via:

```
! U1 setvar "device.user_vars.userVar1" "2"

```

After issuing the above “create” command the `device.user_vars.userVar1` SGD may be retrieved
via:

```
! U1 getvar "device.user_vars.userVar1"

```

788


SGD Printer Commands