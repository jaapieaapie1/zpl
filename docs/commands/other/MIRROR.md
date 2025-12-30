# Mirror


**Mirror**


This section provides an overview of Mirror, details on how to use it, and configuration examples.

## **Mirror Overview**


Mirror is a feature that gives you the ability to:


          - Centrally manage and monitor the deployment of your Zebra printers


          - Centrally configure and maintain your Zebra printers through remote updates


          - Remotely monitor printer updates - via the "Feedback" feature


There are several Set/Get/Do (SGD) commands that are used to configure and initiate Mirror. For details
see, Mirror Printer Configuration.

### **Benefits**


When using Mirror, updating the configuration and firmware on the printer is remotely managed from a
centralized FTP server. Configurations can be uniformly deployed to individual printers or to groups of
printers. Unique Configurations can also be targeted to printers as needed.


Through the "Feedback" feature, Administrators can easily log and monitor configuration updates on a
printer-by-printer basis.


Typical uses of Mirror include:


          - configuring printers as they are first received


          - performing scheduled maintenance updates, sending firmware, fonts, graphics and other objects to the
printer as needed


          - changing printer Configurations in order to move printers from one role to another


This figure provides an illustration of Mirroring.


1681


Mirror


**Figure 29** Mirror Illustration


1 Workstation — sends SGD commands to the printer to configure it for Mirror use.

2 FTP Server — Stores configuration files and responds to Mirror requests from the printer.
Receives and stores “Feedback” content.

3 Access Point — wireless network infrastructure

4 Hub — wired network infrastructure

5 Zebra Printer(s) — Configured using SGD commands. Sends Mirror requests to the FTP
server to receive files. Transmits “Feedback” content to the FTP server to log Mirror event
transactions and resulting printer settings.

### **Professional Services for Mirror Configuration**


Zebra offers a Professional Services group that can help with the configuring the Mirror feature. To inquire
about Zebra's Professional Services, contact your Zebra account representative.

## **Requirements**


These are the requirements for Mirror:


          - Zebra printer loaded with Mirror capable firmware and Print Server. For details, see Mirror Printer
Configuration.


1682


Mirror


- FTP server (with UNIX-style directory listings), configured with the following directories:

  - `"<update-root>"/appl`   - This directory is used for printer firmware updates. During an update
operation, the printer will first check the `"<update-root>/appl"` directory for new printer
firmware

  - `"<update-root>"/files`   - This directory is used for printer-resident files. Files in this directory
will be stored locally on the printer's file system. Files are not processed by the printer; they are only
stored.

  - `"<update-root>"/commands`   - This directory is used for a limited number of printer executable
commands. The contents of files in this directory will be sent to the printer's command interpreter for
execution.

  - `"<feedback-root>"`   - This optional directory is used to receive Mirror feedback content from the
printer. The content sent to this directory is defined by the `"feedback.get"` template file stored
on the printer.


- A client account (user name and password) on the FTP server for the printer to use.


- A Terminal Emulation program, used to send SGD commands to the printer to configure Mirror.


1683


Mirror

### **Supported Printers and Print Server Types**


These are the Zebra printers, firmware versions, and Zebra print servers that support Mirror.


**Table 40** Supported Printers and Print Server Types

|Printer|Firmware|Print Servers|
|---|---|---|
|ZT400|V75.19.7Z (or later)|•<br>ZebraNet a/b/g/n Print Server<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|ZE500|V53.17.15Z (or later)|•<br>ZebraNet a/b/g/n Print Server<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|ZT200|V72.18.2Z (or later)|•<br>ZebraNet a/b/g/n Print Server<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|105SLPlus|V53.17.15Z (or later)|•<br>ZebraNet b/g Print Server<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|Xi4™ series|V53.17.5Z (or later)|•<br>Internal, Integrated 10/100<br>IPV4 wired<br>•<br>Internal Wireless Plus|
|XiIIIPlus™ series|V60.17.5Z (or later)|•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|105SL™|V60.17.5Z (or later)|•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|PAX4™|V60.17.5Z (or later)|•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|ZM400™|V53.17.5Z (or later)|•<br>Internal 10/100 wired\<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|ZM600™|V53.17.5Z (or later)|•<br>Internal 10/100 wired\<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|



1684


Mirror


**Table 40** Supported Printers and Print Server Types (Continued)

|Printer|Firmware|Print Servers|
|---|---|---|
|S4M™|V53.17.5Z (or later)|•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|G™ -series|V56.17.5ZV61.17.5Z (or later)|•<br>Internal 10/100 wired\<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|
|LP 2824 Plus TLP 2824 Plus|V61.17.3Z (or later)|•<br>Internal 10/100 wired\<br>•<br>Internal Wireless Plus<br>•<br>Wireless Plus|



**NOTE:** Print Servers connected to the parallel port, either externally or internally, are not
supported for Mirror use.

### **How Mirror Works**


