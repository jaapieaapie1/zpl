# Introduction


**Introduction**


This guide is the unabridged, alphabetical reference of programming commands supported in the
firmware. This includes all ZPL commands and SGD commands.


**IMPORTANT:** These are important points to note when using ZPL and SGD commands:


          - ZPL and SGD commands should be sent to the printer as separate files.


          - Certain settings can be controlled by both ZPL and SGD. Configuration changes made in ZPL can affect
configuration changes made in SGD.


To contact Zebra or for technical support, visit www.zebra.com/contact.

## **Firmware**


You can find the printer’s firmware version by printing a configuration label. For instructions to do so, see
your printer’s user guide. For firmware upgrades go to: www.zebra.com/firmware.


**IMPORTANT:** These are important points to note when using a Zebra G-Series printer:


          - You can send instructions to the printer using multiple programming languages: EPL, ZPL, or SGD.
EPL and ZPL commands configure the printer, print labels, and get device status information. SGD
commands set and get configuration details. These three languages can be used without the need to
send the printer instructions to switch from one language to another.


          - EPL, ZPL, and SGD commands must be sent to the printer as separate files. They cannot be used
together in one format, or set of commands. For example, if you send a series of SGD commands to the
printer and they are followed by a printable format, this needs to be done using separate files.


Many text editors and word processors can recreate most examples in this guide in ASCII format. However,
for other encodings such as Unicode, a text editor such as Microsoft Notepad is needed.

## **Who Should Use This Document**


This is for programmers who are familiar working with programming languages.


47