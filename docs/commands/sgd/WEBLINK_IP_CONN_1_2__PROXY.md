# weblink.ip.conn[1|2].proxy



This command assigns the URL of the proxy for the connection.


The proxy server protocol, port, domain, username, and password are all encoded into the URL via the
format outlined in RFC2396 (http://www.ietf.org/rfc/rfc2396.txt).


The username and password must avoid the invalid characters listed in RFC2396 (e.g. ':', '@', '/', etc). If an
invalid character must be used it needs to be escaped using '%' as described in RFC2396.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To assign the URL of the connection proxy:

```
       ! U1 setvar "weblink.ip.conn1.proxy" "url"

       ! U1 setvar "weblink.ip.conn2.proxy" "url"

```

**Values**


              - Any valid URL up to 2048 characters


              - Expected URL format: [http|https]://[user:pass@]domain[:port]/[path]


              - The URL will need to be built according to the server/proxy environment the printer is running
within.


**Default**
```
          ""
```

              - The user:pass, port, and path are all optional.

              - The default scheme must be either `HTTPS` or `HTTP` . The default is `HTTP` .

              - The default port is 1080.


              - The default is to omit the username and password.


**Getvar**


To retrieve the URL of the connection proxy:

```
       ! U1 getvar "weblink.ip.conn1.proxy"

       ! U1 getvar "weblink.ip.conn2.proxy"

```

**Do**


To assign the URL of the connection proxy:

```
       ! U1 do "weblink.ip.conn1.proxy" "url"

```

1339


SGD Network Commands

```
! U1 do "weblink.ip.conn2.proxy" "url"

```

**Values**


    - Any valid URL up to 2048 characters


    - Expected URL format: [http|https]://[user:pass@]domain[:port]/[path]


    - The URL will need to be built according to the server/proxy environment the printer is running
within.


**Default**
```
   ""
```

    - The user:pass, port, and path are all optional.

    - The default scheme must be either `HTTPS` or `HTTP` . The default is `HTTP` .

    - The default port is 1080.


    - The default is to omit the username and password.


**Example**


Examples of how to connect to various proxy servers:

```
http://username:password@mydomain.com:3128/

http://mydomain.com/

```

1340




SGD Network Commands