Mirror utilizes a network connection and FTP communications to perform remote updates and feedback
operations. At printer startup and/or at regular intervals, the printer will establish a FTP connection back to
a central FTP server and check for updates. During an update operation, the printer will check for updated
files in specific directories at the FTP server.


These are the specific FTP server directories that will be checked:

        - `"<update-root>"/appl`        - This directory is used for printer firmware updates. During an update
operation, the printer will first check the ''<update-root>/appl'' directory for new printer firmware

        - `"<update-root>"/files`        - This directory is used for printer-resident files. Files in this directory will
be stored locally on the printer's file system. Files are not processed by the printer; they are only stored.

        - `"<update-root>"/commands`        - This directory is used for a limited number of printer executable
commands. The contents of files in this directory will be sent to the printer's command interpreter for
execution.

        - `"<feedback-root>"`        - This optional directory is used to receive Mirror feedback content from the
printer. The content sent to this directory is defined by the "feedback.get" template file stored on the
printer.


**NOTE:** '' `<update-root>` '' refers to the value of the '' `ip.mirror.path` '' configuration
parameter. Files in the `<update-root>/files` directory should not have download headers in
them. They should be in the exact format they will be in when stored on the printer's file system.
Examples of download headers are: `~DY, ~DG, ! CISDFCRC16` or `~DF` .


**IMPORTANT:**


When the printer is in the Mirror process:


        - It is unavailable for other tasks.


        - The LCD will indicate that it is performing a Mirror function, showing when the printer is
downloading firmware and the names of the object files as they are transferred to the printer.


1685


Mirror

### **Mirror Process Summary**


The Mirror process follows a specific series of steps:

**1.** After power-up, the printer will first check the `<update-root>/appl` directory on the FTP server for
new printer firmware and update the printer if necessary

**2.** If the printer did not find new firmware to download, it will then check in the `<update-root>\files`
and `<update-root>/commands` directories for updated files - and download them as needed.

**3.** As a final Mirror step, the printer can perform an optional Feedback operation, transmitting a file of userdefined printer configuration information to `<feedback-root>` directory on the FTP server.

**4.** Finally, if any files or commands were downloaded during Step 2:, the printer will automatically reset
itself.

### **Mirror Process Details**


The following items are important to be aware of when configuring the FTP server to support Firmware
updated via Mirror.

        - Firmware files must be named using the following format: `<firmware version>.zpl`, where
<firmware version> is the exact Firmware revision contained in the file. For example, for Firmware
version “V53.17.5Z”, the filename stored on the FTP server must be “V53.17.5Z.ZPL”. If the file name and
Firmware version do not match, the update will not succeed.


        - The firmware filename stored on the FTP server is not case sensitive. This means that “V53.17.5Z.ZPL”
and “v53.17.5Z.zpl” will be processed in the exact same way.

        - The `/appl` directory can contain only one (1) file at a time. If there is more than 1 file in this directory, the
printer will not download anything and will skip the firmware update.

During a Mirror event, the printer will use `<firmware version>` part of the filename contained in the `/`
`appl` directory on the FTP server to check if the Firmware stored on the FTP server is different than the
firmware the printer is currently using. If the `<firmware version>` part of the Firmware file name in the
`/appl` directory on the FTP server does not exactly match the printer's Firmware version, the file on the
FTP server will be downloaded and used to update the printer. Once the printer is updated with the new
Firmware, the printer will reset and being using the new Firmware.


**NOTE:** Performing the Firmware update first is important because it is possible that the files or
commands to be downloaded via Mirror will be dependent on the new firmware.

After the printer has completed the Firmware portion of an update operation, it will check the '' `<update-`
`root>/files` '' directory and '' `<update-root>/commands` '' directory (in that order) for updated files
that need to be retrieved. If a file exists on the FTP server and the server timestamp for the file does not
match the printer's archived timestamp for the file, the printer will re-download the file and update its
timestamp info. Files that are on the Mirror FTP server but not currently present on the printer will be sent
to the printer. The supported file types are the standard files supported on ZPL printers.


If any files are downloaded during this portion of an update operation, the printer will reboot after the file
downloads have completed. At this point, the update operation is complete.


**IMPORTANT:**

        - Files in the `<update-root>/files` directory should not have download headers in them.
They should be in the exact format they will be in when stored on the printer's file system.
Examples of download headers are: `~DY, ~DG, ! CISDFCRC16` or `~DF` .


1686


Mirror


        - ZPL files in the `<update-root>/files` directory must use the printers internal characters
for the Format Command Prefix (^), Delimiter Character (,) and Control Command Character
(~). This means that the caret (^) should be replaced with a HEX 1E, the comma (,) should be
replaced with a HEX 1F and the tilde (~) should be replaced with a HEX 10.


After an update operation is complete, a printer will perform a feedback operation, if configured to do so.
During a feedback operation, the printer will open its feedback template file (named `feedback.get` ),
populate it based on its contents, and upload the resulting contents file to the FTP server, in the
'' `<feedback-root>` '' directory.


