# Error Detection Protocol


**Error Detection Protocol**


This section explains the Zebra protocol that has been supplanted in TCP/IP based applications because of
the error detection compatibility inherent in the TCP/IP protocol.

## **Introduction**


There are many instances when it is vitally important that the information sent to the Zebra printer is
received completely Error-Free. ZPL II supports an error detection protocol called Zebra Packet Response
Protocol to meet this need.


**NOTE:** This protocol only works when using serial interface. It does not function when using
parallel interface.

### **What is a Protocol?**


A protocol is a precisely defined set of rules. In the case of data communications, a Protocol defines how
data is transmitted, received, and acknowledged between two devices.


The sole purpose of the Packet Response Protocol is to ensure that the information sent from a Host
computer to the Zebra printer is received accurately. Remember, the protocol cannot insure the accuracy
of the data that is actually sent from the Host computer. The commands and data needed to make a label
(ZPL II Format) are encapsulated within the information sent from the Host computer.

### **How Protocols Work**


The basic unit of data transfer in the Packet Response Protocol is called a “Transaction.” A Transaction
is a two-way communication procedure that consists of information being sent from the Host computer
to the Zebra printer, and the printer sending back a response to the Host computer. This response is
an indication that the Zebra printer has either accepted or rejected the information sent from the Host
computer.


Information is sent in the form of “Packets.” Packets sent from the Host computer are called Request
Packets.


When a Request Packet is received, the Zebra printer analyzes the information in the Packet. If the Request
Packet is accepted, the Zebra printer will send a positive response back to the Host computer. The Host
computer can then send the next Request Packet. If the information is rejected, the Zebra printer will send
a negative response back to the Host computer. The Host computer then sends the same Request Packet
again.


1576


Error Detection Protocol


The Zebra Packet Response Protocol can be used in both single-printer applications, where there is only
one Zebra printer connected to the Host computer, and multi-drop systems in which several Zebra printers
are connected to the same Host computer.

## **Request Packet Formats from the Host Computer**


The first part of each data transfer Transaction is the sending of a Request Packet by the Host computer.
The Request Packet contains a fixed length “Header” block and a variable length “Data” block. Each
Packet sent from the Host computer to the Zebra printer must always use the following format.


The Request Packet Header Block is comprised of five fixed-length fields. The Request Packet Data Block
is comprised of four fixed-length fields and one variable-length field. These fields are defined as follows.








|Header Block|Col2|Col3|Col4|Col5|Data Block|Col7|Col8|Col9|Col10|
|---|---|---|---|---|---|---|---|---|---|
|**SOH**|**DST.**~~**Z-ID**~~|**SRC.**~~**Z-ID**~~|**TYPE**|**SEQ. #**|**STX**|**FORMAT**|**EXT**|**CRC**|**EOT**|
|1|3|3|1|1|1|≤1024|1|2|1|


### **Header Block Fields**

This section provides a list of header block fields.


        - SOH (start of header character)


The Zebra printer interprets this character as the beginning of a new Request Packet. The ASCII Control
Code character SOH (01H) is used as the Start of Header Character.


        - DST. Z-ID (destination Zebra-ID)


This is the three-digit ASCII I.D. number used to identify which Zebra printer is to receive the Request
Packet. The Zebra printer compares this number to the Network ID number assigned to it during Printer
Configuration. The Zebra printer will act on the Request Packet only if these numbers match.


        - SRC. Z-ID (source Zebra-ID)


This is a three-digit ASCII number used to identify the Host computer. This number is determined by the
user.


        - TYPE (packet type)


This field is used to define the type of Request Packet being sent by the Host. Only two characters are
valid in this field:


          - ‘P’ indicates a Print Request Packet


          - ‘I’ indicates an Initialize Request Packet


Most of the Packets sent by the Host to the Zebra printer will be of the ‘P’ variety, requesting a label to
be printed.


The ‘I’ character tells the Zebra printer to initialize the packet sequence numbering. It is required in the
first packet of a new printing session, after starting up the Host computer or the Zebra printer.


        - SEQ. # (the sequence number of the request packet)


