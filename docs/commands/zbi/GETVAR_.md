# GETVAR$



This function retrieves printer parameters.


**Format**
```
          GETVAR$ (PARAM$)
```

**Parameters**
`PARAM$` = the printer parameter to get.

**Returns**
The value of the parameter. Refer to the SGD commands for specific parameters.

**Example**
This is an example of the `GETVAR$` command:

```
          AUTONUM 1,1
          LET SGDCOUNT = 7
          DECLARE STRING SGDQUERY$(2,SGDCOUNT)
          LET SGDQUERY$(1,1) = "appl.name"
          LET SGDQUERY$(1,2) = "device.printhead.serialnum"
          LET SGDQUERY$(1,3) = "internal_wired.ip.addr"
          LET SGDQUERY$(1,4) = "internal_wired.ip.netmask"
          LET SGDQUERY$(1,5) = "internal_wired.ip.gateway"
          LET SGDQUERY$(1,6) = "internal_wired.ip.port"
          LET SGDQUERY$(1,7) = "internal_wired.mac_addr"
          FOR I = 1 TO SGDCOUNT
          LET SGDQUERY$(2,I) = GETVAR$(SGDQUERY$(1,I))
          NEXT I
          OPEN #1: NAME "ZPL"
          PRINT #1: "^XA"
          FOR I = 1 TO SGDCOUNT
          PRINT #1: "^FO50,";50*I;"^A0N,25,25^FD";SGDQUERY$(1,I);"=";
          PRINT #1: SGDQUERY$(2,I);"^FS"
          NEXT I
          PRINT #1: "^XZ"

```

**Comments**
None


585


ZBI Commands