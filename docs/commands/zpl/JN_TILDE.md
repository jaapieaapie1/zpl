# ~JN




ZPL Commands


The `~JN` command turns on the head test option. When activated, `~JN` causes the printer to halt when a
head test failure is encountered.


**Head Test Fatal**

Once an error is encountered the printer remains in error mode until the head test is turned off ( `~JO` ) or
power is cycled.

**Format:** `~JN`

**Comments:** If the communications buffer is full, the printer is not able to receive data. In this condition, the
`~JO` command is not received by the printer.


270