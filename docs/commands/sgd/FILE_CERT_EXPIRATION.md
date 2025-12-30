# file.cert.expiration



This command retrieves the certificate expiration information.


**Getvar**


To return the file expiration certificate information:

```
       ! U1 getvar "file.cert.expiration"

```

**Result**


A file with the service name, file name and date of expiration for every certificate in use.


**Example**

In this example, the `getvar` command returns the certificate expiration information for each
communication service (SHA2, WLAN, TLS, WebLink, etc).

```
       ! U1 getvar "file.cert.expiration"
       [{"service":"SHA2","file":"SHA2_DEVICE","end_date":"2037-11-22"},
       {"service":"WLAN","file":"CERTCLN.NRD","end_date":"2019-06-22"},
       {"service":"WIRED","file":null,"end_date":null},
       {"service":"WEBLINK1","file":"WEBLINK1_CERT.NRD","end_date":"2096-01-02"},
       {"service":"WEBLINK2","file":null,"end_date":null},
       {"service":"TLSRAW","file":null," end_date":null},
       {"service":"HTTPS","file":"HTTPS_CERT.NRD","end_date":"2020-03-14"}]

```

In the example above, the command returns the service name, file name and date of expiration for every
certificate in use. The expiration date is in the YYYY-MM-DD format. The certificates that are not provided
by the user are listed as SHA_2 DEVICE as they are available in the Zebra certificate directory. The printer
returns the certificate file information even for not enabled services. If a certificate is not in use for a
particular service, the command returns a null value.


**NOTE:**


          - This command is not displayed in an ALLCV or JSON allconfig as per the SW request as the
JSON SGD is not compatible with the SDK.


          - The command only works with certificate files in use by a service. This command does not
work with CA, KEY, CSR, or any other files.


821


SGD Printer Commands