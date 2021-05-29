# mips-simulator

IC10 state and simulator component of `mips-rs`.

A new simulator state can be constructed via [`ICState::default`] for one with
(per how IC10 chips are in Stationeers):

* 18 memory registers,
* 6 device registers, and
* aliases `sp` and `ra` for memory registers 16 and 17 respectively.

They can also be constructed manually via [`ICState::new`] by providing `mem_size` the number
of memory registers and `dev_size` the number of device registers.
A few helper-builder methods exist for setting the state memory register values
([`with_mem`][`ICState::with_mem`]), state devices ([`with_dev`][`ICState::with_dev`]) and
aliases ([`with_alias`][`ICState::with_alias`]) at the call site.

Note that internally the `sp` and `ra` *registers* are always `mem_size-2` and `mem_size-1`
respectively; that is, internal functions which modify `sp` will always modify the
`mem_size-1`-th register and likewise the `mem_size-2`-th register for `ra`,
regardless of how the `sp` and `ra` *aliases* are set.

#### TODO
- Check how the game handles domain errors on math functions