This block contains a single digit number used to denote the current Transaction Number. The Host
computer must increment this number by “1" for each new Request/Response Transaction pair, i.e. 0, 1,
2,..., 9. The numbers repeat after every 10 Transactions.


1577


Error Detection Protocol

### **Data Block Fields**


This section provides a list of data block fields.


          - STX (Start of Text)


The Zebra printer interprets this character as the beginning of the variable-length Data Format portion
of the Request Packet. The ASCII Control Code character STX (02H) is used as the Start of Text
Character.


          - DATA FORMAT (Label Information)


A variable-length portion of the Request Packet that contains the complete or partial ZPL II label format,
or partial data string (such as a downloaded graphic).


This field can contain from 0 to 1024 characters. If the Format of a label is longer than 1024 characters,
the Data Format fields from consecutive packets will be concatenated together in the printer’s Receive
Data Buffer as if they were sent as one long direct transmission.


Special consideration has been given to the possible requirement to include ASCII Control Characters
(values less than 20H) in the Data Format portion of a Request Packet. Characters such as EOT (04H),
STX (02H), SOH (01H), and ETX (03H), are part of the Error Detection Protocol and could interrupt
normal communication procedures if received at the wrong time.


          - ETX (End of Text)


The Zebra printer interprets this character as the end of the variable length Data Format portion of the
Request Packet. The ASCII Control Code character ETX (03H) is used as the End of Text Character.


          - CRC (Cyclic Redundancy Check)


The CRC is a 2 character field. A Cyclic Redundancy Check is a type of error checking used to maintain
the validity and integrity of the information transmitted between the Host computer and the Zebra
printer. This Protocol uses the 16-bit CCITT method of producing a CRC.


The CRC is a two-byte value derived from the contents of the packet between, but not including, the
SOH character and the CRC code itself. The Zebra printer will calculate a CRC of the Request Packet
received and compare the value with the CRC Value in this field. The CRC of the Request Packet must
match the CRC calculated by the Zebra printer in order for the Request Packet to be valid.


          - EOT (End of Transmission)


The Zebra printer interprets this character as the end of the Request Packet. The ASCII Control Code
character EOT (04H) is used as the End of Transmission Character.

## **Response From the Zebra Printer**


When the Zebra printer receives the EOT character, it will begin acting on the Request Packet received.
The printer will compare certain characters and numeric values within the received Request Packet and
send a response back to the Host computer.

### **Zebra Packet Response**


The Packet Response protocol provides the highest degree of error checking and is well suited to the
Host-Multiple Printer application. The Response Packet from the Zebra printer will always use the following
format.


The Request Packet Header Block is comprised of five fixed-length fields. The Request Packet Data Block
is comprised of four fixed-length fields and one variable-length field. These fields are defined as follows.


1578


Error Detection Protocol








|Header Block|Col2|Col3|Col4|Col5|Data Block|Col7|Col8|Col9|Col10|
|---|---|---|---|---|---|---|---|---|---|
|**SOH**|**DST.**~~**Z-ID**~~|**SRC.**~~**Z-ID**~~|**TYPE**|**SEQ. #**|**STX**|**FORMAT**|**EXT**|**CRC**|**EOT**|
|1|3|3|1|1|1|≤1024|1|2|1|


### **Header Block Fields**

This section provides descriptions for the header block fields.


        - SOH (Start of Header Character)


The Zebra printer sends this character as the beginning of a new Response Packet. The ASCII Control
Code character SOH (01H) is used as the Start of Header Character.


        - DST. Z-ID (Destination Zebra-ID)


This is the same three-digit ASCII number used to identify the Host Computer that was contained in
the SRC. Z-ID field of the Request Packet that initiated this Response Packet. The Host compares this
number to its known value to insure it is the proper destination.


        - SRC. Z-ID (Source Zebra-ID)


This is the three character ASCII Network I.D. of the Zebra printer that is sending the Response Packet.


        - TYPE (Packet Type)


