# ~DN




ZPL Commands


After decoding and printing the number of bytes in parameter `t` of the `~DG` command, the printer returns to
normal Print Mode. Graphics Mode can be aborted and normal printer operation resumed by using the `~DN`
command.


**Abort Download Graphic**

**Format:** `~DN`

**Comments:** If you need to stop a graphic from downloading, you should abort the transmission from the
host device. To clear the `~DG` command, however, you must send a `~DN` command.


177