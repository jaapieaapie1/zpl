# ip.mirror.appl_path



This command overrides the default location of the mirror application path ( `"appl"` in `ip.mirror.path` ).
If the SGD is empty, the default path is used. If an error occurs retrieving application from path (i.e. path not
found or application not there), the default path is NOT used. This path must be fully defined in relation to
the mirror server root and is not relative to `ip.mirror.path` .


**Setvar**


To set the path to the application on the mirror server:

```
       ! U1 setvar "ip.mirror.appl_path" "values"

```

**Values**


A valid application path (location).


**Default**
```
          ""

```

**Getvar**


To retrieve the path to the application on the mirror server:

```
       ! U1 getvar "ip.mirror.appl_path"

```

**Example**

If the default value is used, the application would be copied from `"ip.mirror.path"\appl`

If you send `! U1 setvar "ip.mirror.appl_path" "program\current"` and
`"ip.mirror.path"` has the value `"c:\mirror"`, then the application would be copied from `c:`
`\mirror\program\current` .


1268


SGD Network Commands