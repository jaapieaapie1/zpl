# Using Weblink


**Using Weblink**


Weblink is a feature of Zebra Link-OS™ printers.


Using a secure connection, the Weblink feature allows the printer to directly connect to an internet based
server, for the purpose of either sending information to the server or receiving from the server. Weblink
can transport data securely through a firewall.

## **When Should Weblink be Used?**


Weblink can transport any information related to device management, transactional data and information to
be processed at a later time. It can be used as part of an overall cost reduction solution that leverages web
technologies.


Typically, an application called a ‘servlet’ is created and run on the internet based server, waiting for
printers to connect and interact with the servlet. These servlet applications can provide a variety of
functions – from sending operating system updates to the printer, to receiving data from the printer and, in
turn, using that data to trigger events in other systems.


For example, a solution could be created that would feature the printer consuming data from a Bluetooth®
scanner connected to the printer – with that scanned data then being sent from the printer to the internetbased servlet. The servlet would then seek out additional details related to the scanned data, format a
document, and then send it to the printer for printing.

## **Configuring Weblink**


When any WebLink setting (with the exception of the logging settings) is adjusted either via SNMP, SGD, or
JSON it is required that the printer be reset before the new value takes effect.

The `weblink.printer_reset_required` setting will be set to `"yes"` if there are any settings that
have been modified that require a printer reset.

### **Basic Configuration**


To determine how much configuration is necessary the following questions should be considered:


**1.** Is the remote server the printer is attempting to connect to outside the corporate firewall?


**2.** Does the firewall require a username and password to access the remote server?


**3.** Does the printer require a proxy server to access the remote server?


1717


Using Weblink


**4.** Does the firewall permit HTTPS connections initially or does the printer need to connect via HTTP first?


If the answer to any of these questions is ‘yes’, then more than the basic configuration may be
necessary. Depending upon the network environment that the printer is in, accessing the remote server
may only require that a few settings be set.


The minimum requirement is that the URL for the remote server be set. For simplicity, assume that only
conn1 is being used (this is the typical scenario). See also Difference Between Conn1 and Conn2.


To configure the printer to connect to the remote server:

i. Set `weblink.ip.conn1.location` to the URL of the remote server.

