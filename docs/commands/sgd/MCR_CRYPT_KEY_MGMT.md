# mcr.crypt.key_mgmt



Retrieves the MCR encryption key management algorithm for a fixed key or DUKPT (Derived Unique Key
Per Transaction). The return value applies only if `"mcr.crypt.enabled"` is `"on"` .


**Getvar**


To retrieve the MCR encryption key management algorithm for a fixed key or DUKPT:

```
       ! U1 getvar "mcr.crypt.key_mgmt"

```

**Values**

`""` fixed key algorithm

`"dukpt"` derived unique key per transaction

**Default**
NA


843


SGD Printer Commands