# Architecture dependent code
This module encapsulates all architecture dependent code.  
There are 2 paths that the modules can be referenced by:
- Implicit path: ``crate::arch::host`` for the architecture the code is compiled for
- Explicit path: ``crate::arch::x86_64`` for example to explicitly reference the x86_64 crate. 

# Architecture API
There are modules and functions that every architecture must implement.

| Crate    | Function           |
| -------- | -------            |
| serial   | serial_init()      |