**NOTE:** `"<feedback-root>"` refers to the value of the `"ip.mirror.feedback.path"`
configuration parameter

### **Creating ZPL Files for Use in the <update-root>/files Directory**


When creating ZPL formats that will be stored in the ''<update-root>/files'' directory it is necessary to edit
the files using the following guidelines:

**1.** Files must contain only one format. This means that if a file contains multiple `^XA` and `^XZ` commands,
those sections of the file must be split into separate formats, or combined as one format.


**2.** The characters used for the Format Command Prefix (^), Delimiter Character (,) and Control Command
Characters (~) must be substituted for their Hexadecimal equivalents.

**3.** The `^XA` and `^XZ` commands must be removed from the formats.

**4.** The `^DF` command should be removed.


**One Format per File**


Files must contain only one format. This means that if a file contains multiple `^XA` and `^XZ` commands,
those sections of the file must be split into separate formats, or combined as one format. For example, if
a ZPL file contains both an initialization string and a format, the two sections must be either split into two
files, or combined into one format.


For example, given the following formats:

```
     ^XA
     ^LT0^MNW^MTT^PON^PMN^LH0,0^JMA^PR2,2^LRN^CI0
     ^XZ
     ^XA
     ^FO20,100^IME:ZEBRA.BMP^FS
     ^A@N,75,75,TT0003M_.TTF
     ^FO20,400^FDZebra Technologies^FS
     ^XZ

```

The following file should be created, which includes all of the command in one file:

```
     ^XA
     ^LT0^MNW^MTT^PON^PMN^LH0,0^JMA^PR2,2^LRN^CI0
     ^FO20,100^IME:ZEBRA.BMP^FS
     ^A@N,75,75,TT0003M_.TTF
     ^FO20,400^FDZebra Technologies^FS
     ^XZ

```

1687


Mirror


**Character Substitution**


The characters used for the Format Command Prefix (^), Delimiter Character (,) and Control Command
Characters (~) must be substituted for their Hexadecimal equivalents. During normal operation, this is
how the printer works with ZPL formats. For example, when a ZPL format is sent to the printer via a telnet,
RS-232 or Ethernet connection and stored for later use, it automatically processes the file and makes these
character substitutions.

When ZPL formats are sent to the printer from the `"<update-root>/files"` directory they are stored
on the printers memory, but not processed. For this reason, it is necessary to preprocess the files so that
they are ready for use.

The following character substitutions must be made to files sent from the `"<update-root>/files"`
directory:

|Original Character|Substitute Character|
|---|---|
|Command PrefixThe default is the Caret (^)|HEX 1E|
|Delimeter PrefixThe default is the comma (, )|HEX 1F|
|Control PrefixThe default is the tilde (~)|HEX 10|



For example, given the following format:

```
    ^XA
    ^FO20,100^IME:ZEBRA.BMP^FS
    ^A@N,75,75,TT0003M_.TTF
    ^FO20,400^FDZebra Technologies^FS^XZ

```

It would be necessary to replace the ^ characters with a HEX 1E and the, characters with a HEX 1F. This
can be done using a Text Editor. See Example Files for more information.


**Removing the ^XA and ^XZ commands**


Additionally, the `^XA` and `^XZ` commands should be removed from the format. The printer will
automatically add these commands back in to process the file. See Example Files for more information.


**Removing the ^DF command**


In some cases, you might have been using files that contain the `^DF` command. The purpose of the `^DF`
command is to instruct the printer to store everything that comes after it in a ZPL file. For example, you
might have a file that contains the following:

```
    ^XA
    ^DFE:STOREFMT.ZPL^FS
    ^FO25,25^AD,36,20^FN1^FS
    ^FO165,25^AD,36,20^FN2^FS
    ^FO25,75^AB,22,14^FDBUILT BY^FS
    ^FO25,125^AE,28,15^FN1
    ^XZ

```

The purpose of the above format - when sent to a printer - would be to store a file called "STOREFMT" to
the E memory location on the printer. In production, the goal would be to recall and print the "STOREFMT"
file using the following ZPL commands:


1688


Mirror

```
    ^XA
    ^XFE:STOREFMT.ZPL^FS
    ^FN1^FDZEBRA^FS
    ^FN2^FDPRINTER^FS
    ^XZ

```

When this is done using Mirror, the format being sent to the printer must be altered. The line with the ^DF
command must be removed - this is because the Mirror process is taking care of storing the format to the E
memory location. In this scenario, the original format would be edited to look like this:

```
    ^FO25,25^AD,36,20^FN1^FS
    ^FO165,25^AD,36,20^FN2^FS
    ^FO25,75^AB,22,14^FDBUILT BY^FS
    ^FO25,125^AE,28,15^FN1

```

The character substitution described above must also be done on the file before it is stored in the
''<update-root>/files'' directory.

