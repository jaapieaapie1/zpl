# Applicator Functions



The printer applicator port option can be controlled in part or completely by ZBI 2. When ZBI takes control
of a pin, the printer’s built-in applicator functionality will not have access to that pin. This function will allow
the printer to perform some of the functionality that a programmable logic controller (PLC) could.


**AUXPORT_STEALPIN**
Takes control of a pin and allows ZBI to perform other actions on the pin.

**AUXPORT_SETPIN**
Sets the output level on an applicator pin.

**AUXPORT_GETPIN**
Retrieves the state of the applicator pin.

**AUXPORT_RELEASEPIN**
Returns a pin controlled by ZBI to normal printer operation.


537


ZBI Commands