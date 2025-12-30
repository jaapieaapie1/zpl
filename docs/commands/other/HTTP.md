# HTTP POST Alerts


**HTTP POST Alerts**


Link-OS printers can issue alerts to a web server that is listening for HTTP POST requests. The advantage
of an HTTP POST alert over the other destinations available (for example, TCP, UDP, SNMP) is that HTTP is
firewall friendly.

## **Configuring Alerts Where the Alert Destination is HTTP POST**


Any setting in the `alerts.http` branch that is set will take effect for any `HTTP POST` alerts that occur
from that point forward. A printer reset is not required for the settings to take effect.

If the server is configured to accept and process `HTTP POST` messages either via a CGI script or a serverside script such as PHP or ASP then the alert can be forwarded to that server from the printer. The printer
will send the alert using the multipart/form-data Content-Type. This allows any type of data, including
binary data, to be sent via the `POST` .

The `POST` will support two variables within the body of the `POST` :

          - alertMsg – This is the alert details and the content follows the format of a standard alert when it is
issued over one of the other alert destinations (for example, serial, USB, TCP, etc.).

          - uniqueId – The unique id of the printer. This matches the value in `device.unique_id` .

The HTTP POST request will look as follows (sent when the printer was paused)

```
       POST /http_post/alert.php HTTP/1.1
       Host: 10.3.4.58
       Accept: */*
       Connection: close
       Content-Length: 281
       Expect: 100-continue
       Content-Type: multipart/form-data;
       boundary=----------------------------350c75835f46

       ------------------------------350c75835f46
       Content-Disposition: form-data; name="alertMsg"

       ALERT%3A%20PRINTER%20PAUSED
       ------------------------------350c75835f46
       Content-Disposition: form-data; name="uniqueId"

       XXQLJ120900310
       ------------------------------350c75835f46-
```

1727


HTTP POST Alerts


**IMPORTANT:** The message is using HTTP/1.1 and therefore HTTP/1.1 header fields. This is
important because some older proxy servers do not handle these fields gracefully and may block
the `POST` message.

It is important to note that the message is using HTTP/1.1 and therefore HTTP/1.1 header fields. This is
important because some older proxy servers do not handle these fields gracefully and may block the POST
message.

## **How to Parse via PHP**


The following example shows how to parse the POST message.


It does not, however, show how to use this information on other pages, store the results in a database,
report this to another device on the domain, etc. The response in this example will be sent back to the
printer, but it will be ignored by the printer. If you wish to see the response you can use a packet sniffing
tool such as Wireshark.

```
       <?php

       $alertMsg = urldecode($_POST["alertMsg"]);
       if (preg_match("/(\w+(\s+\w+)?):\s+(((SGD SET)\s+([\w\d\.\_]+)\s+\->\s+(.
       +))|([\w|\s]+))/", $alertMsg, $matches)== 1) {

       $alertType = $matches[1];

       if ($matches[5] === "SGD SET") {
       $alertCondition = $matches[5];
       $alertSgdName = htmlspecialchars($matches[6]);
       $alertSgdData = htmlspecialchars($matches[7]);
       } else {
       $alertCondition = $matches[3];
       }
       }

       echo "<H1>Alert Received</H1><br/>\r\n";
       echo "<h2>Original Message = $alertMsg</h2><br/>\r\n";
       echo "<h2>Alert Type = $alertType</h2><br/>\r\n";
       echo "<h2>Alert Condition = $alertCondition</h2><br/>\r\n";
       echo "<h2>SGD Name = $alertSgdName</h2><br/>\r\n";
       echo"<h2>SGD Value = $alertSgdData</h2><br/>\r\n";
       ?>

## **Basic Configuration**

```

To determine how much configuration is necessary, consider the following questions:


          - Is the remote server that the printer is attempting to connect to outside of the corporate firewall?


          - Does the firewall require a username and password to access the remote server?


          - Does the printer require a proxy server to access the remote server?


1728


HTTP POST Alerts


If the answer to any of these questions is ‘yes’, then more than the basic configuration may be necessary.
Depending upon the network environment that the printer is in access the remote server may only require
that a few settings be set.


To configure an alert to be sent via HTTP POST to a remote server, issue the following command:

```
       ! U1 setvar "alerts.add" "PRINTER PAUSED,HTTP POST,Y,Y,
       http://www.examplecorpinc.com/alerts.php,0,N,"

```

The above command will issue an HTTP POST alert to the remote server (http://www.examplecorpinc.com/
alerts.php) when the printer is paused or un-paused.


          - The first parameter indicates the condition to monitor. A list of available alert conditions can be viewed
by issuing:

```
       ! U1 getvar "alerts.conditions"

```

          - The second parameter indicates the alert destination. For the purposes of this section HTTP-POST is
the preferred destination. A list of available alert destinations can be viewed by issuing:

```
       ! U1 getvar "alerts.destinations"

```

          - The third and fourth parameters are ‘Send on Set’ and ‘Send on Clear’, respectively. They can be either