This block is used to define the type of Response Packet being sent to the Host. Only three characters
are valid in this field.


          - ‘A’ This is a Positive Acknowledgment to the Host computer. It indicates that the Request Packet was
received without a CRC error. The Host computer may send the next Request Packet.


          - ‘N’ This is the Negative Acknowledgment to the Host computer. It indicates that an error was
detected in the packet sent from the Host computer. The Host computer must retransmit the same
Request Packet again.


          - ‘S’ This character indicates that the Response Packet contains the Zebra Printer Status requested by
a `~HS` (Host Status) command received from the Host.

        - SEQ. # (Used to denote the current message sequence number)


This number is identical to the message sequence number in the Request Packet. It denotes the
message sequence number to which the Response Packet is replying.

### **Data Block Fields**


This section provides descriptions for the data block fields.


        - STX (Start of Text)


The Zebra printer sends this character as the beginning of the variable length Data Format portion
of the Response Packet. The ASCII Control Code character STX (02H) is used as the Start of Text
Character.


        - DATA FORMAT (Label Information)


The ‘variable length’ portion of the Response Packet. If the Packet Type field in the Response Header
contains an ‘A’ or an ‘N’, no data will appear in this field. If the Packet Type field contains an ‘S‘, this field
will contain the Printer Status Message.


1579


Error Detection Protocol


        - ETX (End of Text)


The Zebra printer sends this character as the end of the variable length Data Format portion of the
Request Packet. The ASCII Control Code character ETX (03H) is used as the End of Text Character.


        - CRC (Cyclic Redundancy Check)


This is the CRC of the Response Packet as calculated by the Zebra printer. This Cyclic Redundancy
Check maintains the validity and integrity of the information transmitted between the Zebra printer and
the Host computer.


This CRC is a two Byte value derived from the contents of the packet between, but not including,
the SOH character and the CRC code itself. The Host computer will calculate a CRC of the received
Response Packet and compare it to the CRC value in this field. The CRC of the Response Packet must
match the CRC calculated by the Host computer in order for the Response Packet to be valid.


        - EOT (End of Transmission)


The Zebra printer sends this character as the end of the Response Packet. The ASCII Control Code
character EOT (04H) is used as the End of Transmission Character.

### **Disguising Control Code Characters**


There may be occasions when ASCII Control Codes (00H - 19H) must be included as part of the Data
Format block of a Request Packet. To eliminate any problems, these characters must be disguised so that
the communication protocol does not act on them.


This procedure must be used to disguise each Control Code.


        - A SUB (1AH) character must precede each Control Code placed in the Data Format block.


        - The value of 40H must be added to the Hex value of the Control Code.


        - The ASCII Character corresponding to the total value produced in step 2 must be entered in the Data
Format right after the SUB character.


The Zebra printer automatically converts the modified control character back to its correct value by
discarding the SUB (1AH) character and subtracting 40H from the next character.


To include a DLE (10H) character in the Data Format block:


1. Enter a SUB (1AH) character into the Data Format.


2. Add 40H to the DLE value of 10H for a resulting value of 50H.


3. Enter the ASCII character “P” (50H) in the Data Format after the SUB character.


**NOTE:** This technique is counted as two characters of the 1024 allowed in the Data Format block.


**Rules for Transactions**


This section lists the rules for transactions.


        - Every Transaction is independent of every other Transaction and can only be initiated by the Host
computer.


        - A valid Response Packet must be received by the Host computer to complete a Transaction before the
next Request Packet is sent.


        - If an error is encountered during a Transaction, the entire Transaction (i.e., Request Packet and
Response Packet) must be repeated.


1580




Error Detection Protocol


        - The Zebra printer does not provide for system time-outs and has no responsibility for insuring that its
Response Packets are received by the Host computer.


        - The Host computer must provide time-outs for all of the Transactions and insure that communication
continues.


        - If any part of a Transaction is lost or received incorrectly, it is the responsibility of the Host computer to
retry the whole Transaction.

### **Error Detection Protocol Application**


The following are the basic requirements for setting up the Zebra printer to use the Error Detection
Protocol.


Activating the Protocol

