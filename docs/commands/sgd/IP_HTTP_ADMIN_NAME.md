# ip.http.admin_name



This command sets the name to be used for authentication on the print server web pages.


**Setvar**


To set the admin user name to the specified value:

```
       ! U1 setvar "ip.http.admin_name" "value"

```

**Values**


A string with a maximum of 25 characters.


**Default**
```
          "admin"

```

**Getvar**


To respond with the admin user name:

```
       ! U1 getvar "ip.http.admin_name"

```

**Example**

This `setvar` example shows the value set to `"useradmin101"` .

```
       ! U1 setvar "ip.http.admin_name" "useradmin101"

```

1255


SGD Network Commands