# ip.mirror.mode



This command sets the protocol used to perform mirror tasks.


**Setvar**


To sets the protocol for mirror tasks:

```
       ! U1 setvar "ip.mirror.mode" "values"

```

**Values**

`ftp` means the FTP protocol will be used to perform mirror tasks

`sftp` means the SFTP protocol will be used to perform mirror tasks


**Getvar**


To retrieve the path to the application on the mirror server:

```
       ! U1 getvar "ip.mirror.mode"

```

1281


SGD Network Commands