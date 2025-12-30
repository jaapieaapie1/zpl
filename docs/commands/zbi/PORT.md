# Port Usage Examples



Before diving into the syntax of all the commands, let’s look at some simple applications using the different
features of the communications systems in ZBI.

## **Physical Ports (Serial, Parallel, USB, Bluetooth [®] )**


Though the types of devices interacting with the printer's ports may vary greatly, internal to the printer, the
ports are all handled in the same way. These ports are opened with the ZBI `OPEN` command and closed
with the ZBI `CLOSE` command. When one of these ports is opened, it is disconnected from the ZPL parser
and any data in the buffer will be redirected to the ZBI environment.


**Example**

In the following example, `"SER"` could be replaced by `"PAR"`, `"USB"`, or `"BLU"` depending on the
application.

```
       10 CLOSE ALL
       20 LET INPORT = 1
       25 SLEEP 1
       30 OPEN #INPORT: NAME "SER"
       35 ON ERROR GOTO 25
       40 PRINT #INPORT: "Enter your name:";
       50 INPUT #INPORT: YOURNAME$
       55 ON ERROR GOTO 70
       60 PRINT #INPORT: "You entered: "; YOURNAME$
       70 CLOSE #INPORT

## **ZPL Parser**

```

To make a ZBI program print, it is necessary to create a connection from the program to the ZPL parser on
the printer. The connection will function in the same way as a connection to a physical port, except that
the connection will not automatically terminate. The ZPL parser in the printer can handle many incoming
connections simultaneously. For example, a ZBI program could take control of the serial port and send
label formats to the ZPL parser, while the parallel port (unopened by ZBI) could also be used to send label
formats directly into the parser.


**NOTE:** The ZPL parser will lock onto one port once a format is started (via the `^XA` command).
So, in some cases, is it desirable to start and stop your communications to ZPL in one continuous
sequence.


Another use of ZBI is to check printer status, while another application prints to another port.


**Example**


Here is how that can be done:

```
       10 OPEN #1: NAME "ZPL"
       20 PRINT #1: "~HS"
       30 FOR I = 1 TO 3
       40 INPUT #1: A$

```

503


ZBI Commands

```
     50 PRINT A$
     60 NEXT I

## **TCP Client**

```

There are two methods for making a TCP connection to another server. The first method uses the `OPEN`
command while the second method uses the `CLIENTSOCKET` method.

`CLIENTSOCKET` is the preferred method.


**Example**


The following example demonstrates this method:

```
     10 CLOSE ALL
     20 LET INPORT = CLIENTSOCKET("TCP","192.168.0.1",9100)
     40 LET OUTSTR$ = "REQUESTING SERVER NAME";
     50 DO WHILE (LEN(OUTSTR$) > 0)
     60 LET BYTES_WRITTEN = WRITE(INPORT,OUTSTR$,LEN(OUTSTR$))
     70 ON ERROR GOTO RECOVERY
     80 LET OUTSTR$ = OUTSTR$(1+BYTES_WRITTEN:LEN(OUTSTR$))
     90 LOOP
     100 INPUT #INPORT: YOURNAME$
     110 PRINT #INPORT: "Server returned: "; YOURNAME$
     120 CLOSE #INPORT
     130 SUB RECOVERY
     140 END

## **TCP Server**

```

Setting up a listening server in the printer can be accomplished with the `SERVERSOCKET` function. To
connect to incoming TCP sessions, use the `ACCEPT` function.

When starting the application, call `SERVERSOCKET` . This function will create a handle for this listening
server. Check for incoming connections at regular intervals with the `ACCEPT` function. If there are no
pending sessions, the `ACCEPT` function will return with an error. Handle the error using the `ON ERROR`
command and continue looking for other sessions later.


Depending on how the program is set up, it is possible to handle one or more sessions at a time. If the
program is configured to allow only one session, the other connections will remain pending until they are
shut down by the requesting client or the ZBI program connects them.


**Example**

Here is an example of the `SERVERSOCKET and ACCEPT` commands:

```
     10 CLOSE ALL
     20 LET SERVER_HANDLE = SERVERSOCKET("TCPX",19100)
     30 REM There are no connections yet we are just listening for them
     40 REM Lets loop until we get a connection
     50 SLEEP 1
     60 LET INPORT = ACCEPT(SERVER_HANDLE,CLIENT_INFO$)
     70 ON ERROR GOTO 50

```

504


ZBI Commands

```
     80 PRINT #INPORT: "You have successfully connected!"
     90 PRINT #INPORT: "Login:";
     100 INPUT #INPORT: LOGIN$
     110 PRINT #INPORT: "Password:";
     120 INPUT #INPORT: PASSWORD$
     130 REM We will not be nice and reject the connection
     130 PRINT #INPORT: "Login failed"
     140 CLOSE #INPORT
     150 GOTO 60 ! Go look for the next connection
     160 END

## **UDP Client**

```

