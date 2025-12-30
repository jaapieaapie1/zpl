# ip.pop3.print_headers



Determines if the email header will be printed when the email is retrieved via POP3. This only applies if
ip.pop3.enable is set to "on".


**Setvar**


To set the command:

```
       ! U1 setvar "ip.pop3.print_headers" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "ip.pop3.print_headers"

```

1299


SGD Network Commands