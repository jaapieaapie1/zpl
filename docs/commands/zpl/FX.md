# ^FX




ZPL Commands


The `^FX` command is useful when you want to add **non-printing** informational comments or statements
within a label format. Any data after the `^FX` command up to the next caret (^) or tilde (~) command
does not have any effect on the label format. Therefore, you should avoid using the caret (^) or tilde (~)
commands within the `^FX` statement.


**Comment**

**Format:** `^FXc`






|Parameters|Details|
|---|---|
|`c =` non-printing<br>comment|Creates a non-printable comment.|



**Example:** This is an example of how to use the `^FX` command effectively:


**Comments:** Correct usage of the `^FX` command includes following it with the `^FS` command.


209