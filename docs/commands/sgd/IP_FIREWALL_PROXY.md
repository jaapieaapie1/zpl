# ip.firewall.proxy



Use this command when a printer must go through a proxy server before making an outgoing HTTP/HTTPS
connection. This setting is not connection specific and acts as a general value.


**Setvar**


To specify a proxy server:

```
       ! U1 setvar "ip.firewall.proxy" "[http|https]://[user:password@]host[:port][/
       path]"

```

**Values**

              - `HTTP` or `HTTPS` is a required value.

              - `user:password` is an optional value.

              - `host` is a required value. DNS or IP address is acceptable.

              - `:port` is an optional value

              - `/path` is an optional value.

The maximum length of the string is 2048 characters.


**Getvar**


To have the printer return the value of the proxy server setting:

```
       U1 getvar "ip.firewall.proxy"

```

If the proxy value includes a `user:password`, the are replaced with a `*` .

The maximum length of the string is 2048 characters.


1249


SGD Network Commands