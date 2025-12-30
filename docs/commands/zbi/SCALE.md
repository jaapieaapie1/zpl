# Scale Program


```

This program reads data from a scale connected to the serial port by sending a "W" to the scale and
waiting for a weight to be returned. When the weight is received, it is inserted into a simple label format
and printed.


**Example**


This is an example of Scale

```
     1 rem ********************************************************
     1 rem Zebra Technologies ZBI Sample Program
     1 rem
     1 rem Professional programming services are available. Please contact
     1 rem ZBI-Experts@zebra.com for more information.
     1 rem
     1 rem This is an example of using ZBI to read scale data from the
     1 rem serial port.
     1 rem ********************************************************
     1 rem close all ports except console, open channels to parallel and
     1 rem serial ports
     1 rem ********************************************************
     05 for i = 1 to 9 step 1
     10  close #i
     20  next i
     30 open # 2 : name "SER"
     40 open # 1 : name "ZPL"
     1 rem ********************************************************
     1 rem main program; send serial port a 'W' in order to get a weight

```

600




ZBI Commands

```
1 rem ********************************************************
50 do
60  do
70    sleep 1  ! sleep so scale is not bombarded with incoming
1 rem data
80    print # 2 : "W" ;  ! semicolon ends sent W without a CRLF
1 rem ********************************************************
1 rem get response from scale; note that input requires a CRLF to be
1 rem entered
1 rem ********************************************************
90    input # 2 : a$
100    if a$ = "EXIT" then ! back door exit - if EXIT is received, ZBI
ends
110     close # 2
120     print #1: "^XZ"
130     close #1
140     end
150    end if

1 rem ********************************************************
1 rem loop until valid weight is received, then print on label
1 rem ********************************************************
160  loop while pos ( a$, "000.00" ) = 1 or pos ( a$, "?" ) = 1
170  print # 1 : "~SD25^XA^FS";
180  print # 1 : "^LH0,0^FS";
190  print # 1 : "^FO56,47^A0N,69,58^FDThis weighs^FS";
1 rem ********************************************************
1 rem print weight on label; & character concatenates strings
1 rem ********************************************************
200  print # 1 : "^FO56,150^A0N,69,58^FD" & A$ & " lbs^FS";
210  print # 1 : "^PQ1,0,0,N";
220  print # 1 : "^XZ"
1 rem ********************************************************
1 rem loop until weight is off scale, then repeat for next item
1 rem weighed
1 rem ********************************************************
230  do
240    print # 2 : "W" ;
250    input # 2 : A$
260  loop until pos(A$, "000.00") = 1 or pos(A$, "?") = 1
270 loop

```

601


# **About SGD Printer** **Commands**

**About SGD Printer Commands**


This chapter provides a high-level overview of printer setting Set / Get / Do (SGD) commands.






|SGD commands are available in printers with the following firmware versions or later:|Col2|
|---|---|
|•<br>V66.17.4Z or later<br>•<br>V61.15.xZ or later<br>•<br>V60.16.2Z or later<br>•<br>V60.15.xZ or later<br>•<br>V50.15.xZ or later<br>•<br>V56.15.xZ or later<br>•<br>V53.16.x or later|•<br>V53.15.2Z or later<br>•<br>R53.16.3Z or later<br>•<br>R60.15.8Z or later<br>•<br>R62.15.8Z or later<br>•<br>R63.15.8Z or later<br>•<br>R65.15.8Z or later|



**IMPORTANT:** These are important points to note when using ZPL and SGD commands:


- SGD commands are case-sensitive.


- ZPL and SGD commands should be sent to the printer as separate files.


- Certain settings can be controlled by both ZPL and SGD. Configuration changes made in ZPL
can affect configuration changes made in SGD.


- Changes made with one command type (ZPL or SGD) will affect the data returned to the host
in response to both ZPL and getvar commands. The command type (ZPL or SGD) that was
sent last determines the current setting.


- Some RF cards do not support all of the SGD commands.


**IMPORTANT:** These are important points to note when using a Zebra G-Series printer:


- You can send instructions to the printer using multiple programming languages: EPL, ZPL,
or SGD. EPL and ZPL commands configure the printer, print labels, and get device status
information. SGD commands set and get configuration details. These three languages can
be used without the need to send the printer instructions to switch from one language to
another.


- EPL, ZPL, and SGD commands must be sent to the printer as separate files. They cannot be
used together in one format, or set of commands. For example, if you send a series of SGD


602


## **Overview**



About SGD Printer Commands


commands to the printer and they are followed by a printable format, this needs to be done
using separate files.


This section describes how and why to use the Set/Get/Do (SGD) commands. It also provides an example
of a typical command structure.


SGD commands are commands that allow you to configure all printers with firmware versions V60.15.xZ,
V50.15.xZ, V61.15.xZ, V56.15.xZ, V53.15.xZ, or later. The printer performs the specified function immediately
after receiving the command. The commands are:


- setvar


- getvar


- do


**IMPORTANT:** SGD commands must be terminated by a carriage return or a space and line feed,
and the command, attributes, and values must be specified in lower case.


### **setvar Command**

`Setvar` commands:

        - are used to configure printer settings to specific values by setting them in the printer


        - must be terminated by a space character or a CR/LF (0x0D, 0x0A)

Some `Setvar` commands require additional settings, which must be enclosed in double quotes.

### **getvar Command**


`Getvar` commands:

        - are used to get the current value of the printer settings


        - must be terminated by a space character or CR/LF (0x0D, 0x0A)


The printer responds with the printer setting of “?” if:


        - the printer setting does not exist (usually due to incorrect spelling of the printer setting)


        - it has not been configured yet

### **do Command**


`Do` commands:

        - are used to instruct the printer to perform predefined actions


        - must be terminated by a space character or a CR/LF (0x0D, 0x0A)

Some `Do` commands require additional settings which must be enclosed in double quotes.


603


About SGD Printer Commands

## **Command Structure**


It is important to understand the structure of the command and its components. A command structure
illustration is provided for each command in this guide.


This is an example of a command structure illustration:

```
            ! U1 setvar "ip.addr" "value"
