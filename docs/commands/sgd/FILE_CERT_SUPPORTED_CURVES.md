# file.cert.supported_curves



This command retrieves a list of supported elliptical curves for certificates.


**Getvar**


To return the list of supported elliptical curves:

```
       ! U1 getvar "file.cert.supported_curves"

```

**Result**


A comma delimited list of curve names from OpenSSL that the printer supports for certificates and
encryption in general.


822