Protocol is a front panel selection, or can be done with the ZPL command `^SC` .

Setting Up Communications


Insure that the Host computer and the Zebra printer are characterized with the same communication
parameters; i.e., Parity, Baud Rate, etc. The communications must be set up for 8 data bits.


Setting the Printer ID Number


The Protocol uses the printer’s Network ID number to insure communication with the proper unit. The
Network ID is programmed into the printer by sending the printer a `^NI` (Network ID Number) command or
done through the front panel.


If there is only one printer connected to the Host computer, the Network ID number should be set to all
zeros (default).


If there is more than one printer, such as in a broadcast or multi-drop environment, each printer should be
assigned its own unique ID number. Printers in this environment, with an ID of all zeros, will receive ALL
label formats regardless of the actual printer ID number in the DST. Z-ID block of the Request Packet.

### **Error Conditions and System Faults**


This section describes error conditions and system faults that might occur.


**Restarting a Transmission**


If a break in communication occurs, the Host must restart the transmission of the current label format
with an Initialization Request Packet. The Zebra printer will not respond to Request Packets sent out
of sequence. However, the Zebra printer will respond to an Initialization Request Packet and restart its
internal counting with the sequence number of the Request Packet.


**CRC Error Conditions and Responses**


A CRC error condition can be detected when the printer receives a Request Packet or when the Host
computer receives a Response Packet. The following list defines these errors and how the Host computer
should respond to them.



|Error|Response|
|---|---|
|The CRC calculated by the Zebra printer<br>does not match the one received as part of<br>the Request Packet.|The Zebra printer will return a Negative Acknowledgment<br>Response Packet. The Host computer should retry the<br>same Transaction with the same Sequence Number.|


1581




Error Detection Protocol



|Error|Response|
|---|---|
|The CRC calculated by the Host computer<br>does not match the one received as part of<br>the Response Packet.|The Host computer should retry the same Transaction with<br>the same Sequence Number.|


**Time-Out Error Conditions and Responses**





There are certain conditions at the Zebra printer that might cause the Host computer to time-out while
processing a Transaction. The following list illustrates these conditions and how the Host computer should
respond to them.






|Error|Response|
|---|---|
|A Request Packet from the Host computer<br>is not received by the Zebra printer.|The Host computer times out and resends the Request<br>Packet of the same Transaction with the same Sequence<br>Number.|
|A Request Packet from the Host computer<br>is partially received by the Zebra printer.|The Host computer times out and resends the Request<br>Packet of the same Transaction with the same Sequence<br>Number.|
|A Response Packet from the Zebra printer<br>is not received by the Host computer.|The Host computer times out and resends the Request<br>Packet of the same Transaction with the same Sequence<br>Number.|
|A Response Packet from the Zebra printer<br>is partially received by the Host computer.|The Host computer times out and resends the Request<br>Packet of the same Transaction with the same Sequence<br>Number.|


### **How the Zebra Printer Processes a Request Packet**

The following describes the steps taken at the Zebra printer to process a Request Packet.


**1.** The Zebra printer looks for a SOH (Start of Header) character. As soon as it finds one, it places the SOH
and all the data after it into its Receive Data Buffer. This process continues until the printer receives an
EOT (End of Transmission) character.


**NOTE:** If a second SOH is received before an EOT is detected, the contents of the Receive
Buffer will be discarded. All of the data after the second SOH will be placed in the Receive
Data Buffer.


**2.** After detecting the EOT, the printer checks for the following:


        - The DST. Z-ID matches the printer’s Network I.D.


**NOTE:** If the Network ID at the printer is all zeros, the printer will accept all Request Packets
regardless of the DST. Z-ID received. If a Request Packet is received with the DST. Z-ID all
zeros, it is accepted by all printers regardless of their Network ID setting.


*The Data Format begins with STX and ends with ETX.


*The Sequence Number has not been used before.


If the check is satisfactory, proceed to Step 3 on the following page.


If any part of the check is unsatisfactory, the printer discards the data in its Receive Data Buffer and
waits for another SOH. No response is sent to the computer.


1582


