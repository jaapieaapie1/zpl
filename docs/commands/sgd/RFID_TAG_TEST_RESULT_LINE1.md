# rfid.tag.test.result_line1



This command reports the results of the `rfid.tag.test.execute` command.


**Getvar**

To retrieve the results of the `rfid.tag.test.execute` command:
```
       ! U1 setvar "rfid.tag.test.result_line1"

```

1540




SGD RFID Commands

## **rfid.tag.test.result_line2**


This command reports the results of the `rfid.tag.test.execute` command.


**Getvar**

To retrieve the results of the `rfid.tag.test.execute` command:

```
       ! U1 setvar "rfid.tag.test.result_line2"

```

1541


SGD RFID Commands

## **rfid.tag.type**


This command sets the reader’s RFID tag type.


**Setvar**


To set the reader’s tag type:

```
       ! U1 setvar "rfid.tag.type" "value"

```

**Values**


**UHF Printers**


**Value** **Definition**


none None


class0 EPC Class 0


class0+ EPC Class 0 Plus


class1_64bit EPC Class 1 64-bit


class1_96bit EPC Class 1 96-bit


ucode_epc_1_19 UCODE EPC 1.19


class0+_impinj mpinj Class 0 Plus


ISO18000A ISO 18000-06A


gen2 EPC Class 1, Generation 2 (Gen 2)


ISO18000B ISO18000B


**HF Printers**


**Value** **Definition**


none None


detect Auto detect (query tag to determine)


tagit Tag*It (Texas Instruments Tagit tags)


icode I*code (Phillips Icode tags)


pico Pico Tag (Inside Technology’s)


ISO15693 ISO 15693


EPC EPC tag (13.56 MHz)


UIC UID Tag


mifare_ultralight Mifare UltraLight


**Getvar**


To respond with the reader’s current tag type:


1542


SGD RFID Commands

```
! U1 getvar "rfid.tag.type"

```

**Example**

This `setvar` example shows the reader’s tag type being set to Gen 2.

```
! U1 setvar "rfid.tag.type" "gen2"

```

For tag types supported by older printers, refer to the original RFID Programming Guide, part number
58978L-xxx).


1543


SGD RFID Commands

## **rfid.log.enabled**


This command enables or disables the RFID host log.


**Setvar**


To set the command:

```
       ! U1 setvar "rfid.log.enabled" "value"

```

**Values**

`"yes"` Enables the RFID host log

`"no"` Disables the RFID host log

**Default**
```
          "no"

```

**Example**


In this example, the setvar enables the RFID host log.

```
       ! U1 setvar "rfid.log.enabled" "yes"

```

**Getvar**


To view the current setting value:

```
       ! U1 getvar "rfid.log.enabled"

```

1544


SGD RFID Commands

## **rfid.log.entries**


This command returns the RFID host log. This command is equivalent to the `^HL` and `~HL` command. Host
logs are not displayed during an ALLCV.


**Getvar**


To get the RFID host logs:

```
       ! U1 getvar "rfid.log.entries"

```

**Result**

```
          [0x02]<start>
          Nov-13-2017 23:31:30,R,F0,A1,16,00000000,E200905962180075209038CD
          ...
          <end>[0x03]

```

In this example, "..." can be more entries.

`[0x02]` and `[0x03]` are the STX and ETX binary characters.


1545


SGD RFID Commands

## **rfid.log.clear**


This command clears the RFID host log.


**Setvar**


To set the command:

```
       ! U1 setvar "rfid.log.clear" ""

```

**Values**
NA


**Do**


To clear the RFID host logs:

```
       ! U1 do "rfid.log.clear" ""

```

**Values**
NA


1546