# rfid.tag.test.content



This command instructs the printer which test to perform on the tag with the `rfid.tag.test.execute`
command.


See rfid.tag.test.execute on page 1539.


**Setvar**

To instruct the printer which test to perform on the tag with the `rfid.tag.test.execute` command:

```
       ! U1 setvar "rfid.tag.test.content" "value"

```

**Values**

              - `"quick"` performs a read EPC test and a write EPC test (using random data)

              - `"read"` performs a read EPC test

              - `"write"` performs a write EPC test (using random data)

              - `"up"` sets the command to the previous test

              - `"down"` sets the command to the next test

**Default**
```
          "quick"

```

**Getvar**


To retrieve the current setting:

```
       ! U1 getvar "rfid.tag.test.content"

```

1538


SGD RFID Commands