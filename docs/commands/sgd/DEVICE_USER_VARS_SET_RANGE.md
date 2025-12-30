# device.user_vars.set_range



This command compliments the `device.user_vars.create` command, allowing a user to change the
range of a user-created SGD variable. It has a similar syntax to `device.user_vars.create` with the
exception that no default is specified.


**Setvar**


To change the range of a user-created variable:

```
       ! U1 setvar "device.user_vars.set_range" "name:type:range"

```

**Values**

              - `name` identifies the name of the SGD to modify

              - `type` must be the same type for `name` as when it was created

              - `range` is x-y (for all but UPDOWNCHOICES and CHOICES) or a,b,c,d (for CHOICES and
UPDOWNCHOICES)


If no range is specified then it will delete whatever range is currently specified.


**Default**
NA


**Example**

This example modifies my_var to: `device.user_vars.my_var : b, Choices: a,b,c,d,e`


786


SGD Printer Commands