The `"recall"` format - using the `^XFE:STOREFMT.ZPL` command - does not need to be altered or
edited. It can be used as it was previously.


**Example Files**


Example of files that have already been altered in the manner described above are available as "Mirror File
Examples" at www.zebra.com.


To see an example file, right-click the paper-clip icon and select **Open File** or **Save Embedded File to Disk** .


**File Naming Recommendations**


Provides information on file naming recommendations

     - Files in the `"<update-root>/files"` directory and `"<update-root>/commands"` directory
should not have the same name.

     - Files in the `"<update-root>/files"` should not contain multiple label formats. If you need to Mirror
multiple formats, the recommended method is to split the formats into separate files.


**Command Use Recommendations**


Files in the `<update-root>/commands` directory should use only SGD commands or the following ZPL
commands.

     - `~CC`

     - `~CD`

     - `~CT`

     - `~JA`

     - `~JL`

     - `~JS`

     - `~JX`

     - `~RO`


1689


Mirror


Do not add a `device.reset` SGD command to the end of a file in the `<update-root>/commands`
directory. Mirror will reset itself automatically after performing an update, so there is no need for this
command.

## **Configuration**


This section provides detail on the configuring the printer and FTP server for Mirror.

### **Mirror FTP Server Configuration**


For a Zebra printer to successfully use Mirror, the Mirror FTP server must have the following:


For a Zebra printer to successfully use Mirror, the Mirror FTP server must have the following:


          - A client account (user name and password) for the printer to use


          - A root (base) directory for Mirror updates. This root directory must have the following subdirectories:

```
       /appl

       /files

       /commands

```

          - A Mirror feedback folder (optional)


**IMPORTANT:** The printer's FTP user account must have the necessary permissions to read/
write files in the update and feedback root directories. The `/appl`, `/files`, and `/commands`
subdirectories are read only; the Mirror feedback folder is read/write. If these permissions are not
properly set, the Mirror update and feedback processes will be unsuccessful.

### **Mirror Printer Configuration**


The following SGD commands are used to configure Mirror on the printer.


          - ip.mirror.auto


          - ip.mirror.error_retry


          - ip.mirror.feedback.auto


          - ip.mirror.feedback.freq


          - ip.mirror.feedback.odometer


          - ip.mirror.feedback.path


          - ip.mirror.fetch


          - ip.mirror.freq


          - ip.mirror.freq_hours


          - ip.mirror.last_error


          - ip.mirror.last_time


1690




Mirror


        - ip.mirror.password


        - ip.mirror.path


        - ip.mirror.reset_delay


        - ip.mirror.server


        - ip.mirror.success


        - ip.mirror.success_time on page 1287


        - ip.mirror.username


        - ip.mirror.version

### **The Feedback.get File**


The Feedback feature is one of the key benefits of the Mirror process. During a Mirror operation, the
printer can upload a file to the FTP Server that contains information about the configuration of the printer.
This information can then be leveraged by the Administrator to monitor the printer's setup. Using the
Feedback feature is optional.

The `"feedback.get"` file is a template file stored on the printer. It controls what content is uploaded to
the `"<feedback-root>"` directory on the FTP server. The directory on the FTP server where the printer
will send Feedback content to is controlled by the `"ip.mirror.feedback.path"` command.

Within the feedback.get file it is possible to leverage SGD commands to insert current printer status and
configuration strings into the file. This feature can help make the Feedback file on the FTP server more
unique and useful to the Administrator.


Additionally, the first line of the feedback.get file is used to control the name of the file that will be
uploaded and stored on the FTP server.

For example, if first line of the `"feedback.get"` file was:

```
     "zebra.<wlan.mac_raw>.<ip.mirror.feedback.odometer>"

```

That line would be evaluated by the printer and used as the Feedback destination file name to create on
the FTP server.


Using the example above, if the MAC address of the wireless print server was "00a0f8ae56d7"and the
Feedback odometer was currently at "33", the Feedback file created the FTP server would be named:

```
     "zebra.00a0f8ae56d7.33.txt"

```

The "feedback.get" file can be sent to the printer using the `!CISDFCRC16` command. For additional
information, see CISDFCRC16 Download Files on page 648.


**NOTE:** The first line of the feedback.get file is not included when the Feedback data is written to
FTP server.

### **Example Feedback.get file**


Here is an example of a "feedback.get" file and its resulting uploaded file:

```
     zebra.<wlan.mac_raw>.<ip.mirror.feedback.odometer>

```

1691


Mirror

