# ip.http.custom_link_url



This command creates a custom link on the printer web page for the printer’s URL. Use
`ip.http.custom_link_name` to define the text that will display for your link.


**Setvar**


To change the custom URL:

```
       ! U1 setvar "ip.http.custom_link_url" "value"

```

**Values**


A string of 64 characters or less.


**Default**
```
          ""

```

**Getvar**


To respond with the custom URL:

```
       ! U1 getvar "ip.http.custom_link_url"

```

**Example**

```
       ! U1 setvar "ip.http.custom_link_url" "http://www.zebra.com/sdk"

```

1258


SGD Network Commands