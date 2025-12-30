# ~JA




ZPL Commands


The `~JA` command cancels all format commands in the buffer. It also cancels any batches that are printing.


**Cancel All**


The printer stops after the current label is finished printing. All internal buffers are cleared of data and the
**DATA** LED turn off.

Submitting this command to the printer scans the buffer and deletes only the data before the `~JA` in the
input buffer — it does not scan the remainder of the buffer for additional `~JA` commands.

**Format:** `~JA`


251