Error Detection Protocol


**Exceptions**


It is possible that the printer will send a response to the host that the host does not receive. Therefore, the
host will send the same request packet to the printer again. If this happens, the printer will not use the data
if it already used it before. However, the printer will send a response back to the host.


The printer calculates the CRC and compares it with the one received in the Request Packet. If the CRC
is valid, the printer sends a Positive Response Packet to the Host computer. It then transfers the ‘Variable
Length’ data from the Receive Buffer to its memory for processing. If the CRC does not match, and the
printer is set up to return a Negative Response Packet, the following will take place:


**1.** The printer assumes that the DST. Z-ID, SRC. Z-ID, and Sequence Number are correct and that the error
was in the variable data.


**2.** The same DST. Z-ID, printers SRC. Z-ID, and Sequence Number will be returned back to the host in the
Negative Response Packet.


**3.** If the assumption in (a) is incorrect, the Host computer can time-out and retransmit the original Request
Packet.

### **How the Zebra Printer Responds to Host Status**


The following describes how the Zebra printer to responds to host status command.

If a `~HS` (Host Status) command is received by the Zebra printer, the printer will send back an
acknowledgment for the receipt of the packet. It then sends an additional packet that includes the Host
Status information in the Variable Length portion of the packet.


1583


# **ZB64 Encoding and** **Compression**

**ZB64 Encoding and Compression**


This section describes the Base 64 MIME (ZB64) encoding and compression.


This is the same type of MIME encoding that is used in e-mail.


For more information on ZB64 Encoding and Compression, contact your Reseller or Zebra Representative.

## **Introduction to B64 and Z64**


The first encoding, known as B64, encodes the data using the MIME Base64 scheme. Base64 is used to
encode e-mail attachments and is specifically designed to address communications path limitations, such
as control characters and 7-bit data links.


It encodes the data using only the printable ASCII characters:


With the use of ZPL, this has the added benefit of avoiding the caret ( `^` ) and tilde (~) characters. Base64
encodes six bits to the byte, for an expansion of 33 percent over the un-enclosed data. This is much better
than the 100 percent expansion given by the existing ASCII hexadecimal encoding.


The second encoding, known as Z64, first compresses the data using the LZ77 algorithm to reduce its size.
(This algorithm is used by the PKWARE [®] compression program PKZIP™ and is integral to the PNG graphics
format.) The compressed data is then encoded using the MIME Base64 scheme as described above.


A CRC is calculated across the Base64-encoded data. If the CRC-check fails or the download is aborted,
the object can be invalidated by the printer.


The robust encodings can be piggybacked on the existing download commands with full backward
compatibility. This is done by prefacing the new encodings with a header that uniquely identifies them. The
download routines in the printer firmware can key-off the header to determine whether the data is in the
old ASCII hexadecimal encoding or one of the new encodings. This allows existing downloadable objects
to be used in their present format, while new objects can be created using the same download commands
with the new encodings for increased integrity and reduced download times.


For easy reference, B64 and Z64 are referred to as ZB64. In any reference to the ZB64 encoding, assume
that both Base64-only (B64) and LZ77/Base64 (Z64) encodings are accepted.


The following is an example of an existing download command using the new encoding:


1584


ZB64 Encoding and Compression

```
~DTARIAL,59494,:Z64:H4sICMB8+DMAC0FSSUFMLlRURgDsmnd8VEW7x5+ZOedsyibZNNJhly
WhbEJIwYSwJDGNkmwghJIgJYEEEhQIPSggKAjEAiIiVaSoIJYNBAkIGgGxUBVUUCGU0JQSC0WFnPvbE
+SF18+9H+
8f973X+3Jm93umzzNznvnNSSFGRJ6ARAVZvXK7XDaXLyTiR5B7ontuZPQ824I5RKIa6ew
+aba8+pU1rVDZiciv

```

[multiple lines deleted]

```
/O6DU5wZ7ie2+g4xzDPwCpwm3nqW2GAPcdclxF4fIP66jHjncmKvKzh/ZUNCxl9/QQx2HXHYB4m/
PkQcdCdx2G7OY
t+mszkMh4iZxoifvkh89BFipo87kwD/Bf/dOcycAAEA:a1b2

```