"Y" for monitor the alert or "N" for don’t monitor the alert. If both are set to "N" then the alert will not be
added or it will be deleted if the alert already existed. To view which alerts already exist issue:

```
       ! U1 getvar "alerts.configured"

```

          - The fifth parameter holds the URL for the server that will be sent the HTTP POST. It holds a maximum
[of 255 characters for the URL and it must conform to the URI standards described in RFC3986 (http://](http://www.ietf.org/frc/frc3986.txt)
[www.ietf.org/rfc/rfc3986.txt).](http://www.ietf.org/frc/frc3986.txt)


          - The sixth parameter should be set to 0 for HTTP POST alerts.


          - The seventh parameter and eighth parameter will not be covered in this section and should be set as
indicated in the description above. See the SGD documentation for details on these two parameters.

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


1729


HTTP POST Alerts

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

```

1730




HTTP POST Alerts

## **Enabling Logging**


If your printer has trouble connecting, you may wish to enable logging.


By default, logging is not enabled in order to reduce the amount of memory consumed when the HTTP
alert feature is enabled. It is recommended that once the alert HTTP feature is configured properly and is
performing as expected that the logging be disabled or that a very small (less than 100) number of logging
entries be permitted.

To enable logging, `alerts.http.logging.max_entries` needs to be modified. By default it is set to
0, which indicates that no messages are logged. When attempting to troubleshoot connection issues it is
recommended that max_entries be set to at least 100 entries. Setting max_entries to 100 means that the
100 newest logging entries will be present in alerts.http.logging.entries as older entries are discarded once
the maximum number of entries is reached.

```
       ! U1 setvar "alerts.http.logging.max_entries" "100"

```

The logging settings do not require the printer to be reset before taking effect. This does not mean that
previous logging message that would have been logged will appear when the max_entries setting is
changed from 0 to a greater value. It means that any new logging messages will be logged from that point
forward.


Issue the following command to clear any log entries currently in the alerts.http.logging.entries buffer.

```
       ! U1 do "alerts.http.logging.clear" ""

## **Navigating the Log Output**

```

The log can contain much information, even in the scenario where the printer successfully connects to the
remote server. This section explains how to read the log and highlights some of the key entries that will
help to determine if the connection was successful.


A typical log entry looks as follows:

```
       [12-04-2012 14:57:10.625] [http] Attempting connection to http://
       www.examplecorpinc.com/alerts.php

```

The first column is the date and time that the event occurred. The format of the date and time matches
the format of `rtc.date` and `rtc.time` . The time, however, also includes the milliseconds to aid in
troubleshooting network latency concerns.

The second column indicates the connection name, which is currently always set to `‘http’` .

The third column is the actual message that contains information about what occurred in the printer at the
corresponding time in column one. In the above example the printer was attempting to POST the alert to
the connection to the URL specified in the configured alert.


Review Understanding Errors in the Alerts HTTP Log to understand what it means when certain logging
messages/errors appear in the log.


1731


HTTP POST Alerts

## **Understanding Errors in the Alerts HTTP Log**


Errors in the Alerts HTTP log.



|Error|Cause / Solution|
|---|---|
|`Couldn’t connect to`<br>`host`|This could mean any number of things occurred that prevented the<br>printer from connecting. This message is always present when the<br>connection to the remote failed and is typically accompanied by an<br>HTTP Response Code. SeeHTTP Messages for the possible HTTP<br>Response Codes and their meaning.If this issue persists contact<br>Zebra Technical Support and provide the output of the following<br>command (ensure that logging is enabled and that this error appears<br>within the entries).<br>`! U1 getvar "alerts"`|

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


**10.** Is the proxy server port the default (1080) or another port (for example, 3128 for the Linux Squid
proxy)?


**NOTE:** If using the Linux Proxy Server Squid, and you are having trouble connecting, note
that it may be configured to:


   - disallow POST messages


   - only operate in HTTP/1.0 mode 3


   - disallow SSL connections.


Refer to your Linux Squid documentation for complete details.


**11.** Does the firewall permit HTTPS connections initially or do I need to connect via HTTP first?


1732


HTTP POST Alerts


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
|`HTTP/1.1 200 OK`|This indicates that the`HTTP POST` was successful.|
|`HTTP/1.1 30x Moved/`<br>`Redirect/etc`|This indicates that the URL specified has moved or that the firewall<br>redirected the printer to another location (typically this is done to<br>authenticate a user in a transparent proxy configuration).|
|`HTTP/1.1 401`<br>`Unauthorized`|This indicates that the printer either needs to authenticate with the<br>server or failed to authenticate with the remote server (or server/<br>router along the route to the server).|
|`HTTP/1.1 403 Forbidden`|This typically means that the authentication was provided and valid,<br>however, the user does not have access to the requested resource.|
|`HTTP/1.1 404 Not Found`|This indicates that the remote URL provided points to an invalid<br>location on the server. This does indicate, however, that the server<br>name is valid. Just the path after the domain name is invalid.|


1733