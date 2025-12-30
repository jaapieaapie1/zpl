# weblink.zebra_connector.proxy



This command assigns the URL for the proxy used to connect to the Zebra connector.


The proxy server protocol, port, domain, username, and password are all encoded into the URL via the
format outlined in RFC2396 (http://www.ietf.org/rfc/rfc2396.txt).


The username and password must avoid the invalid characters listed in RFC2396 (e.g. ':', '@', '/', etc). If an
invalid character must be used it needs to be escaped using '%' as described in RFC2396.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To assign the URL for the proxy used to connect to the Zebra connector:

```
       ! U1 setvar "weblink.zebra_connector.proxy" "url"

```

**Values**


Any valid URL up to 2048 characters


Expected URL format: [http|https]://[user:pass@]domain[:port]/[path]


The URL will need to be built according to the server/proxy environment the printer is running
where:

              - `"user"` username

              - `"password"` password

              - `"host"` either the hostname or IP address

              - `"port"` port number

              - `"other"` anything else needed in the path

**Default**
```
          ""
```

              - The user:pass, port, and path are all optional.

              - The default scheme must be either `HTTPS` or `HTTP` . The default is `HTTP` .

              - The default port is 1080.


The default is to omit the username and password.


**Example**

```
       ! U1 setvar "weblink.zebra_connector.proxy"
       "https://user:pass@my.internal.proxy:7840/init"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "weblink.zebra_connector.proxy"

```

1358


SGD Network Commands


**Result**

```
 "https://user:pass@my.internal.proxy:7840/init"

```

1359


SGD Network Commands