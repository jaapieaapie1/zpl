# comm.mode



This command selects which serial interface to use. Other than RS232, the other options are used only with
an external dongle.


**Getvar**


To report the serial interface currently in use:

```
       ! U1 getvar “comm.mode”

```

**Values**

              - `"rs232"`

              - `"rs422_rs485"`

              - `"rs485_multidrop"`

**Default**
```
          "rs232"

```

**Setvar**


To specify the serial interface to be used:

```
       ! U1 setvar “comm.mode” "value"

```

**Values**


              - For ZE5X1

                - `"rs232"`

                - `"rs422_rs485"`

                - `"rs485_multidrop"`

              - For other printers

                - `"rs232"`


652


SGD Printer Commands