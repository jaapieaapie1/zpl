# ^TO




ZPL Commands


The `^TO` command is used to copy an object or group of objects from one storage device to another. It is
similar to the copy function used in PCs.


**Transfer Object**


Source and destination devices must be supplied and must be different and valid for the action specified.
Invalid parameters cause the command to be ignored.

The asterisk (*) can be used as a wild card for object names and extensions. For instance, `ZEBRA.*` or

- `.GRF` are acceptable forms for use with the `^TO` command.

At least one source parameter ( `d`, `o`, or `x` ) and one destination parameter ( `s`, `o`, or `x` ) must be specified. If
only `^TO` is entered, the command is ignored.

**Format:** `^TOs:o.x,d:o.x`













|Parameters|Details|
|---|---|
|`s =` source device of<br>stored object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** if a drive is not specified, all objects are transferred to the drive set<br>in parameter`s`|
|`o =` stored object name|**Values:** any existing object conforming to Zebra conventions<br>**Default:** if a name is not specified, * is used — all objects are selected|
|`x =` extension|**Values:** any extension conforming to Zebra conventions<br>**Default:** if an extension is not specified, * is used — all extensions are<br>selected|
|`d =` destination device of<br>the stored object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** a destination must be specified|
|`o =` name of the object at<br>destination|**Values:** up to 8 alphanumeric characters<br>**Default:** if a name is not specified, the name of the existing object is used|
|`x =` extension|**Values:** any extension conforming to Zebra conventions<br>**Default:** if an extension is not specified, the extension of the existing object<br>is used|


**Comments:** Parameters `o`, `x`, and `s` support the use of the wild card (*).

If the destination device does not have enough free space to store the object being copied, the command
is canceled.

Zebra files ( `Z:*.*` ) cannot be transferred. These files are copyrighted by Zebra Technologies.


**Transferring Objects**

These are some examples of using the `^TO` command.

**Example:** To copy the object `ZLOGO.GRF` from DRAM to an optional Memory Card and rename it
`ZLOGO1.GRF`, write the following format:

```
^XA
^TOR:ZLOGO.GRF,B:ZLOGO1.GRF
^XZ

```

356


ZPL Commands


**Example:** To copy the object `SAMPLE.GRF` from an optional Memory Card to DRAM and keep the same
name, write this format:

```
^XA
^TOB:SAMPLE.GRF,R:SAMPLE.GRF
^XZ

```

**Transferring Multiple Objects**

The asterisk ( `*` ) can be used to transfer multiple object files (except `*.FNT` ) from DRAM to the Memory
Card. For example, assume you have several object files that contain logos. These files are named
`LOGO1.GRF`, `LOGO2.GRF`, and `LOGO3.GRF` .

To transfer all these files to the memory card using the name NEW instead of LOGO, place an asterisk
after the names NEW and LOGO in the transfer command. This copies all files beginning with LOGO in one
command.

```
^XA
^TOR:LOGO*.GRF,B:NEW*.GRF
^XZ

```

During a multiple transfer, if a file is too big to be stored on the memory card, that file is skipped. All
remaining files attempt to be transferred. All files that can be stored within the space limitations are
transferred, while other files are ignored.


357