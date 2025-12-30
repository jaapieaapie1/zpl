# ~JB




ZPL Commands


**Reset Optional Memory**

The `~JB` command is used for these conditions:

- The `~JB` command must be sent to the printer if the battery supplying power to the battery powered
memory card fails and is replaced. A bad battery shows a battery dead condition on the Printer
Configuration Label.

- The `~JB` command can also be used to intentionally clear (reinitialize) the `B:` memory card. The card
must not be write protected.

**Format:** `~JB`

**Comments:** If the battery is replaced and this command is not sent to the printer, the memory card cannot
function.


253