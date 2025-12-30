# ip.ftp.execute_file



This command setting controls the ability of the printer to process or not process commands received via
the FTP protocol using the printers ZPL engine.


**Setvar**


To turn FTP processing ability on or off:

```
       ! U1 setvar "ip.ftp.execute_file" "value"

```

**Values**

`"off"` disables the printer’s ability to process FTP commands

`"on"` enables the printer’s ability to process FTP commands

**Default**
```
          "on"

```

**Getvar**


To respond with the FTP processing ability status:

```
       ! U1 getvar "ip.ftp.execute_file"

```

**Example**

This `setvar` example shows the FTP processing ability set to `"on"` .

```
       ! U1 setvar "ip.ftp.execute_file" "on"

```

1252


SGD Network Commands