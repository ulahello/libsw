# changelog

## [unreleased]
* added `Guard::elapsed`
* added `Guard::elapsed_at`

## [1.3.0] - 2022-08-10
* added `Guard`
  * added `Stopwatch::guard`
  * added `Stopwatch::guard_at`
  * added `Debug` and `Drop` impl for `Guard`

## [1.2.0] - 2022-08-09
* added `Stopwatch::elapsed_at`
* added `Stopwatch::start_at`
* added `Stopwatch::stop_at`

## [1.1.0] - 2022-08-09
* added `Stopwatch::saturating_add`
* added `Stopwatch::saturating_sub`
* improved docs organisation

## [1.0.0] - 2022-08-09
* changed `Stopwatch::new`
* added `Stopwatch::new_started`
* added `Stopwatch::with_elapsed`
* added `Stopwatch::with_elapsed_started`
* added `Stopwatch::from_raw`
* added `Stopwatch::replace`
* added `Hash` impl for `Stopwatch`
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
