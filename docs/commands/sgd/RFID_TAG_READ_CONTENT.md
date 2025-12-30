# rfid.tag.read.content



This command instructs the printer which data to read from the tag with the `rfid.tag.read.execute`
command.


See rfid.tag.read.execute on page 1532.


**Setvar**

To instruct the printer which data to read from the tag with the `rfid.tag.read.execute` command:

```
       ! U1 setvar "rfid.tag.read.content" "value"

```

**Values**

              - `"epc"` reads the EPC data based on the EPC size specified in the RFID tag’s protocol bits, up to
160 bits

              - `"tid information"` reads the first 32 bits of the TID (Tag ID)

              - `"password status"` reads the tag’s access and kill passwords

              - `"protocol bits"` reads the protocol bits from the EPC memory banks and converts that
value to the EPC size

              - `"memory bank sizes"` reads the EPC, TID, and user memory banks sizes

              - `"up"` sets the command to the previous test

              - `"down"` sets the command to the next test

**Default**
```
          "epc"

```

**Getvar**


To retrieve the current setting:

```
       ! U1 getvar "rfid.tag.read.content"

```

1531


SGD RFID Commands