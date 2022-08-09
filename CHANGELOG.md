# changelog

## [unreleased]
* removed `Stopwatch::new`
* added `Stopwatch::new`
* added `Stopwatch::new_started`
* added `Stopwatch::with_elapsed`
* added `Stopwatch::with_elapsed_started`
* added `Stopwatch::from_raw`
* added `Stopwatch::replace`
* added `Display`, `Error`, and `Hash` impls for `Error`
* added examples to docs
* improved test consistency
* renamed license files

## [0.2.0] - 2022-08-07
* added `Debug` impl for `Stopwatch`
* improved docs organisation

## [0.1.1] - 2022-08-07
* added README.md
* updated crate metadata

## [0.1.0] - 2022-08-07
* added `Stopwatch`
  * added `Stopwatch::new`
  * added `Stopwatch::start`
  * added `Stopwatch::stop`
  * added `Stopwatch::toggle`
  * added `Stopwatch::reset`
  * added `Stopwatch::set`
  * added `Stopwatch::elapsed`
  * added `Stopwatch::is_running`
  * added `Stopwatch::is_stopped`
  * added `Clone`, `Copy`, `Default`, `Add`, `Sub`, `AddAssign`, and `SubAssign` impls for `Stopwatch`
* added `Error`
  * added `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq` impls for `Error`
