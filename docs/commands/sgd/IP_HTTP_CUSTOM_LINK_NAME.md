# ip.http.custom_link_name



This command creates a custom link below the FAQ link on the print server settings page. Use
`ip.http.custom_link_url` to define the URL for your link name.


**Setvar**


To set the custom link name:

```
       ! U1 setvar "ip.http.custom_link_name" "value"

```

**Values**


Any string, maximum of 25 characters.


**Default**
```
          ""

```

**Getvar**


To respond with the custom link name

```
       ! U1 getvar "ip.http.custom_link_name"

```

**Example**

This `setvar` example shows the value set to `"Click Here for Info"` .
```
       ! U1 setvar "ip.http.custom_link_name" "Click Here for Info"

```

1257


SGD Network Commands