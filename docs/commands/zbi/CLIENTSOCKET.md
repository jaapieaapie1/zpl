# CLIENTSOCKET



This function creates an outgoing TCP connection or sets up UDP transmissions. Once set up for UDP,
packets can be sent by printing to the socket. Packets are sent when the size limit is met or a `EOT`
character is written.


**Format**
```
          CLIENTSOCKET (TYPE$, IPADDR$, PORT)
```

**Parameters**

              - `TYPE$` = set to "UDP" or "TCP".

              - `IPADDR$` = connects to this address.

              - `PORT` = connects to this IP port.

**Returns**
The port number assigned to the connection.

**Example**
See the examples for TCP Server on page 504 and UDP Server on page 505.

**Comments**
Multiple communications connections can be made up to the maximum of 10. Each protocol may
have a different limit based on the support of the print server used. Test the worst case situation
based on your application’s needs or use `ONERROR` to recover from failed connection attempts.


491