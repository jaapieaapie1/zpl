# alerts.http.proxy



This command assigns the URL of the proxy for any HTTP POST alerts. The proxy server protocol, port,
domain, username, and password are all encoded into the URL via the format outlined in RFC2396.


The username and password must avoid the invalid characters listed in RFC2396 (such as ':', '@', '/'). If an
[invalid character must be used, it needs to be escaped using '%' as described in ietf.org/rfc/rfc2396.txt.](https://www.ietf.org/rfc/rfc2396.txt)


When the setting is changed, the next HTTP POST alert will use the new value.


**Setvar**


To assign the proxy URL for HTTP POST alerts:

```
       ! U1 setvar "alerts.http.proxy" "http://username:password@mydomain.com:3128/"

```

**Values**


Any valid URL up to 2048 characters

URL format expected: `http://[user:pass@]domain[:port]/[path]`

**Default**


              - The user:pass, port, and path are all optional.


              - The default port is 1080.


              - The default is to omit the username and password.


**Getvar**


To retrieve the proxy URL for HTTP POST alerts:

```
       ! U1 getvar "alerts.http.proxy"

```

**Do**


To assign the proxy URL for HTTP POST alerts:

```
       ! U1 do "alerts.http.proxy" "http://username:password@mydomain.com:3128/"

```

**Values**


Any valid URL up to 2048 characters

URL format expected: `http://[user:pass@]domain[:port]/[path]`

**Default**


              - The user:pass, port, and path are all optional.


              - The default port is 1080.


              - The default is to omit the username and password.


**Example**


Examples of how to connect to various proxy servers:


622


SGD Printer Commands

```
http://username:password@mydomain.com:3128/
http://mydomain.com/

```

623


SGD Printer Commands