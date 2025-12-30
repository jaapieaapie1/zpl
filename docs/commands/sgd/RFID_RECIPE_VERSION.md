# rfid.recipe_version



The RFID recipe file controls how the printer manages RFID tag encoding, according to the type of tag in
use.


This command returns the version number of the RFID recipe file currently in use. The RFID recipe file is
named RFIDRCPE.XML. The default location for this file is `Z:RFIDRCPE.XML` . If a file using the same name
is stored in the E: memory location, it will be used instead of the file stored in the Z: memory location.


**Getvar**


To return the version number of the RFID recipe file currently in use:

```
       ! U1 getvar "rfid.recipe_version"

```

1527


SGD RFID Commands