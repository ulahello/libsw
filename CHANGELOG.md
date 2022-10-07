# changelog

## [unreleased]
* changed `Guard::new` to return `libsw::Result<Guard>`
* changed `Error`
  * replaced all variants with `SwStart`, `SwStop`, `SwGuard`, and `GuardNew`
  * marked `non_exhaustive`
  * added `Error::expects_running`
  * added `Error::expects_stopped`
  * changed `Display` impl
* changed `Stopwatch::guard` and `Stopwatch::guard_at` to return `Error::SwGuard`
* shortened impl of `Stopwatch::start_at` and `Stopwatch::stop_at`
* replaced unreachable unwrap in `Stopwatch::guard_at` with debug assertion
* re-licensed under `MIT OR Apache-2.0`

## [1.8.2] - 2022-09-17
* mark `Guard` with `#[must_use]`

## [1.8.1] - 2022-09-08
* updated crate description
* added documentation notes on functions which take an anchor
  * `Stopwatch::elapsed_at`
  * `Stopwatch::start_at`
  * `Stopwatch::toggle_at`
  * `Guard::elapsed_at`

## [1.8.0] - 2022-09-05
* added `Stopwatch::toggle_at`

## [1.7.0] - 2022-09-04
* added `Guard::new`
* improved clarity of error messages
* cleaned up implementation of `Stopwatch::guard` and `Stopwatch::is_stopped`
* defined MSRV as 1.58.1

## [1.6.0] - 2022-09-03
* added `PartialEq`, `Eq`, and `Hash` impls for `Guard`
* minor docs improvements

## [1.5.0] - 2022-09-02
* added `PartialEq` and `Eq` impls for `Stopwatch`
* added `Result<T>` type, an alias for `Result<T, Error>`
* changed implementation of `Hash` for `Stopwatch`
* updated documentation for `Stopwatch::from_raw`
* minor docs improvements

## [1.4.0] - 2022-08-20
* added `Guard::elapsed`
* added `Guard::elapsed_at`
* minor docs improvements

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
