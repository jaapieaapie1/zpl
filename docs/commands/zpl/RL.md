# ^RL




ZPL RFID Commands


Use this command to lock/unlock RFID tag memory.


**Lock/Unlock RFID Tag Memory**

The `^RL` command has four distinct formats and functions:

- `^RLP` - `Permanently Lock All Tag Memory` Locks all memory banks and/or passwords, as
defined by the chip manufacturer.

- `^RLB` - `Permanently Lock Specified Memory Sections` Locks blocks of user memory in an
unwriteable state.

- `^RLM` - `Lock/Unlock the Specified Memory Bank` Locks a password or an entire memory bank
in a writeable or unwriteable state. These locks/unlocks can be permanent or reversible.


**^RLP – Permanently Lock All Tag Memory**


Some chip manufacturers have implemented an alternative permalocking mechanism that must be applied
to all memory by permalocking certain banks and/or passwords. The RFID chips' datasheet may specify
permalocking any combination of the Kill, Access, EPC, User, and TID memory to permalock the RFID tag.
The `^RLP` command automatically selects the required memory locations to permalock the RFID tag for a
particular chip.


**NOTE:** The access password is not required for this command. The printer will use the default of
`00000000` .

**Format:** `^RLP`


**^RLB – Permanently Lock Specified Memory Sections**

The `^RLB` command permanently locks (permalocks) one or more sections (individual in a
sub-portions)
tag’s user memory. The section sizes for each tag is defined by the tag manufacturer.

**Format:** `^RLB,s,n`

|Parameters|Details|
|---|---|
|`s =`starting section|Specify the starting section of memory to lock.|
|`n =`number of sections|Specify the number of sections to lock.|



**^RLM – Lock/Unlock the Specified Memory Bank**

The `^RLM` command locks/unlocks the specified password or memory bank on an RFID tag. You can use
this command to do the following:


- lock individual passwords, thereby preventing or allowing subsequent reads or writes of that password


- lock individual memory banks, thereby preventing or allowing subsequent writes to those banks


- Permanently lock (permalock) the lock status for a password or memory bank

**Format:** `^RLM,k,a,e,u`


429


ZPL RFID Commands







|Parameters|Details|
|---|---|
|`k =`kill password<br>function|**Values:**<br>`U` = unlock the kill password*<br>`L` = lock the kill password*<br>`O` = permanently unlock (Open) the kill password<br>`P` = permanently lock (Protected) the kill password|
|`a =`access password<br>function|**Values:**<br>`U` = unlock the access password*<br>`L` = lock the access password*<br>`O` = permanently unlock (Open) the access password<br>`P` = permanently lock (Protected) the access password|
|`e =`EPC memory bank<br>function|**Values:**<br>`U` = unlock the EPC memory bank*<br>`L` = lock the EPC memory bank*<br>`O` = permanently unlock (Open) the EPC memory bank<br>`P` = permanently lock (Protected) the EPC memory bank|
|`u =`USER memory bank<br>function|**Values:**<br>`U` = unlock the USER memory bank*<br>`L` = lock the USER password bank*<br>`O` = permanently unlock (Open) the USER memory bank<br>`P` = permanently lock (Protected) the USER memory bank|
|* The access password must be set to something other than the default of`00000000` to use this value.<br>See the examples for this command for guidance.|* The access password must be set to something other than the default of`00000000` to use this value.<br>See the examples for this command for guidance.|


**Examples**


**^RLM Example 1:** The following command locks all memory banks using a previously specified access
password.

```
^RLM,L,L,L,L^FS

```

**^RLM Example 2:** The following command locks the user memory banks using a previously specified
access password.

```
^RLM,,,,L^FS

```

**^RLB Example:** The following command permalocks sections 0 to 4 of user memory using a previously
specified access password.

```
^RLB,0,4^FS

```

**Combination ^RLM and ^RLB Example 1:** This code does the following:


- writes 12 bytes to user memory


430




ZPL RFID Commands


- writes `12345678` to the access password and `11223344` to the kill password

- permalocks 6 sections of user memory using `12345678` as the access password

- locks the kill and access passwords and permanently unlocks the EPC memory, using `12345678` as the
access password

```
^XA
^RFW,H,0,12,3^FD112233445566778899001122^FS
^RFW,H,P^FD12345678,11223344^FS
^RLB,0,6^FS
^RLM,L,L,O^FS
^XZ

```

**Combination ^RLM and ^RLB Example 2:** This code does the following:


- writes 12 bytes to user memory

- permalocks 6 sections of user memory using `00000000` as the access password

- permalocks the kill password and access password using `00000000` as the access password

```
^XA
^RFW,H,0,12,3^FD112233445566778899001122^FS
^RLB,0,6^FS
^RLM,P,P^FS
^XZ

```

431