```

1 2 3


**1.** Command—always preceded with an exclamation point (!) and must be specified in lower case. A space
resides between the `!` and `U1` and between `U1` and the command ( `setvar` or `getvar` ).

**2.** Attribute—always in double quotes and must be specified in lower case.

**3.** Chosen value—always in double quotes. Only applicable for `setvar` and `do` .

This command must be terminated by a space character or a CR/ LF (0x0D, 0x0A).

### **How to Send Multiple SGD Commands**


For any `getvar`, `setvar`, or `do` command, if you issue the syntax without the `"1"` and use the END
command followed by a space, multiple SGD commands are sent simultaneously.

This syntax shows how you can send multiple `getvar` commands:

```
       ! U getvar "ip.telnet.enable"
       getvar "ip.dhcp.enable" getvar "ip.dhcp.cid_prefix"
       END

```

**1.** The command portion of the string does not use the "1" after the "! U".

```
         ! U getvar "ip.telnet.enable"

```

**2.** Commands issued after the first command do not require the "! U".

```
         getvar "ip.dhcp.enable" getvar "ip.dhcp.cid_prefix"

```

**3.** The string of commands is terminated by the word "END" with a space after the word, and by a carriage
return/ line feed.

```
         END

## **JSON (JavaScript Object Notation)**

```

JSON (JavaScript Object Notation) is an open standard format that uses human- and machine-readable text
for device management. It transmits data objects consisting of elements as attribute–value pairs.


You can use use JSON as an alternative to using the SGD (Set-Get-Do) mechanism when reading or writing
parameters on QLn and iMZ mobile printers. JSON is a popular open standard for exchanging data objects
and is well suited to this task.


604


About SGD Printer Commands


The main settings channel for JSON is TCP port 9200, but other ports can be used. JSON commands are
processed when received. Up to eight connections are allowed, and all connected ports are active, and the
JSON commands will work while the printer is printing.


The port used for JSON can be changed or disabled using ip.port_json_config on page 1306.


**NOTE:** JSON is available on all communications ports, unless `line_print` is enabled, in which
case you must use the main TCP JSON port, 9200. If you connect to port 9200, the printer ONLY
accepts JSON commands. CPCL, SGD, ZPL, and other command languages are not supported.

### **Configuring JSON Usage for Communications**


All JSON commands should follow the JSON specification for escaping, spacing, etc. All JSON commands
are prefixed by `{}` .

Refer to http://www.json.org/ for full details on JSON formatting.


By enclosing a variable’s value in curly braces, it indicates that the value is an object. Inside the object, you
can declare any number of properties using a `"name": "value"` pairing, separated by colons. Multiple
pairings are separated by commas.


Use the SGD variable name in the JSON command structure. To configure JSON usage for communication,
refer to the following examples.


**Getvar using JSON**


To do a `getvar` in SGD you use the format:

```
     ! U1 getvar "sgd.name"
     ! U1 getvar "ip.port"
     ! U1 getvar "device.location"

