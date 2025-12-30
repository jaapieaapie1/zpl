# SERVERCLOSE



This function closes a listening server socket created by SERVERSOCKET.


**Format**
```
          SERVERCLOSE(SOCKET)
```

**Parameters**
`SOCKET` = the socket handle returned from a successful SERVERSOCKET invocation.

**Returns**
Returns a 0 if the socket was already closed or a 1 if the socket was closed successfully.

**Example**
This example shows how to close a listening server socket.

```
          10 LET SERVER_HANDLE = SERVERSOCKET("TCPX", 19100)
          20 LET SCERR = SERVERCLOSE(SERVER_HANDLE)

```

490




ZBI Commands