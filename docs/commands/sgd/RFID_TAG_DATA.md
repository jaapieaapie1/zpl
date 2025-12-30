# rfid.tag.data



This command tells the RFID reader to attempt to read a tag over the RFID antenna, even if the printhead is
open. Results are returned to the host.


Before running this command, position an RFID label over the printer’s RFID antenna.


**Getvar**


To return the current tag’s data:

```
       ! U1 getvar "rfid.tag.data"

```

For more information about this option and for the location of the RFID antenna, refer to the RFID
[Programming Guide for your printer. A copy is available online at www.zebra.com/manuals.](http://www.zebra.com/manuals)


**Examples**


This example gets data from the current tag, assuming that an RFID label with data
`"0123456789ABCDEF12345678"` is in place over the antenna.

```
       ! U1 setvar "rfid.tag.data"

```

The printer responds with `"0123456789ABCDEF12345678"` .

This example gets data from the current tag, assuming that no tag data can be read or that no tag is
present.

```
       ! U1 setvar "rfid.tag.data"

```

The printer responds with “ `NO DATA` ”.


1530




SGD RFID Commands