```
     Application Name = <appl.name>
     Serial Number = <device.friendly_name>

     Mirror Success = <ip.mirror.success>
     Mirror Auto = <ip.mirror.auto>
     Mirror Path = <ip.mirror.path>
     Mirror Last Update = <ip.mirror.success_time>

     Bootp Enable = <ip.bootp.enable>
     DHCP Enable = <ip.dhcp.enable>
     Data Port = <ip.port>
     Associated AP = <wlan.bssid>
     RF ESSID = <wlan.essid>
     RF Firmware = <wlan.firmware_version>
     RF Signal Strength = <wlan.signal_strength>
     RF Channel Mask = <wlan.channel_mask>

     Label Length = <odometer.total_print_length>
     Print Length = <odometer.label_dot_length>
     When this file is processed by the printer and uploaded to the FTP server,
     the resulting upload file on the FTP server would contain data similar to
     this:
     Application Name = V53.17.2Z
     Serial Number = ZBR123456

     Mirror Success = Yes
     Mirror Auto = on
     Mirror Path = /update-root/ZM400/
     Mirror Last Update = 12345678

     Bootp Enable = on
     DHCP Enable = on
     Data Port = 9100
     Associated AP = 124
     RF ESSID = MyEssid
     RF Firmware = 5.2.1
     RF Signal Strength = 98
     RF Channel Mask = FF

     Label Length = 100
     Print Length = 200

### **How to Set Up and Use Mirror**

```

This section provides multiple scenarios which include specific examples that demonstrate how to set up
and use Mirror.


**Scenario One**


