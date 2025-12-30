# ^JB




ZPL Commands


The `^JB` command is used to initialize various types of Flash memory available in the Zebra printers.


**Initialize Flash Memory**

**Format:** `^JBa`

|Parameters|Details|
|---|---|
|`a =` device to initialize|**Values:**<br>`A =` Option Flash memory<br>`B =` Flash card (PCMCIA)<br>`E =` internal Flash memory<br>**Default:** a device must be specified|



**Example:** This is an example of initializing the different types of flash memory:

`^JBA –` initializes initial Compact Flash memory when installed in the printer.

`^JBB –` initializes the optional Flash card when installed in the printer.

`^JBE –` initializes the optional Flash memory when installed in the printer.


**NOTE:** Initializing memory can take several minutes. Be sure to allow sufficient time for the
initialization to complete before power cycling the printer.


252