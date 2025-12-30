# power.label_queue.shutdown



Specifies if the printer should wait to shut down until all labels in its internal queue have been printed.


**Setvar**


To specify the label queue shutdown time:

```
       ! U1 setvar "power.label_queue.shutdown" "value"

```

**Values**

              - `"yes"` specifies that the printer will wait to shut down until all labels in its internal queue have
been printed

              - `"no"` specifies that the printer will not wait to shut down until all labels in its internal queue have
been printed


**Default**
```
          "no"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "power.label_queue.shutdown"

```

949


SGD Printer Commands