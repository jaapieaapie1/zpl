# device.serial_numbers.wired_ethernet_option_board_date



This command retrieves the Ethernet option board date.


For printers that do not store this value, the printer returns an empty string.


**Getvar**


To return the current setting:

```
       ! U1 getvar "device.serial_numbers.ethernet_option_board_date"

```

**Example**

In this example, the `getvar` returns the current date of the Ethernet option board.

```
       ! U1 getvar "device.serial_numbers.ethernet_option_board_date"
       "12/31/2014"

```

756


SGD Printer Commands