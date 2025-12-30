# ~JO




ZPL Commands


The `~JO` command configures the printer to run the head test with error reporting enabled. When `~JO` is
used an error will be displayed and printing will stop if the head test fails. The user can push the PAUSE
button on the printer to bypass the error. This command differs from the `~JN` (Head Test Fatal) command in
that a power cycle is not required in the event of a head test failure.


**Head Test Non-Fatal**

`~JO` is the default print head test condition. This setting is changed when the printer receives a `~JN` (Head
Test Fatal) command.

**Format:** `~JO`


271