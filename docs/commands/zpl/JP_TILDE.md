# ~JP




ZPL Commands


The `~JP` command clears the format currently being processed and places the printer into Pause Mode.


**Pause and Cancel Format**


The command clears the next format that would print, or the oldest format from the buffer. Each
subsequent `~JP` command clears the next buffered format until the buffer is empty. The **DATA** indicator
turns off when the buffer is empty and no data is being transmitted.

Issuing the `~JP` command is identical to using **CANCEL** on the printer, but the printer does not have to be
in Pause Mode first.

**Format:** `~JP`


272