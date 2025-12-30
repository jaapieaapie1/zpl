# ACCEPT




ZBI Commands


This function will accept incoming TCP or UDP connections and assign a channel for the connection.
`SERVERSOCKET` must be used to set up the listening socket before `ACCEPT` can be used.


**Format**
```
   ACCEPT (SERVER, CLIENT_INFO$)
```

**Parameters**

    - `SERVER` = the handle returned by the `SERVERSOCKET` call.

    - `CLIENT_INFO$` = string variable will have the connecting client’s IP address and port separated
by a space when using UDP.


**Returns**
The channel number to use to communicate with the client.

**Example**
See the examples for TCP Server on page 504 and UDP Server on page 505.

**Comments**
It is best to poll this function at regular intervals. When there is no connection waiting, this function
will trigger an error. Follow this function with the `ON ERROR` command to divert to a section of code
that handles an unsuccessful connection.

`ACCEPT` can be called before closing a previous connection. This allows for processing multiple
incoming streams of data. There are limits on the number of simultaneous incoming connections
based on the print server model on the printer.


Connection closure can be detected when any input or output command to the port triggers an
error. These commands should be followed by an `ON ERROR` statement to send the program into a
recovery state and to shutdown the connection cleanly.


492


ZBI Commands