There are also two methods for making a UDP connection to another server. The first method uses
the `OPEN` command, while the second method uses the `CLIENTSOCKET` method. UDP is a one way
communication medium, thus, you can only use output commands. Because UDP is connectionless, the
output will be queued up until an `EOT` character is written or the maximum packet size is exceeded. Once
the `EOT` character is written, the packet is formatted and sent.

With UDP, it is important to be careful about understanding what the network being used will support.


In many cases, there will be a limit to the size of the packet that can be used, typically between 1000
and 1500 bytes, but some networks cut this down into the 500 to 600 byte range. To be safe, keep your
packets less than 500 bytes.


UDP does not guarantee transmission. See UDP specifications for more details.


**Example**

Since `CLIENTSOCKET` is the preferred method, an example is shown below.

```
     10 CLOSE ALL
     20 LET INPORT = CLIENTSOCKET("UDP","192.168.0.1",22222)
     30 LET EOT$ = CHR$(4)
     40 PRINT #INPORT: "Packet #"; I; EOT$;
     50 LET I = I + 1
     60 SLEEP 1
     70 GOTO 40

## **UDP Server**

```

Setting up a listening server in the printer can be accomplished with the `SERVERSOCKET` function. Then,
to connect to incoming UDP packets, use the function `ACCEPT` . When starting your application, call
`SERVERSOCKET` . This function will create a handle for this listening server.

Check for incoming packets at a regular interval with the `ACCEPT` function. If there are no pending
sessions, the `ACCEPT` function will return with an error. Just handle the error using the `ON ERROR`
command and continue looking for other sessions later. You will need to call `ACCEPT` for each incoming
packet. When the accept is successful, all of the data will be available. Call `READ` with a `MAX` string size of
2000 and you will have the whole packet in your string. Close the port and wait for the next packet. You
can only read in data using a UDP server.


505


## **E-mail**



ZBI Commands


**Example**


Here is an example of how to set up to receive UDP messages:

```
10 CLOSE ALL
20 LET ZPLPORT = 1
35 OPEN #ZPLPORT: NAME "ZPL"
40 LET SERVER_HANDLE = SERVERSOCKET("UDP",33333)
50 REM There are no connections yet: listening
60 REM Let’s loop until we get a connection
70 SLEEP 1
80 LET INPORT = ACCEPT(SERVER_HANDLE,CLIENT_INFO$)
90 IF INPORT = -1 THEN
92 GOTO 70
94 END IF
100 LET PACKET_SIZE = READ(INPORT,PACKET$,2000)
110 PRINT #ZPLPORT: "^XA^FO100,100^A0N,40,40^FDPACKET FROM:";
115 PRINT #ZPLPORT: CLIENT_INFO$; "^FS"
120 PRINT #ZPLPORT: "^FO100,150^A0N,40,40^FDPACKET SIZE:";
125 PRINT #ZPLPORT: PACKET_SIZE; "^FS"
130 PRINT #ZPLPORT: "^FO100,200^A0N,40,40^FDPACKET DATA:";
135 PRINT #ZPLPORT: PACKET$; "^FS^XZ"
140 CLOSE #INPORT
150 GOTO 60 ! go look for the next connection
160 END

```

ZBI can be used to enhance the printer’s ability to send status via e-mail messages. The process is simple:
open the email port "EML", send the recipient list, send the header, and send the body of the message.


The printer can only process a limited number of outgoing email messages at one time. For this reason,
error handling should be used when opening the connection to wait for the printer to be ready to send the
message. The EOT character is important for delimiting sections of the email message. If it is left out, the
message will not be sent properly.


Before the following code will work, the email settings for the print server must be set up. Consult the print
server manual to learn how to configure the unit.


**Example**


Here is an example of how to send e-mails:

```
1 REM EOT$ this is used to denote end of transmission
5 LET EOT$ = CHR$(4)
1 REM Open a connection to the e-mail port and if it errors
1 REM try again until complete
10 OPEN #1: NAME "EML"
15 ON ERROR GOTO 10
1 REM Specify address to send message to then end signal end
1 REM of recipients with EOT$
1 REM To send to multiple addressees separate addressees by
1 REM space

```

506


ZBI Commands

```
20 PRINT #1: "youraddress@yourdomain.com";EOT$;
1 REM Fill in the message information
30 PRINT #1: "From: HAL"
40 PRINT #1: "To: Dave"
50 PRINT #1: "Subject: A message from HAL"
60 PRINT #1: ""
70 PRINT #1: "Dave, I am sorry I can not let you do that."
80 PRINT #1: i
1 REM Terminate message
90 PRINT #1: "";EOT$
1 REM You must close the port, each open port is only good
1 REM for sending one message
100 CLOSE #1

```

507


ZBI Commands