The parameters are identical to the existing `~DT` command:

|Parameter|Details|
|---|---|
|`o` = font name|Values: any valid TrueType name, up to 8 characters<br>Default:  if a name is not specified, UNKNOWN is used<br>In this example, Arial is the specified font.|
|`s` = font size|Values: the number of memory bytes required to hold the Zebra-downloadable<br>format of the font<br>Default: if an incorrect value or no value is entered, the command is ignored In<br>this example, 59494 is the size.<br>To maintain compatibility with the existing ASCII hexadecimal encoding, this<br>field must contain the size of the un-enclosed and uncompressed object —<br>the number of bytes that are finally placed into the printer’s memory, not the<br>number of bytes downloaded.|
|`data` = data string|Values: a string of ASCII hexadecimal values (two hexadecimal digits/byte). The<br>total number of two-digit values must match parameter`s`.<br>Default: if no data is entered, the command is ignored<br>Everything following the size field is data. The new encoding imposes a header<br>with a unique signature. The new encoding must start with the characters :B64:<br>(data encoded in Base-64 only) or :Z64: (data compressed with LZ77, then<br>encoded in Base-64) followed by the encoded data.<br>After the data is presented, another colon (:) and four hexadecimal digits<br>comprise the CRC. The Base64 standard allows new-line characters (carriage<br>returns and line feeds) to be inserted into the encoded data for clarity. These<br>characters are ignored by the printer.|



When downloading graphics, the colon is used in the current ASCII hexadecimal encoding indicate “repeat
the previous dot row.” Since this shorthand is invalid for the first character of data (no previous dot row has
been downloaded), it is safe for the printer to detect the leading colon character as the lead-in for the new
encodings.


1585


ZB64 Encoding and Compression

## **B64 and Z64 Encoding**


These download encodings, B64 and Z64, are created as drop-in replacements for the existing ASCII
hexadecimal encoding.


B64 encoding do the following:


          - Encode the compressed data using the MIME Base64 algorithm.


          - Calculate a CRC across the encoded data.


          - Add a unique header to differentiate the new format from the existing ASCII hex encoding.


Z64 encoding do the following:


          - Compress the data using the LZ77 algorithm.


          - Encode the compressed data using the MIME Base64 algorithm.


          - Calculate a CRC across the encoded data.


          - Add a unique header to differentiate the new format from the existing ASCII hexadecimal encoding.


The data field have this format:

```
       :id:encoded_data:crc

|Parameter|Details|
|---|---|
|`:id`|The identifying string B64 or Z64.|
|`:iencoded_data`|Data to download, compressed with LZ77 (if the<br>id parameter is set to Z64) and encoded with<br>Base64.|
|`:crc`|Four hexadecimal digits representing the CRC<br>calculated over the :encoded_data field.|


```

The printer calculates a CRC across the received data bytes and compare this to the CRC in the header. A
CRC mismatch is treated as an aborted download.


The B64 and Z64 encodings can be used in place of the ASCII hexadecimal encoding in any download

|command. The commands are:|Col2|
|---|---|
|`~DB`|Download Bitmap Font|
|`~DE`|Download Encoding|
|`~DG`|Download Graphic|
|`~DL`|Download Unicode Bitmap Font|
|`~DS`|Download Scalable Font|
|`~DT`|Download TrueType Font|
|`~DU`|Download Unbounded TrueType Font|
|`^GF`|Graphic Field (with compression type set to “ASCII<br>hex”)|



1586


ZB64 Encoding and Compression


The `~DB` (Download Bitmap Font) command can use the new encodings in place of the ASCII hexadecimal
encoding in data sub-fields. Each character is encoded individually. However, for small amounts of data,
the identifying B64 or Z64 header and trailing CRC may negate any gains made by using the new format.

For backward compatibility, the `^HG` (Host Graphic) command uses the ASCII hexadecimal encoding. It
does not use the new encodings.


1587