```

To get a variable value using JSON:

```
     {}{"sgd.name":null} returns {"sgd.name":"value"}
     {}{"ip.port":null} returns {"ip.port":"9100"}
     {}{"device.location":null} returns {"device.location":"my desk"}

```

You can get several values as follows:

```
     {}{"device.friendly_name":null, "device.company_name":null,
     "device.company_contact":null, "device.location":null}

```

The response is:

```
     {"device.friendly_name":"XXQLJ120900310",
     "device.company_name":"Zebra Technologies",
     "device.company_contact":"123-555-1212",
     "device.location":"My Desk"}

```

605


About SGD Printer Commands


**Setvar using JSON**


To do a `setvar` in SGD you use the format:

```
     ! U1 setvar "sgd.name" "value"
     ! U1 setvar "ip.port" "9200"
     ! U1 setvar "device.location" "my desk"

```

To set a variable value using JSON:

```
     {}{"sgd.name":"value"} sets the variable value to "value"
     {}{"ip.port":"1234"} sets the variable value to "1234"
     {}{"device.location":"my desk"} sets the variable value to "my desk"

```

**NOTE:** When you set an SGD value, it will return the value that was set, or the old value if the set
failed. If:

```
     {}{"sgd.name":"new_value"} fails, the variable value remains "old_value"

```

To set several values at once:

```
     {}{"device.friendly_name":"XXQLJ120900310", "device.company_contact":"123#
     555#1212", "device.location":"My Desk"}

```

The response is:

```
     {"device.friendly_name":"XXQLJ120900310", "device.company_contact":"123#555#
     1212", "device.location":"My Desk"}

### **Get an SGD Branch**

```

You can retrieve all branch values by specifying the branch.

```
     {}{"bluetooth":null} returns all SGDs in branch and their values.

### **Get an allvalues Report**

```

You can request an `allvalues` report with just the values for all settings with characteristics. This will
return all SGDs and their values.

```
     {}{"allvalues":
     {"ip.port":"6101”, "ip.port_alternate":"9100”, "ip.sgd_json_port":"9200”,
     ...
     }}

```

606


About SGD Printer Commands

### **Get an allconfig Report**


You can request an allconfig report using JSON, and it will return all settings with characteristics.


To get all SGDs and their values along with various other information including defaults:

```
     {}{"allconfig":null}

```

**NOTE:** For the `"allconfig"` response, it will start with `{"allconfig":{` and end with `}}`


If you do an `allconfig`, you can get the setting attributes for all settings as follows:

```
     {"allconfig": {"ip.port":{"value":"6101","type":"integer","range":"0     65535","clone":true,
     "archive":true,"access":"RW"},
     "ip.port_alternate":
     {"value":"9100","type":"integer","range":"0-65535","clone":true,
     "archive":true,"access":"RW"},
     "ip.sgd_json_port":{"value":"9200","type":"integer","range":"0     65535","clone":true,
     "archive":true,"access":"RW"}, another setting, ... the last setting}}

```

where:

        - `"value"` indicates the current value stored in the setting.

        - `"type"` indicates the type of value. Possible values are integer, enum, bool, string, double, ipv4address, ipv6- address.

        - `"range"` indicates the range of a setting. For strings this is the range of the string length. For enums it
is the possible enum values.

        - `"clone"` indicates if it is safe to store this setting and apply it to another link-os printer.

        - `"archive"` indicates if is safe to store this setting and apply it to same link-os printer at a later time.

        - `"access"` indicates if the setting is RW (read/write), R (read-only), or W (write-only).

If you do an `allconfig`, you can get the setting attributes for all settings as follows:

```
     {}{"allconfig":null}

```

For the values used above it returns these entries:

```
     "device.friendly_name":{"value":"XXQLJ120900310","type":"string","range":
     "0-17","clone":false,"archive":true,"access":"RW"},
     "device.company_contact":{"value":"123-555-1212","type":"string","range":
     "0-128","clone":true,"archive":true,"access":"RW"},
     "device.location":{"value":"my desk","type":"string","range":"
     0-128","clone":true,"archive":true,"access":"RW"},

```

607