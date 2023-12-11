# Architecture dependent code
This module encapsulates all architecture dependent code.  
There are 2 paths that the modules can be referenced by:
- Implicit path: ``crate::arch::host`` for the architecture the code is compiled for
- Explicit path: ``crate::arch::x86_64`` for example to explicitly reference the x86_64 crate. 

# Architecture API
There are modules and functions that every architecture must implement.

## Platform module
The ``platform`` crate is the main crate of the architecture crate.  
Any vital platform setup code lives here.  
That can be code like how to set up the CPU, MMU, etc.  
All functions in this module must be implemented to port the operating system to another architecture.

## Other modules
Other modules are modules like ``serial`` that set up platform specific devices but are not vital to the functioning of the system itself.  
You could call these kinds of modules optional drivers.  
These modules are optionally implemented when your platform needs them.