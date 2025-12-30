# ^SF




ZPL Commands


The `^SF` command allows you to serialize a standard `^FD` string. The maximum size of the mask and
increment string is 3K combined.


**Serialization Field (with a Standard ^FD String)**


In firmware version x.14 and later, strings are serialized from the last character in the backing store with
regard to the alignment of the mask and increment strings. For combining semantic clusters that do not get
incremented, the mask character % needs to be added to the increment string.

**Format:** `^SFa,b`

|Parameters|Details|
|---|---|
|`a =` mask string|The mask string sets the serialization scheme. The length of the string mask<br>defines the number of characters (or in firmware version x.14 and later,<br>combining semantic clusters) in the current`^FD` string to be serialized.<br>The mask is aligned to the characters (or in firmware version x.14 and later,<br>combining semantic clusters) in the`^FD` string starting with the right-most<br>(or in firmware x.14 and later, last) in the backing store position.<br>**Mask String placeholders:**`D`<br>or`d` – Decimal numeric 0–9<br>`H` or`h` – Hexadecimal 0–9 plus a-f or A-F<br>`O` or`o` – Octal 0–7<br>`A` or`a` – Alphabetic A–Z or a–z<br>`N` or`n` – Alphanumeric 0–9 plus A–Z or a–z<br>`%` – Ignore character or skip|
|`b =` increment string|The increment string is the value to be added to the field on each label.<br>The default value is equivalent to a decimal value of one. The string<br>is composed of any characters (or in firmware version x.14 and later,<br>combining semantic clusters) defined in the serial string. Invalid characters<br>(or in firmware version x.14 and later, combining semantic clusters) are<br>assumed to be equal to a value of zero in that characters (or in firmware<br>version x.14 and later, combining semantic clusters) position.<br>The increment value for alphabetic strings starts with ‘`A`’ or ‘`a`’ as the<br>zero placeholder. This means to increment an alphabetic character (or in<br>firmware version x.14 and later, combining semantic cluster) by one, a value<br>of ‘`B`’ or ‘`b`’ must be in the increment string.|



For characters that do not get incremented, the `%` character needs to be added to the increment string.

**Example:** This is an example of serializing a `^FD` string. The ZPL II code generates three separate labels as
seen in Generated Labels:


334


ZPL Commands


This mask has the first characters (or in firmware version x.14 and later, the first combining semantic
clusters) as alphanumeric ( `nn =` 12), and the last digit is uppercase alphabetic (A). The decimal value of the
increment number is equivalent to 5 (F). The number of labels generated depends on the number specified
by the `^PQ` command.

In a similar instance, the `^FD` string could be replaced with either of the `^FD` strings below to generate a
series of labels, determined by `^PQ` .

Using this ZPL code:

```
^FDBL0000^SFAAdddd,1

```

The print sequence on this series of labels is:

```
BL0000, BL0001,...BL0009, BL0010,...
BL0099, BL0100,...BL9999, BM0000...

```

Using this ZPL code:

```
^FDBL00-0^SFAAdd%d,1%1

```

The print sequence on this series of labels is:

```
BL00-0, BL01-1, BL02-2,...BL09-9,
BL11-0, BL12-1...

```

Important notes about masking for firmware version V60.14.x, V50.14.x, or later:


- A single % masks an entire combining semantic cluster rather than a single code point.


- The mask string and increment string should be aligned at the last code point in their respective
backing stores.


- Control and bidirectional characters do not require a mask and are ignored for serialization purposes.


The following examples show the importance of capitalization and location within the mask.


335


ZPL Commands


**Example 1**


In this example, the printer cycles with every two printed labels and alternates between H (position 18),
and then Z (position 36). With n or N, the serial number increments from 0 - 9 and a–z or A–Z (36 positions
overall). With each completed cycle, the second cluster (nn) increments one position (from 00, 01, 02 …) per
cycle:


**Example 2**


In this example, lowercase i increments with a mask string of nnN. Nothing changes because the first
cluster (Z) never triggers the second cluster (zz) to change.


336