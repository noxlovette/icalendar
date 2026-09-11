Winnow-based parser

## Test data attribution

Delivering a correct, dependable parser isn't possible by testing only against
hand-written examples — real `.ics` files produced by actual calendar clients
(Google Calendar, Thunderbird, Etar, khal, and others) are full of edge cases,
quirks, and outright spec violations that synthetic fixtures never surface.

`tests/fixtures/collective-icalendar/` vendors the `.ics` test fixtures from
[`collective/icalendar`](https://github.com/collective/icalendar), the most
widely used iCalendar library on GitHub, whose test suite has accumulated
these real-world files over more than a decade of bug reports and interop
fixes. They're used here under the terms of that project's BSD-style license
(Copyright (c) 2012-2013, Plone Foundation; see
[`LICENSE.rst`](https://github.com/collective/icalendar/blob/main/LICENSE.rst)).

`tests/fixtures/libical/` and `tests/fixtures/libical-fuzz-corpus/` vendor the
curated `.ics` test cases and fuzzer-discovered crash corpus from
[`libical/libical`](https://github.com/libical/libical) (`test-data/`, `4.0`
branch), the reference C implementation of iCalendar. The fuzz corpus in
particular exercises parser-robustness edge cases (malformed/adversarial
input) that no hand-written fixture set would think to construct. These files
are used under the terms libical itself declares for that directory
(`LGPL-2.1-only OR MPL-2.0`; Copyright 1999 Contributors to the Libical
project) — see the `LICENSE.txt`, `COPYING.LESSER.txt`, and `LICENSES/`
copied alongside them in each fixture directory. This license is **narrower
than, and does not change, this crate's own `Apache-2.0` license**: MPL-2.0's
copyleft is file-level (it doesn't extend to unrelated files in the same
repo) and LGPL-2.1's obligations are about linking compiled code into a
program, which plain-text test data never does. To keep the published crate
package unambiguous, `tests/fixtures/` is excluded from what gets shipped to
crates.io (see `Cargo.toml`) — these fixtures exist for this repo's own test
suite, not as part of the distributed library.
