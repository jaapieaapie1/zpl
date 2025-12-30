# ^MD




ZPL Commands


The `^MD` command adjusts the darkness relative to the current darkness setting.


**Media Darkness**

**Format:** `^MDa`

|Parameters|Details|
|---|---|
|`a =` media darkness level|**Values:** `-30` to`30`, depending on current value<br>**Initial Value at Power Up:** `0` If no value is entered, this command is<br>ignored.|



**Example:** These examples show setting the printer to different darkness levels:

- If the current value (value on configuration label) is 16, entering the command `^MD-9` decreases the
value to 7.

- If the current value (value on configuration label) is 1, entering the command `^MD15` increases the value
to 16.

- If the current value (value on configuration label) is 25, entering the command `^MD10` increases only the
value to 30, which is the maximum value allowed.

Each `^MD` command is treated separately in relation to the current value as printed on the configuration
label.


**NOTE:** On Zebra G-Series™ printers the value set with the `^MD` command is persistent across
power cycles.


**IMPORTANT:** The darkness setting range for the **&XiIIIPlus;**, Xi4, and RXi4 is 0 to 30 in
increments of 0.1. The firmware is setup so that the ^MD and ~SD commands (ZPL darkness
commands) accepts that range of settings.


**Example:** These are examples of the **XiIIIPlus**, Xi4, and RXi4 Darkness Setting:
```
^MD8.3
~SD8.3
```

**Example:** For example, this is what would happen if two `^MD` commands were received:

Assume the current value is 15. An `^MD-6` command is received that changes the current value to 9.
Another command, `^MD2`, is received. The current value changes to 17.

The two `^MD` commands are treated individually in relation to the current value of 15.

**Comments:** The `~SD` command value, if applicable, is added to the `^MD` command.


301