The URL must conform to the standards described in RFC3986 (http://www.ietf.org/rfc/rfc3986.txt). For
example, if the remote servlet’s full URL is
```
         https://www.examplecorpinc.com/zebra/weblink/
```

Configure the location setting as follows:

```
         ! U1 setvar "weblink.ip.conn1.location"
         "https://www.examplecorpinc.com/zebra/weblink/"

```

ii. Reset the printer.


When the printer has an IP address, it will attempt to connect to the remote server. In the event that
the remote server does not indicate that the printer has connected, logging may need to be enabled in
order to determine the failure.

## **When a Proxy Server is Part of the Network Configuration**


If a proxy server must be used to access the remote server, the printer’s proxy setting must be set to
connect to the server.


There are typically four properties associated with a proxy server:

          - The proxy server scheme: `HTTP` or `HTTPS`

          - The proxy server address


          - The proxy server port (optional)


          - The username and password for the proxy (optional)


To supply the address of the proxy server (assuming a default port and no username/password), configure
the proxy setting as follows:

```
       ! U1 setvar "weblink.ip.conn1.proxy" "https://my.internal.proxy/"

```

In this scenario the proxy address is `my.internal.proxy` and the scheme is `HTTPS` . The default port
(1080) will be used. No username or password will be used to authenticate with the proxy.


To specify an alternate port configure the proxy as follows:

```
       ! U1 setvar "weblink.ip.conn1.proxy" "https://my.internal.proxy:3128/"

```

To specify a username and password configure the proxy as follows:


1718


Using Weblink

```
       ! U1 setvar "weblink.ip.conn1.proxy" "https://user:pass@my.internal.proxy/"

```

[The proxy username, password, and the rest of the URL must follow the rules specified in RFC3986 (http://](http://www.ietf.org/rfc/rfc3986.txt)
[www.ietf.org/rfc/rfc3986.txt).](http://www.ietf.org/rfc/rfc3986.txt)

## **When HTTP Authentication is Necessary**


Use this configuration when, for example, a firewall requires a username and/or password.


It may be necessary to specify a username and password to various routers and servers along the path to
the remote server. Typically when using a browser to access the server the authentication request will be
presented in the form of a dialog window that asks for the username and password.


Since the printer’s connection to the remote server is headless and non-interactive, the Weblink
configuration allows a user to enter in a server name/username/password triplet. The triplet will be used
in the event that the printer is presented with an authentication request (for example, this typically is
requested via the `HTTP/1.1 401 Unauthorized` request).

To specify authentication credentials, issue the following:
```
       ! U1 setvar "weblink.ip.conn1.authentication.add" "servername.com username
       password"
```

In this scenario the server requesting authentication is servername.com. The username and password to
be supplied are ‘username’ and ‘password’. The server name can be either a DNS name or an IP address.
The username and password cannot be retrieved from SGD, SNMP, or JSON once added. Only the server
name will be returned.


More than one set of authentication triplets can be added. The printer will only use the credentials as they
are needed. In other words, the printer will only use the credentials for servername.com if it receives a
`HTTP/1.1 401 Unauthorized` request from servername.com.

To see what authentication triplets are specified issue:

```
       ! U1 getvar "weblink.ip.conn1.authentication.entries"

```

To remove authentication credentials issue the following:

```
       ! U1 setvar "weblink.ip.conn1.authentication.remove" "servername.com"

## **Additional Firewall Configuration**

```

Some firewalls do not allow the first connection attempt for a device to be HTTPS or require new
connections periodically to keep the initial connections intact. The weblink test branch was provided to
address issues that typically arise because the printer is an unattended device.

To configure the printer to attempt an `HTTP` connection anytime the `HTTPS` connection drops :

```
       ! U1 setvar "weblink.ip.conn1.test.location" "http://www.zebra.com/apps/
       linktest"
       ! U1 setvar "weblink.ip.conn1.test.test_on" "failure"

```

1719


Using Weblink


The weblink.ip.conn1.test.location can be any valid HTTP address. The default uses a link provided by
Zebra that exists for no other purpose than to help developers test their connections to the internet.
Setting `weblink.ip.conn1.test.test_on` to interval or both will force the printer to attempt a
connection to the URL in location every `weblink.ip.conn1.test.retry_interval` seconds (default
is 900 seconds/15 minutes).


To configure the printer to try an HTTP connection periodically, independent of the HTTPS success:

```
       ! U1 setvar "weblink.ip.conn1.test.location" "http://www.zebra.com/apps/
       linktest"
       ! U1 setvar "weblink.ip.conn1.test.test_on" "interval"
       ! U1 setvar "weblink.ip.conn1.test.retry_interval" "900"

## **Difference Between Conn1 and Conn2**

```

The printer has the ability to connect to two different servers.


Connection 1 (conn1) and Connection 2 (conn2) are identical in every way in terms of their configuration. It
is expected that conn2 will typically be left unmodified unless a user has an alternate server that they wish
to use to configure the printer.


A typical scenario in which both connections would be used is if a user wishes to have the printer connect
to both a configuration server and a data source.

## **Enable Logging**


If your printer has trouble connecting, you may wish to enable logging.


By default logging is not enabled in order to reduce the amount of memory consumed when the Weblink
feature is enabled. It is recommended that once the Weblink feature is configured properly and is
performing as expected that the logging be disabled or that a very small (less than 100) number of logging
entries be permitted.

To enable logging, `weblink.logging.max_entries` needs to be modified. By default it is set to 0,
which indicates that no messages are logged. When attempting to troubleshoot connection issues it is
recommended that `max_entries` be set to at least 100 entries. Setting `max_entries` to 100 means
that the 100 newest logging entries will be present in `weblink.logging.entries` as older entries are
discarded once the maximum number of entries is reached.

```
       ! U1 setvar "weblink.logging.max_entries" "100"

```

The logging settings are atypical to the Weblink settings as they do not require the printer to be reset
before taking effect. This does not mean that previous logging message that would have been logged
will appear when the `max_entries` setting is changed from 0 to a greater value. It means that any new
logging messages will be logged from that point forward.


Issue the following command to clear any log entries currently in the weblink.logging.entries buffer.

```
       ! U1 do "weblink.logging.clear" ""

```

1720




Using Weblink

## **Navigating the Log Output**


The log will contain useful information, even in the scenario where the printer successfully connects to the
remote server. This section explains how to read the log and highlights some of the key entries that will
help to determine if the connection was successful.


A typical log entry looks as follows:

```
       [12-04-2012 14:57:10.625] [conn1.1] Attempting connection to https://
       www.examplecorpinc.com/zebra/weblink/

```

The first column is the date and time that the event occurred. The format of the date and time matches
the format of `rtc.date` and `rtc.time` . The time, however, also includes the milliseconds to aid in
troubleshooting network latency concerns.


**NOTE:** For printers that do not have a battery to store the Real Time Clock (RTC) value, the date
will be restored to the default value upon a power cycle. The default value depends upon how
the `rtc.date` SGD is managed. If it has never been set then it will default to the firmware build
date (the value in `appl.date` ). Otherwise, the value in `rtc.date` will default to the value that
it was last set to. This does not mean the value of the `rtc.date` when it was power cycled. It
means that when a user sets `rtc.date` that becomes the new default value.

If the printer has a battery then the `rtc.date` is never default and continues to track the date as
expected.


The second column indicates the connection name and channel that the entries are associated with. The
connection name will match the weblink branch that was configured with the respective URL (for example,
conn1 or conn2). The channel number indicates which channel on the respective connection the entries
corresponds to.


**NOTE:** Channels are additional connections that are requested by the server when the server
needs to perform a specific operation that cannot be done on the channel(s) currently open.
Typically only the RAW channel is open which operates similar to the RAW TCP port. It is typical
to see two channels opened, the main channel and the RAW channel.


The third column is the actual message that contains information about what occurred in the printer at the
corresponding time in column one. In the above example the printer was initiating the connection to the
URL specified in `weblink.ip.conn1.location` .

Review the section titled SSL/TLS Certificate Errors to understand what it means when certain logging
messages/errors appear in the log.

## **SSL/TLS Certificate Errors**


Secure connections to the remote server present the opportunity for several errors when attempting to
connect.


The errors typically involve the certificates used when connecting via SSL or TLS. This section highlights
some of the most common issues involving the certificates.


1721


Using Weblink





|Error|Cause / Solution|
|---|---|
|`"SSL certificate`<br>`problem: self signed`<br>`certificate in`<br>`certificate chain"`|One of the situations that prevent a successful connection is not<br>having the correct Certificate Authority certificates installed on<br>the remote server. Zebra requires that the Zebra Root Certificate<br>Authority and the Zebra Subordinate Certificate Authority be installed<br>on the remote server. This error typically indicates that only one of<br>the Zebra Certificate Authority certificates is installed on the remote<br>server.<br>**IMPORTANT:** When using certificate files, the time on the<br>printer must be set correctly for the websocket connection<br>to succeed, as the time is used in the certificate validation.|
|`"SSL certificate`<br>`problem: unable to`<br>`get local issuer`<br>`certificate"`|One of the situations that prevent a successful connection is not<br>having the correct Certificate Authority certificates installed on<br>the remote server. Zebra requires that the Zebra Root Certificate<br>Authority and the Zebra Subordinate Certificate Authority be installed<br>on the remote server. This error typically indicates that neither of the<br>Zebra Certificate Authority certificates are installed on the remote<br>server.<br>**IMPORTANT:** When using certificate files, the time on the<br>printer must be set correctly for the websocket connection<br>to succeed, as the time is used in the certificate validation.|
|`"SSL certificate`<br>`problem: certificate`<br>`has expired"`|This error indicates that the remote server’s certificate has expired.<br>This is typically an indication that the printer’s date and/or time<br>are incorrect as the Zebra certificates are typically issued for long<br>durations. Check that`rtc.date` and`rtc.time` are set correctly.<br>**NOTE:** For printers that do not have a battery to store the<br>Real Time Clock (RTC) value, the date will be restored to<br>the default value upon a power cycle. The default value<br>depends upon how the`rtc.date` SGD is managed. If<br>it has never been set then it will default to the firmware<br>build date (the value in`appl.date`). Otherwise, the value<br>in`rtc.date`will default to the value that it was last<br>set to. This does not mean the value of the`rtc.date`<br>when it was power cycled. It means that when a user sets<br>`rtc.date` that becomes the new default value.<br>If the printer has a battery then the`rtc.date` is never default and<br>continues to track the date as expected.<br>**IMPORTANT:** When using certificate files, the time on the<br>printer must be set correctly for the websocket connection<br>to succeed, as the time is used in the certificate validation.|


1722


Using Weblink



|Error|Cause / Solution|
|---|---|
|`"SSL certificate`<br>`problem: certificate is`<br>`not yet valid"`|This error indicates that the remote server’s certificate was<br>incorrectly issued or that the printer’s date and/or time are incorrect.<br>Check that the printer’s date and time (`rtc.date` and`rtc.time`)<br>are set correctly and that the certificate’s start and expiration date<br>are valid.<br>**NOTE:** For printers that do not have a battery to store the<br>Real Time Clock (RTC) value, the date will be restored to<br>the default value upon a power cycle. The default value<br>depends upon how the`rtc.date` SGD is managed. If<br>it has never been set then it will default to the firmware<br>build date (the value in`appl.date`). Otherwise, the value<br>in`rtc.date`will default to the value that it was last<br>set to. This does not mean the value of the`rtc.date`<br>when it was power cycled. It means that when a user sets<br>`rtc.date` that becomes the new default value.<br>If the printer has a battery then the`rtc.date` is never default and<br>continues to track the date as expected.|
|`"subjectAltName does`<br>`not match 1.2.3.4"`|Part of the certificate validation process involves making sure<br>that the remote server is who it claims to be. A certificate can<br>be created to validate against several aliases/DNS names.<br>Typically the certificate will not contain the IP address of the<br>server as IP addresses are subject to change. When specifying<br>the remote server’s URL via`weblink.ip.conn1.location`<br>be sure to specify one of the DNS aliases listed in the certificate.<br>The valid names will be listed either under the Common<br>Name (CN) field and/or the subjectAltName (SAN or Subject<br>Alternate Name) field within the certificate. For example, the<br>certificate may have the CN set to`'examplecorpinc'`<br>and the SAN set to`'examplecorpinc.com'` or<br>`'alias.for.examplecorpinc.com'`. Any of the CN or SAN<br>names can be used, but, as the IP address is not listed in the CN or<br>SAN it cannot. It is not recommended that the IP address be part of<br>the SAN if a DNS name is available to avoid connection issues that<br>may arise due to subnet change or DHCP lease expirations, etc.<br>**IMPORTANT:** When using certificate files, the time on the<br>printer must be set correctly for the websocket connection<br>to succeed, as the time is used in the certificate validation.|
|`"SSL certificate`<br>`subject name`<br>`'examplecorpinc.com'`<br>`does not match target`<br>`host name '1.2.3.4'"`|`"SSL certificate`<br>`subject name`<br>`'examplecorpinc.com'`<br>`does not match target`<br>`host name '1.2.3.4'"`|
|`"Unknown SSL protocol`<br>`error in connection`<br>`to ...”`|When this message is seen it means that the remote server’s SSL/<br>TLS configuration is incorrect. Refer toTroubleshooting to ensure the<br>server and printer are both configured correctly.|
|I do not see any of these errors,<br>but the printer still does not<br>connect.|Refer toTroubleshooting to ensure the server and printer are both<br>configured correctly.|

## **Other Typical Errors**





While SSL/TLS connection errors are the most common, there are issues that can arise that prevent a
successful connection.


This section highlights the most common issues.


1723


Using Weblink



|Error|Cause / Solution|
|---|---|
|`"Read failed with an`<br>`unexpected error"`|This message typically indicates that connection to the remote<br>server was lost. The connection can either be lost due to the server<br>powering off or resetting, the firewall or proxy server shutting down<br>the connection, or because the remote server gracefully requests<br>that the connection be discontinued.<br>**NOTE:** After 60 seconds of inactivity on the connection<br>the printer will attempt to contact the server via a TCP<br>Keepalive. If the connection is still present the server<br>will respond and the connection will remain open. After<br>10 successive failed attempts to contact the remote the<br>printer will assume the connection is severed and close the<br>connection. The printer will resume it's attempt to connect<br>to the remote server so that when the server comes back<br>online the printer will re-establish communication.|
|`"Failed to connect (SP`<br>`= #, CI = #, UW = #, AC`<br>`= #, PC = #)"`|If this error is seen one or more of the ‘#’ values will be set to 0. This<br>is an indication of an incorrect configuration of the remote server.<br>Ensure that the remote server is setup according to the Servlet<br>configuration in the Zebra Link-OS SDK documentation.This typically<br>indicates an incorrect version of the remote Application Server (for<br>example, Apache/Tomcat version may be incorrect). If this issue<br>persists contact Zebra Technical Support.. Provide the output of the<br>following command (ensure that logging is enabled and that this<br>error appears within the entries).! U1 getvar "weblink"|

## **Troubleshooting**





Whenever troubleshooting a connection issue, the following questions should be answered to ensure the
configuration is correct.


**1.** Is the printer connected correctly via Wireless or Ethernet?


**2.** Does the printer have a valid IP address?


**3.** Can I ping the printer’s IP address from a device on the same network as the printer?

**4.** Is the remote server URL in `weblink.ip.conn1.location` correct and does it point to the remote
server that is configured for weblink functionality?

**5.** Can you connect to the location defined in the `weblink.ip.conn1.location` setting via a
browser?


**6.** Is the remote server I am attempting to connect to outside the corporate firewall?

**7.** Can the URL specified in `weblink.ip.conn1.test.location` be accessed?

If this is the case, talk with your administrator about altering restrictions for accessing HTTPS
connections.


**8.** Does the firewall require a username and password to access the remote server?


**9.** Do I require a proxy server to access the remote server?


1724


Using Weblink


**10.** Is the proxy server port the default (1080) or another port (for example, 3128 for the Linux Squid
proxy)?


**NOTE:** If using the Linux Proxy Server Squid, and you are having trouble connecting, note
that it may be configured to:


             - disallow POST messages


             - only operate in HTTP/1.0 mode 3


             - disallow SSL connections.


Refer to your Linux Squid documentation for complete details.


**11.** Does the firewall permit HTTPS connections initially or do I need to connect via HTTP first?


**12.** Is the remote server configured to use a supported version of TLS?


Zebra currently supports TLS 1.2 and recommends using the most recent version whenever possible.
Earlier versions may not be supported in the future.


**13.** Are the Zebra Certificate Authority Certificates correctly installed on the remote server?


**14.** Was the server’s certificate issued by Zebra and is it signed by the Zebra Certificate Authority?


**15.** Has the server’s certificate expired?


**16.** Is the printer’s date and time within the issue and expired period of the server’s certificate?

**17.** Does the value in `weblink.ip.conn1.location` match either the Common Name or one of the
names listed in the Subject Alternate Name of the remote server’s certificate?


**18.** Is the proxy server configured correctly and does the respective proxy server allow HTTPS
connections via the HTTP CONNECT method?


**19.** Are there any HTTP authentication attempts when trying to connect that fail?


**20.** Are there any HTTP/1.1 4xx messages in the log?


If your connection issues persist and the solutions in this document do not help, contact Zebra Tech
Support and provide the output of the following command. Ensure that logging is enabled and that the
error(s) appear within the entries)


! U1 getvar "weblink"

## **HTTP Messages**


List of HTTP messages.











|Message|Cause / Solution|
|---|---|
|`HTTP/1.1 100 Continue`|This indicates that the server and printer have begun communicating<br>and is often seen in place of`HTTP/1.1 200 OK`.|
|`HTTP/1.1 101 Switching`<br>`Protocols`|This indicates that the basic connection to the server worked and<br>the protocol is being switched to a more efficient protocol for data<br>transfer.|
|`HTTP/1.1 200 OK`|This indicates that an`HTTP GET` or`HTTP POST` was successful.|
|`HTTP/1.1 30x Moved/`<br>`Redirect/etc`|This indicates that the URL specified has moved or that the firewall<br>redirected the printer to another location (typically this is done to<br>authenticate a user in a transparent proxy configuration).|


1725


Using Weblink







|Message|Cause / Solution|
|---|---|
|`HTTP/1.1 401`<br>`Unauthorized`|This indicates that the printer either needs to authenticate with the<br>server or failed to authenticate with the remote server (or server/<br>router along the route to the server).|
|`HTTP/1.1 403 Forbidden`|This typically means that the authentication was provided and valid,<br>however, the user does not have access to the requested resource.|
|`HTTP/1.1 404 Not Found`|This indicates that the remote URL provided points to an invalid<br>location on the server. This does indicate, however, that the server<br>name is valid. Just the path after the domain name is invalid.|


1726