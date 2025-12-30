# power.battery_type



This command retrieves the battery type installed in the printer.


**Getvar**


To get the type of battery installed in the printer:

```
       ! U1 getvar "power.battery_type"

```

**Default**
```
          "unmanaged"
```

**Result**

`"sb"` smart battery

`"ppp"` power precision plus

`"none"` no battery

`"unmanaged"` legacy unmanaged battery


**Example**

In the example below, the `getvar` retrieves the battery type installed in the printer.

```
       ! U1 getvar "power.battery_type" "sb"

```

**NOTE:** QLn and ZQ5 are not capable of authenticating a Power Precision Plus battery so these
printers will report `"sb"` .


945


SGD Printer Commands