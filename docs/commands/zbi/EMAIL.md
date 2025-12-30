# Email Program


```

This program sends a simple email message to user@domain.com, assuming a valid email server is set up
by identifying the SMTP server on the print server. In order to write email via ZBI, the port written to must
be named "EML".


**Example**


This is an example of email

```
     1 rem **************************************************
     1 rem Zebra Technologies ZBI Sample Program
     1 rem
     1 rem Professional programming services are available. Please contact
     1 rem ZBI-Experts@zebra.com for more information.
     1 rem
     1 rem This is an example of connecting to an email server to send
     1 rem email.
     1 rem **************************************************
     1 rem EOT$ is the special character used to denote end of transmission
     1 rem **************************************************
     5 let EOT$ = chr$(4)
     1 rem **************************************************
     1 rem Open a connection to the email port; if there is an error, try
     1 rem again
     1 rem **************************************************
     10 open #1: name "EML"

```

593


ZBI Commands

```
     15 on error goto 10
     1 rem **************************************************
     1 rem Specify address to send message to, signal end of recipients
     1 rem with EOT$
     1 rem Note: To send to multiple addressees, separate addressees with
     1 rem a space
     1 rem **************************************************
     20 print #1: "user@domain.com";EOT$;
     1 rem **************************************************
     1 rem Fill in the message information
     1 rem **************************************************
     30 print #1: "From: Sample User"
     40 print #1: "To: Recipient"
     50 print #1: "Subject: This is a test"
     60 print #1: ""
     70 print #1: "Hello!"
     80 print #1: i
     1 rem **************************************************
     1 rem Terminate message
     1 rem **************************************************
     90 print #1: "";EOT$
     1 rem **************************************************
     1 rem Close the port, since each open port is only good for sending
     1 rem one message
     1 rem **************************************************
     100 close #1
     110 sleep 2
     120 let i = i + 1
     130 goto 10