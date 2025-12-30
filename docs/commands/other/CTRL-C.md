# CTRL-C




ZBI Commands


Sending an end-of-transmission character, `ETX` (3 in hex), to the console (port 0) terminates the ZBI
program currently running.


**Format**
N/A

**Parameters**
N/A

**Example**
N/A

**Comments**
In most terminal programs, you terminate the program using the `Ctrl-C` key sequence. Another
method is to store an ETX character in a file and have the terminal program send the file to the
console port.


**NOTE:** It is not recommended to use `RESTART` after using a `CTRL-C` because a
command may have been prematurely interrupted. Restarting will have an undefined
result.


458