# ^RF




ZPL RFID Commands


Use this command to read or write to (encode) an RFID tag or to specify the access password.


**Read or Write RFID Format**


When using this command to read a tag, you may use a field variable to print the tag data on the label or to
return the data to the host. For more information on how memory is stored on a Gen 2 tag or for examples
that use a field variable, refer to the RFID Programming Guide for your printer. A copy of the manual is
[available at http://www.zebra.com/manualshttp://www.zebra.com/manuals.](http://www.zebra.com/manuals)

**Format:** `^RFo,f,b,n,m`

|Parameters|Details|
|---|---|
|`o `= operation|Specifies the action to be performed.<br>Values:<br>`W` = write to (encode) the tag<br>`L =`write with LOCK (if supported by tag type; Gen 2 tag type does not use<br>this locking function)`R =`read the tag<br>`P` = read password (Gen 2 tag type only. Not supported on all Gen 2 printers,<br>including the ZD500R printer.)<br>`S =`specify the access password<br>**Default:**`W`|
|`f` = format|**Values:**<br>`A` = ASCII<br>`H` = Hexadecimal<br>`E` = EPC (ensure proper setup with the`^RB` command)<br>**Default:**`H`|



425


ZPL RFID Commands





|Parameters|Details|
|---|---|
|`b` = passwordOR`b` =<br>starting block number|**For Gen 2 tag type only:**<br>What you specify for this parameter depends on what you enter for other<br>parameters.<br>**NOTE:** When the Gen 2 memory bank parameter is set to`E`<br>(EPC96-bit) or`A` (EPC and Auto adjust PC bits),`W` and`R` values are<br>always set to 2.<br>**If the Operation parameter value is...**<br>**W**<br>**Values:**<br>`P`, which indicates that an access password, a kill password, or both follow in<br>a ^FD command. Each password must be 8 hex characters. If the password<br>is omitted, it is not written. An access password is used in subsequent lock<br>commands in the format.<br>0 to n, which specifies the 16-bit starting block number, where n is the maximum<br>number of blocks for the bank specified in the memory bank parameter.<br>**Default:**`0`<br>**R**<br>**Values:**<br>0 to n, which specifies the 16-bit starting block number, where n is the maximum<br>number of blocks for the bank specified in the memory bank parameter.<br>**Default:** `0`<br>**S **This parameter must be`P` and must be followed by the access password in a<br>^FD command.<br>**For tag types other than Gen 2:**<br>Specifies the starting block number.<br>**Values:** `0` to n, where n is the maximum number of blocks for the tag.<br>**Default:** `0`|
|`n` = number of bytes<br>to read or write|Specifies the number of bytes to read or write.<br>**For high-frequency (HF) printers:**<br>**Values:**1 to n, where n is the maximum number of bytes for the tag.<br>**Default:**1<br>**For Gen 2 tag type only:**<br>When`E` or`A` is specified for the memory bank parameter, this value is not<br>required.<br>**Values:**1 to n, where n is the maximum number of bytes for the tag.<br>**Default:**1<br>**For all other printers and tag types:**This parameter applies only when the<br>starting block number is 1.<br>**Values:**1 to n, where n is the maximum number of bytes for the tag. For UCODE<br>EPC 1.19, n is 32.<br>**Default:**1|


426


ZPL RFID Commands

|Parameters|Details|
|---|---|
|`m` = Gen 2 memory<br>bank|**NOTE:** This parameter applies to Gen 2 tags only.<br>Specifies the Gen 2 memory bank. For more information about Gen 2 memory,<br>refer to the RFID Programming Guide for your printer.<br>`E =`EPC 96-bit (When writing data, this parameter performs the operation on<br>Gen 2 bit address 20h and accesses 12 bytes of the EPC memory bank. When<br>reading data, this parameter reads the amount of data specified in the PC bits<br>on the tag.)<br>`A =`EPC and Auto adjust PC bits (When writing data, this parameter performs<br>the operation on Gen 2 bit address 20h of the EPC memory bank and accesses<br>the number of bytes specified in the`^FD`. The PC bits will be updated to match<br>the amount of data written to the tag. When reading data, this parameter reads<br>the amount of data specified in the PC bits on the tag.<br>This value is supported only by the ZD500R printer and ZT400 Series and<br>ZT600_Series RFID printers.<br>`0 =`Reserved<br>`1 =`EPC<br>`2 =`TID (Tag ID)<br>`3 =`User<br>**Default:**`E`|



**Example:** This example encodes 96-bit data in ASCII format. (The `^RS` command can be omitted for
printers that use Gen 2 tag types only.)

```
^XA
^RS8
^RFW,A^FD00 my data^FS
^XZ

```

**Example:** This example encodes 96-bit EPC data, as specified by the `^RB` command.

```
^XA
^RB96,8,3,3,20,24,38
^RFW,E^FD16,3,5,78742,146165,1234567891^FS
^XZ

```

**Example:** This example encodes 4 bytes of hexadecimal formatted data, starting in block 3 of Gen 2 EPC
bank 1. (The `^RS` command can be omitted for printers that use Gen 2 tag types only.)

```
^XA
^RS8
^RFW,H,3,4,1^FD11112222^FS
^XZ

```

427


ZPL RFID Commands


**Example:** This example reads the extended Gen 2 tag ID (TID), which is not read by the `^RI` command,
and returns the results to the host computer. The results are labeled with the header Tag ID
“8-byte
Data.” (The `^RS` command can be omitted for printers that use Gen 2 tag types only.)

```
^XA
^RS8
^RFR,H,0,8,2^FN1^FS^HV1,,8-byte Tag ID Data:^FS
^XZ

```

**Example:** This command writes and specifies both the access password ( `12345678` ) and the kill password
( `88887777` ) separated by a comma.

```
^RFW,H,P^FD12345678,88887777^FS

```

This command writes the access password only:

```
^RFW,H,P^FD12345678^FS

```

This command writes the kill password only (a comma must be used before it to distinguish it from an
access password):

```
^RFW,H,P^FD,88887777^FS

```

See the examples for ^RL for how this command would be used in a format.

**Example:** This command writes `1122334455667788` to the bit address 20h of the EPC memory and
updates the PC bits bit address 10h to 14h to reflect 8 bytes (4 words) of data.

```
^RFW,H,,,A^FD1122334455667788^FS

```

**Example:** This command specifies the access password for the tag, which will be used in subsequent
lock commands in the format. The access password specified must match the one stored on the tag. This
command does not write the password to the tag. See the examples for ^RL for how this command would
be used in a format.

```
^RFS,H,P^FD12345678^FS

```

428