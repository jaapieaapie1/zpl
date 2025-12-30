# ^ID




ZPL Commands


The `^ID` command deletes objects, graphics, fonts, and stored formats from storage areas. Objects can
be deleted selectively or in groups. This command can be used within a printing format to delete objects
before saving new ones, or in a stand-alone format to delete objects.


**Object Delete**

The image name and extension support the use of the asterisk ( `*` ) as a wild card. This allows you to easily
delete a selected groups of objects.

**Format:** `^IDd:o.x`







|Parameters|Details|
|---|---|
|`d =` location of stored<br>object|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|o `=` object name|**Values:** any`1` to`8` character name<br>**Default:** if a name is not specified, UNKNOWN is used|
|x `=` extension|**Values:** any extension conforming to Zebra conventions<br>**Default:** `.GRF`|


To delete stored formats from DRAM:

```
^XA
^IDR:*.ZPL^FS

^XZ

```

To delete formats and images named SAMPLE from DRAM, regardless of the extension:

```
^XA
^IDR:SAMPLE.*^FS
^XZ

```

To delete the image `SAMPLE1.GRF` prior to storing `SAMPLE2.GRF` :

```
^XA
^FO25,25^AD,18,10
^FDDelete^FS
^FO25,45^AD,18,10
^FDthen Save^FS
^IDR:SAMPLE1.GRF^FS
^ISR:SAMPLE2.GRF^FS^XZ

```

In this the * is a wild card, indicating that all objects with the `.GRF` extension are deleted:

```
^XA
^IDR:*.GRF^FS

```

245


ZPL Commands

```
^XZ

```

**Comments:** When an object is deleted from `R:`, the object can no longer be used and memory is available
for storage. This applies only to `R:` memory. With the other memory types ( `A:`, `B:`, `E:` ) the deleted object
is no longer available. The memory space recovers when an automatic defragmentation or initialization
occurs.

The `^ID` command also frees up the uncompressed version of the object in DRAM.

If the name is specified as `*.ZOB`, all downloaded bar code fonts (or other objects) are deleted.

If the named downloadable object cannot be found in the `R:`, `E:`, `B:`, and `A:` device, the `^ID` command is
ignored.


246