# SERVERSOCKET



This function opens a listening socket for incoming UDP packets or TCP connections. It must be used in
conjunction with the `ACCEPT` function.


**Format**
```
          SERVERSOCKET (TYPE$,PORT)
```

**Parameters**
`TYPE$` = listens for any of the following communication protocols:

              - `"TCP"` = TCP – PORT parameter is ignored. The current port will be used.

              - `"TCPX"` = TCP – any open port

              - `"UDP"` = UDP – any open port

**Returns**
`NUMERIC` = returns the handle of the server upon success.

**Example**
See the examples for TCP Server on page 504 and UDP Server on page 505.

**Comments**
When using `TCPX`, care needs to be taken not to use a port that is already open on the printer. No
error message will be returned until the `ACCEPT` function is called.


489


ZBI Commands