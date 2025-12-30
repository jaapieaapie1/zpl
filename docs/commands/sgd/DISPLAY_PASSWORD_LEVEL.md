# display.password.level



Controls when to display the password WML card on the LCD display.


**Setvar**


To control when to display the password WML card on the LCD display:

```
       ! U1 setvar "display.password.level" "value"

```

**Values**

              - `"all"` The user will always be prompted to enter a password when a field is to be modified.

              - `"none"` The user will not be prompted to enter a password on the LCD display.

              - `"selected"` The user will be prompted to enter a password only if the WML card contains a
password ="on" attribute and the user attempts to change a setting.


**Default**

              - `"selected"` QLn420, QLn320 Healthcare, and QLn220 Healthcare

              - `"none"` all other platforms


**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "display.password.level"

```

803


SGD Printer Commands