In this scenario, the printer is configured to perform a Mirror update operation (" `ip.mirror.auto =`
`on` ") and feedback operation (" `ip.mirror.feedback.auto = on` ") every time the printer restarts.


1692


Mirror


On startup, after a network connection has been established, the printer will attempt to make a FTP
connection to the server address 10.14.5.133, using the “user name” and “password” of the printer.


If the connection is successful, the printer will attempt to perform an update operation using the root
directory " `/all_printers/s4m/role1` ". After the update operation is complete, the printer will attempt
a Feedback operation, uploading the resulting Feedback file to the " `/all_feedback` " directory on the
server.


Using the command set in the example below, the printer will not attempt any periodic Mirror Update or
Feedback operations. It will only perform Mirror operations on startup or when explicitly instructed to using
the " `ip.mirror.fetch` " command.


**NOTE:** If a file starts with "/" it signifies the base directory of that file system. If a file is contained
in the user’s account, they do not start with a "/".


This example shows a Mirror configuration command set. Each line item of the command set is identified
with a number. For details on each line item, see the table below.

```
    10 ! U1 SETVAR "ip.mirror.auto" "on"
    20 ! U1 SETVAR "ip.mirror.username" "printer"
    30 ! U1 SETVAR "ip.mirror.password" "printer"
    40 ! U1 SETVAR "ip.mirror.server" "10.14.5.133"
    50 ! U1 SETVAR "ip.mirror.path" "/all_printers/s4m/role1"
    60 ! U1 SETVAR "ip.mirror.feedback.auto" "on"
    70 ! U1 SETVAR "ip.mirror.feedback.path" "/all_feedback"
    80 ! U1 SETVAR "ip.mirror.feedback.freq" "0"

```

10 Configures the printer to perform a Mirror Update operation at power-up.

20 Configures the FTP Server "user name" for the printer to use

30 Configures the FTP Server "password" for the printer to use

40 Configures the FTP server address the printer should make a FTP connection to.

50 If the FTP connection is successful, the printer should attempt to perform an Update operation
using this root directory.

60 Configures a printer to automatically perform a Mirror Feedback operation at start-up.

70 Configures the printer to upload the resulting Feedback file to the designated directory on the
server.

80 Configures a printer to repeat the Feedback operation zero times.


**Scenario Two**


In this scenario, the printer is configured to not perform a Mirror Update function at start-up. It is configured
to perform a Mirror Feedback operation at start-up and thereafter at every 60 minutes.


When the 60 minutes elapses, the printer will attempt to make a FTP connection to the server address
10.14.5.133. If the FTP connection is successful, the printer will attempt a Feedback operation, uploading
the resulting Feedback file to the "/all_feedback" directory on the server. After the initial Feedback
operation, subsequent Feedback operations will occur at an interval of 60 minutes.


**IMPORTANT:** Using the command set in the example below, for the printer to attempt any Mirror
Update operation unless the `"ip.mirror.fetch"` command is sent to the printer.


1693


Mirror


This example shows a Mirror configuration command set. Each line item of the command set is identified
with a number. For details on each line item, see the table below.

```
    10 ! U1 SETVAR "ip.mirror.auto" "off"
    20 ! U1 SETVAR "ip.mirror.username" "printer"
    30 ! U1 SETVAR "ip.mirror.password" "printer"
    40 ! U1 SETVAR "ip.mirror.server" "10.14.5.133"
    50 ! U1 SETVAR "ip.mirror.path" "/all_printers/s4m/role1"
    60 ! U1 SETVAR "ip.mirror.feedback.auto" "on"
    70 ! U1 SETVAR "ip.mirror.feedback.path" "/all_feedback"
    80 ! U1 SETVAR "ip.mirror.feedback.freq" "60"

```

10 Configures the printer to not perform a Mirror Update operation at start-up

20 Configures the FTP Server "user name" for the printer to use

30 Configures the FTP Server "password" for the printer to use

40 Configures the FTP server address the printer should make a FTP connection to.

50 If the FTP connection is successful, the printer should attempt to perform an Update operation
using this root directory.

60 Configures a printer to automatically perform a Mirror Feedback operation at start-up.

70 Configures the printer to upload the resulting Feedback file to the designated directory on the
server.

80 Configures the printer to attempt a Feedback operation every 60 minutes.


**Troubleshooting**


If a Mirror process completes unsuccessfully, troubleshooting information can be retrieved by sending this
command to the printer:

```
    ! U1 GETVAR "ip.mirror.last_error"

```

Table 41  Printer Response Troubleshooting on page 1695 lists possible printer responses, an
explanation of each, and resolutions. Table 42  Problem Scenario Troubleshooting on page 1697
provides problem scenarios and solutions.


1694


Mirror


**IMPORTANT:** A mirror path can have up to 50 characters.


**Table 41** Printer Response Troubleshooting







|Printer Response|Explanation|Resolution|
|---|---|---|
|`"connection failed"`|The network connection to the<br>Mirror FTP server failed while<br>attempting to perform a printer<br>update.|•<br>Check the user name,<br>password, and server<br>address for the Mirror FTP<br>server and ensure that these<br>values are set correctly in the<br>printer.<br>•<br>Ensure that the user name<br>assigned to the printer has<br>the proper permission to log<br>into the Mirror FTP server.<br>•<br>Check that the printer<br>has a successful network<br>connection and is able to<br>send and receive network<br>data.|
|`"Failed to get File:`<br>`[filename]"`|During an update operation, the<br>printer's attempt to retrieve the<br>file [filename] failed.|•<br>Ensure that the printer's<br>network connection has<br>not been interrupted. If it<br>has, re-establish network<br>connectivity and retry the<br>update.<br>•<br>Check the server's access<br>permissions for the user<br>name assigned to the printer.<br>Make sure the user name<br>is granted access to read<br>[filename] from the server.|
|`"feedback connection`<br>`failed"`|The network connection to<br>the Mirror FTP server failed<br>while attempting to send printer<br>feedback.|•<br>Check the user name,<br>password, and server<br>address for the Mirror FTP<br>server and ensure that these<br>values are set correctly in the<br>printer.<br>•<br>Ensure that the user name<br>assigned to the printer have<br>the proper permission to log<br>into the Mirror FTP server.<br>•<br>Check that the printer<br>has a successful network<br>connection and is able to<br>send and receive network<br>data.|


1695




Mirror


**Table 41** Printer Response Troubleshooting (Continued)





|Printer Response|Explanation|Resolution|
|---|---|---|
|`"Failed getting file to`<br>`parser : [filename]"`|During an update operation,<br>the printer’s attempt to<br>retrieve a file [filename] in<br>the`<mirror_path>/appl`<br>directory failed.|•<br>Ensure that the printer’s<br>network connection has<br>not been interrupted. If it<br>has, re-establish network<br>connectivity and retry the<br>update.<br>•<br>Check the server’s access<br>permissions for the user<br>name assigned to the printer.<br>Make sure the user name<br>is granted access to read<br>`[filename]` from the<br>server.|
|`"Failed to send feedback`<br>`file: <feedback_path>/`<br>`<feedback_filename>"`|During a feedback operation,<br>the printer’s attempt to store<br>the feedback file in the<br>`<feedback_path>` directory<br>failed.|•<br>Ensure that the printer’s<br>network connection has<br>not been interrupted. If it<br>has, re-establish network<br>connectivity and retry the<br>update.<br>•<br>Check the server’s access<br>permissions for the user<br>name assigned to the printer.<br>Make sure the user name<br>is granted access to write<br>to the`<feedback_path>`<br>directory.<br>•<br>Ensure that the<br>`<feedback_path>`<br>directory exists on the remote<br>server.|
|`"Too many files in`<br>`the firmware download`<br>`directory"`|The Mirror FTP server has<br>more than one file in the<br>`<mirror_path>/appl`<br>directory.|Ensure that there is only<br>one (1) firmware file in the<br>`<mirror_path>/appl`<br>directory.|


1696




Mirror


**Table 42** Problem Scenario Troubleshooting







|Problem Scenario|Solution|
|---|---|
|I performed a Mirror Update and<br>now my printer is continuously<br>reprogramming.|Ensure that the name of the firmware file in`<mirror_path>/appl`<br>matches the version of firmware contained in that file.<br>For firmware version V53.17.2Z, the name of the file in the<br>`<mirror_path>/appl` directory must be`V53.17.2Z.ZPL` to<br>prevent the continuous reprogramming cycle.|
|Every time a Mirror Update is<br>run, a file is fetched even though<br>no changes have been made to<br>the files on the server.|•<br>Check the names of the files in the`<mirror_path>/files`<br>and`<mirror_path>/commands` directories. If the names are<br>longer than 16 characters (minus extensions), then the printer<br>will truncate them to 16 characters when downloading. If two<br>filenames truncate to the same 16 characters, the printer will not<br>be able to tell the difference between them and will re-download<br>one of the two files during every update operation.<br>•<br>Check the names of the files in the`<mirror_path>/files`<br>and`<mirror_path>/commands` directories. If any of the<br>names are the same, then the printer will not be able to tell the<br>difference. Therefore it will get the one in the files directory,<br>update the timestamp file, and then it will get the one in the<br>commands directory and update the timestamp file. Then,<br>the next time through the one in the files directory will have a<br>different timestamp, so it will get that file again and then check<br>the commands directory, and so on.<br>•<br>Check the server's access permissions for the user name<br>assigned to the printer. Make sure the user name is granted<br>access to read all files in the`<mirror_path>/files` and<br>`<mirror_path>/commands` directories.|
|The printer is continually<br>rebooting.|•<br>Check the` /commands` directory. If a`file.delete` command<br>resides, then you need to remove`file.delete` or modify your<br>script.|


1697


# **Wireless Markup Language** **(WML)**

**Wireless Markup Language (WML)**


Wireless Markup Language (WML) offers a text-based method of designing a menu structure for the display
screen of selected printers. By leveraging Set/Get/Do (SGD) and files containing Zebra Programming
Language (ZPL) commands, customized menus can be created.

## **WML Overview**


Wireless Markup Language (WML) offers a text-based method of designing customized menus on the LCD
front panel of selected printers.


By leveraging Set-Get-Do (SGD) and Zebra Programming Language (ZPL) commands, menus that feature
both display and command features can be created. The WML “card” structure makes it possible to link
from one menu screen to another, creating menus that are as many levels “deep” as desired or reduced to
only those options needed by the printer operator.


For details on SGD commands, see SGD Command Support. For details on ZPL commands, see ZPL
Commands.

## **WML Details**


A WML file is made up of tags, which are similar to HTML tags. For a list of the supported WML tags, see
WML Tags.

Using WML on the printer is dependent on the presence of a single `index.wml` file, stored in the printer's
E: memory. The `index.wml` file can contain one or more “cards”, with each card defining the content of
a single menu. Everything within the card tag (<card> </card>) constitutes one complete front panel menu.
Cards can also contain hyperlinks to other menus. If the index.wml has three cards, with links between the
cards, that means there are three front panel menus. It is also possible to create multiple .wml files, with
links between them and the index.wml file. In cases where multiple .wml files are used, it is recommended
that each file should be structured to provide a link back to the main menu as described in the index.wml
card.


**NOTE:** Only one `index.wml` file can reside on a printer at any time


WML defined menus can use Set-Get-Do (SGD) commands to retrieve or set printer settings. For example,
a menu might display the printer’s current baud rate, while also offering other potential baud rate settings
for the printer selection. In more advanced uses, WML defined menus can cause ZPL command files,
stored in the printer E: memory, to be injected into the printers command engine – where they will be read
in and acted upon. In this use, the ZPL command file files are known as .nrd files.


1698


Wireless Markup Language (WML)


For example, a WML defined menu could call an .nrd file that contains a customized set of printer
configuration commands. In this way, different profiles can be created for the printer - making it possible for
the printer operator to select the appropriate configuration profile needed for the task the printer is being
used in.


An important concept to consider is that the WML menu completely defines what is displayed on the
printers screen. If an item is not included in the WML menu definition it will not be displayed to the user.


**NOTE:** The `index.wml` file must reside on the printer’s `E:` drive for the WML menu to display. If
the `index.wml` file is on a drive other than `E:`, then the standard front panel menus display. 3


**NOTE:** When a WML menu is resident on the printer, the standard menu system can be easily
be accessed by holding down the Cancel and Setup/Exit buttons (on the ZM400) or the Cancel
and Setup/Exit buttons (on Xi4) or the Select button (on GX) on the front panel while the printer
powers up. Hold the buttons down until the PRINT READY message displays on the front panel.
To return to the WML defined menu, reset the printer again.3

## **Supported Printers**


WML is supported on the following printers, using the indicated firmware. The buttons on the printers' front
panel that are used for Navigating WML defined menus are noted.


**NOTE:** When a WML defined menu is in use, the stripes pattern found at the top of selected
printers is not displayed.


**Table 43** WML-Supported Printers











|Printer|Firmware|Number<br>of “lines”<br>available|Menu Navigation<br>Buttons|Keys to Access<br>Standard<br>Menu System|
|---|---|---|---|---|
|105SLPlus|V53.17.15Z (or later)|5|Select + (PLUS) -<br>(MINUS)|Hold down CANCEL<br>& SETUP/EXIT during<br>power-up|
|Xi4 series|V53.17.5Z (or later)|5|Select + (PLUS) -<br>(MINUS)|Hold down CANCEL<br>& SETUP/EXIT during<br>power-up|
|ZE500|V53.17.15Z (or later)|5|Select + (PLUS) -<br>(MINUS)|Hold down CANCEL<br>& SETUP/EXIT during<br>power-up|
|ZM400|V53.17.5Z (or later)|5|Select + (PLUS) -<br>(MINUS)|Hold down CANCEL<br>& SETUP/EXIT during<br>power-up|
|ZM600|V53.17.5Z (or later)|5|NEXT/SAVE + (PLUS) -<br>(MINUS)|Hold down CANCEL<br>& SETUP/EXIT during<br>power-up|
|G series|V53.17.5Z (or later)|4|SELECT SCROLL|Hold down SELECT<br>during power-up|


1699


Wireless Markup Language (WML)

## **Professional Services for WML Content Creation**


Zebra offers a Professional Services group that can help with the creation of WML content. To inquire
about Zebra’s Professional Services, contact your Zebra account representative.

## **WML Tags**


This section lists the WML tags and tag parameters that can be used to create a menu system.


Table 1 shows the WML tags and tag parameters that can be used to create a menu system. As with
other tag-based languages, such as HTML and XML, ending tags should be used to indicate the end of a
structure. An example of an ending tag would be </wml>, which indicates the end of a WML script.


**IMPORTANT:** Using end tags is required to create well formed and functional WML scripts.



|Table 44 WML Tag Descriptions|Col2|
|---|---|
|`<wml> </wml>`|indicates the beginning/end of the WML script|
|`<display> </display>`|indicates the beginning/end of the content to display on-screen|
|`<card> </card>`|indicates the beginning/end of a card|
|`<p> </p>`|indicates the beginning/end of a Paragraph|
|`<br/>`|Line break|
|`<a href="#menu">Menu</a>`|Hyperlink to another card|
|`<timer value="xx"> </timer>`|Controls display timer in 10th of a second increments|
|`" ontimer="#main"`|Controls action to take at timer end|
|`alerts="on"`|Controls display of on-screen alerts|
|`$(command.command)`|$ executes a SGD “get” command|
|`<do .....><setvar .......></`<br>`do>`|Controls execution of do and setvar commands|

## **Using WML**





This section provides you with the necessary steps to prepare and transmit WML content to the
printer. There are two methods to send WML content to the printer - via the FTP protocol or using the
“CISDFCRC16” command. Both methods are detailed below.


**IMPORTANT:** The & (ampersand) character should not be used within the body of any Paragraph
tag (<p>). If an ampersand is present within the body of a Paragraph tag, a WML-based menu may
not function as expected. The ampersand character should NEVER be used within a paragraph
tag for any of the printer's soft keys (P1, P2, etc.); doing so can render the menu inoperable.

### **Create a Sample index.wml File**


This procedure shows how to create an index.xml file.


**1.** Open a text editor.


1700




Wireless Markup Language (WML)


**2.** Type (or copy/paste) the following text:

```
       <wml>
       <display>
       <card>
       <p>Hello World!!</p>
       </card>
       </display>
       </wml>

```

Save this file with this name: `index.wml` .

**3.** To confirm these commands are correctly set, send the `getvar` command to check the settings. To do
this, send these commands to the printer:

## **Prepare the Printer to Receive WML Content via FTP**


NEED


WML files – and any .nrd files used by a WML menu structure – must be stored in the printers E: memory
location. While the files are first being transmitted to the printer, they should not be processed by the
printers ZPL formatting engine. This can be done by configuring the SGD settings `"ip.ftp.enable"` and
`"ip.ftp.execute_file"` .

The `ip.ftp.enable"` setting allows the printer to receive content via the FTP protocol. The
`“ip.ftp.execute_file”` setting controls the printers’ ability to process or not process commands
received via the FTP protocol using the printers ZPL engine. By default, both settings are enabled.

Set `"ip.ftp.enable"` to `"on"` and `"ip.ftp.execute_file"` to `"off"` .

To do this, send these commands to the printer:

```
     ! U1 setvar "ip.ftp.enable" "on"
     ! U1 setvar "ip.ftp.execute_file" "off"
     ! U1 getvar "ip.ftp.enable"
     ! U1 getvar "ip.ftp.execute_file"

```

To confirm these commands are correctly set, send the `getvar` command to check the settings. To do
this, send these commands to the printer:

```
     ! U1 getvar "ip.ftp.enable"
     ! U1 getvar "ip.ftp.execute_file"

```

If a terminal emulation program is being used, the following response should be returned from the printer.

```
     "on""off"

```

**NOTE:** Only printers using the Internal 10/100 wired or Internal Wireless Plus & Wireless Plus print
server can use the `! U1 setvar "ip.ftp.execute_file" "off"` command. For other
print servers, use the `“CISDFCRC16”` command method detailed below.


1701


Wireless Markup Language (WML)