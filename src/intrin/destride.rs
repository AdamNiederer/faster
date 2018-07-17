// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Destride : Sized {
    fn destride_two(self, other: Self) -> (Self, Self);
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self);
}

// TODO: LLVM actually autovectorizes our polyfills, but we should still have an
// explicit implementation for everything

macro_rules! destride_two_polyfill {
    ($self:expr, $other:expr, $($n:expr),*) => {
        (Self::new($($self.extract($n)),*,
                   $($other.extract($n)),*),
         Self::new($($self.extract($n + 1)),*,
                   $($other.extract($n + 1)),*))
    }
}

macro_rules! destride_four_polyfill {
    ($self:expr, $b:expr, $c:expr, $d:expr, $($n:expr),*) => {
        (Self::new($($self.extract($n)),*,
                   $($b.extract($n)),*,
                   $($c.extract($n)),*,
                   $($d.extract($n)),*),
         Self::new($($self.extract($n + 1)),*,
                   $($b.extract($n + 1)),*,
                   $($c.extract($n + 1)),*,
                   $($d.extract($n + 1)),*),
         Self::new($($self.extract($n + 2)),*,
                   $($b.extract($n + 2)),*,
                   $($c.extract($n + 2)),*,
                   $($d.extract($n + 2)),*),
         Self::new($($self.extract($n + 3)),*,
                   $($b.extract($n + 3)),*,
                   $($c.extract($n + 3)),*,
                   $($d.extract($n + 3)),*))
    }
}
