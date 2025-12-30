# ^NS




ZPL Network Commands


Use this command to change the wired print server network settings.


**Change Wired Networking Settings**

**Format:** `^NSa,b,c,d,e,f,g,h,i`







|Parameters|Details|
|---|---|
|`a` = IP resolution|**Values:**<br>A = ALL<br>B = BOOTP<br>C = DHCP AND BOOTP<br>D = DHCP<br>G = GLEANING ONLY<br>R = RARP<br>P = PERMANENT<br>**Default:**`A`<br>Use of GLEANING ONLY is not recommended when the Wireless Print<br>Server or Wireless Plus Print Server is installed|
|`b` = IP address|**Values:** Any properly formatted IP address in the xxx.xxx.xxx.xxx format.|
|`c` = subnet mask|**Values:** Any properly formatted subnet mask in the xxx.xxx.xxx.xxx format.|
|`d` = default gateway|**Values:** Any properly formatted gateway in the xxx.xxx.xxx.xxx format.|
|`e` = WINS server address|**Values:** Any properly formatted WINS server in the xxx.xxx.xxx.xxx format.|
|`f` = connection timeout<br>checking|**Values:**<br>`Y =` Yes<br>`N =` No<br>**Default:** `Y`|
|`g` = timeout value|Time, in seconds, before the connection times out.<br>**Values:** `0` through`9999`<br>**Default:** `300`|
|`h` = ARP broadcast interval|Time, in minutes, that the broadcast is sent to update the device’s ARP<br>cache.<br>**Values:** `0` through`30`<br>**Default:** `0` (no ARP sent)|
|`i` = base raw port number|The port number that the printer should use for its RAW data.<br>**Values:** `1` through`65535`<br>**Default:** `9100`|


**Example:**

```
^XA

```

387


ZPL Network Commands

```
^NSa,192.168.0.1,255.255.255.0,192.168.0.2
^XZ

```

**Comments:**

For the Xi4, RXI4, ZM400/ZM600, and RZ400/RZ600 printers, Zebra recommends that you use the `^ND`
command instead of the `^NS` command.


388