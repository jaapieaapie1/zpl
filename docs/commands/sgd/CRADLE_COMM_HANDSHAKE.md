# cradle.comm.handshake



Sets or retrieves the cradle serial USB port handshake mode.


**Setvar**


To set the cradle serial USB port handshake mode:

```
       ! U1 setvar "cradle.comm.handshake" "value"

```

**Values**

              - `"rts/cts"` use hardware handshake via the request-to-send/clear-to-send pins

              - `"xon/xoff"` use software handshake

              - `"none"` no flow control

**Default**
```
          "rts/cts"

```

**Getvar**


To retrieve the cradle serial USB port handshake mode:

```
       ! U1 getvar "cradle.comm.handshake"

```

658


SGD Printer Commands