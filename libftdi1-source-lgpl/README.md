### Internal use only!

This crate provides the LGPL source of `libftdi` for `libftdi1-sys[vendored]`.
If your dependency tree includes this crate, your final binary likely includes LGPL-licensed code.
If you're not sure whether that's acceptable for your use case, please consult your lawyer.

The only reason this is a separate crate is to provide correct licensing metadata
because `libftdi1-sys` is permissively licensed, while the vendored source is LGPL.
You should not depend on it directly.

Version `X.Y.Z` of this crate is interpreted as "`libftdi X.Y`, crate revision Z".
As a result, it may not follow SemVer if `libftdi` ever breaks it.
Thus `libftdi1-sys` uses strict version dependencies like `=1.4.0`.

Note that the repository refers to the `libftdi` repository as a submodule,
and that repository contains code under multiple licenses including GPL.
The crate package, however, takes care to include only LGPL components.
