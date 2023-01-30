# changelog

## [unreleased]
### added
* added `Instant` trait
* added feature flag `nightly`: Depends on the standard library. Implements `std::error::Error` for `Error`.
* added feature flag `std`: Implements `core::error::Error` for `Error` if std is not enabled. Requires a nightly compiler.
* added feature flag `std_instant`: Implements `Instant` for `std::time::Instant`.
* added feature flag `std_systemtime`: Implements `Instant` for `std::time::SystemTime`.
* added feature flag `tokio`: Implements `Instant` for `tokio::time::Instant`.
* added `libsw::Stopwatch` type alias to `StopwatchImpl<std::time::Instant>`
* added `libsw::TokioStopwatch` type alias to `StopwatchImpl<tokio::time::Instant>`
* added `StopwatchImpl::new_started_at`
* added `StopwatchImpl::set_in_place_at`
* added `StopwatchImpl::reset_in_place`
* added `StopwatchImpl::reset_in_place_at`
* added `StopwatchImpl::replace_at`
* added documentation notes about the chronology of `anchor` to the following methods:
  * `StopwatchImpl::checked_elapsed_at`
  * `StopwatchImpl::set_in_place_at`
  * `StopwatchImpl::replace_place_at`

### changed
* **BREAKING:** bumped MSRV to `1.61.0`
* renamed `Stopwatch` struct to `StopwatchImpl`

## [2.2.0] - 2023-01-13
### added
* added `Stopwatch::set_in_place`

### changed
* clarified documentation for `Stopwatch::from_raw` and `<Stopwatch as PartialEq>::eq`
* optimized `Stopwatch` `PartialEq` and `Hash` impls

## [2.1.1] - 2022-12-18
### changed
* changed `#[must_use]` message for `Guard`

## [2.1.0] - 2022-12-03
### added
* added `Stopwatch::checked_add` and `Stopwatch::checked_sub`
* added `Stopwatch::checked_elapsed` and `Stopwatch::checked_elapsed_at`
* added `Stopwatch::checked_stop` and `Stopwatch::checked_stop_at`

## [2.0.2] - 2022-11-19
### changed
* expanded on main-page crate documentation
* clarified `*_at` method docs
  * previously they may have implied that they block until the current time is `anchor`, when they actually just pretend it is
* re-ordered definitions of `Stopwatch` methods

## [2.0.1] - 2022-11-04
### added
* added `#[must_use]` message for `Guard`

### changed
* inlined function called in `Display` impl for `Error`
* removed unused lints

### fixed
* docs: fixed `Error` description, which implied it's exclusive to `Stopwatch`

## [2.0.0] - 2022-10-07
### changed
* **BREAKING:** changed `Guard::new` to return `libsw::Result<Guard>`
* **BREAKING:** changed `Error`
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
### changed
* mark `Guard` with `#[must_use]`

## [1.8.1] - 2022-09-08
### added
* added documentation notes on functions which take an anchor
  * `Stopwatch::elapsed_at`
  * `Stopwatch::start_at`
  * `Stopwatch::toggle_at`
  * `Guard::elapsed_at`

### changed
* updated crate description

## [1.8.0] - 2022-09-05
### added
* added `Stopwatch::toggle_at`

## [1.7.0] - 2022-09-04
### added
* added `Guard::new`
* defined MSRV as 1.58.1

### changed
* improved clarity of error messages
* cleaned up implementation of `Stopwatch::guard` and `Stopwatch::is_stopped`

## [1.6.0] - 2022-09-03
### added
* added `PartialEq`, `Eq`, and `Hash` impls for `Guard`

### changed
* minor docs improvements

## [1.5.0] - 2022-09-02
### added
* added `PartialEq` and `Eq` impls for `Stopwatch`
* added `Result<T>` type, an alias for `Result<T, Error>`

### changed
* changed implementation of `Hash` for `Stopwatch`
* updated documentation for `Stopwatch::from_raw`
* minor docs improvements

## [1.4.0] - 2022-08-20
### added
* added `Guard::elapsed`
* added `Guard::elapsed_at`

### changed
* minor docs improvements

## [1.3.0] - 2022-08-10
### added
* added `Guard`
  * added `Stopwatch::guard`
  * added `Stopwatch::guard_at`
  * added `Debug` and `Drop` impl for `Guard`

## [1.2.0] - 2022-08-09
### added
* added `Stopwatch::elapsed_at`
* added `Stopwatch::start_at`
* added `Stopwatch::stop_at`

## [1.1.0] - 2022-08-09
### added
* added `Stopwatch::saturating_add`
* added `Stopwatch::saturating_sub`

### changed
* improved docs organisation

## [1.0.0] - 2022-08-09
### added
* added `Stopwatch::new_started`
* added `Stopwatch::with_elapsed`
* added `Stopwatch::with_elapsed_started`
* added `Stopwatch::from_raw`
* added `Stopwatch::replace`
* added `Hash` impl for `Stopwatch`
* added `Display`, `Error`, and `Hash` impls for `Error`
* added examples to docs

### changed
* **BREAKING:** changed `Stopwatch::new` to take no arguments
* improved test consistency
* renamed license files

## [0.2.0] - 2022-08-07
### added
* added `Debug` impl for `Stopwatch`

### changed
* improved docs organisation

## [0.1.1] - 2022-08-07
### added
* added README.md

### changed
* updated crate metadata

## [0.1.0] - 2022-08-07
### added
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
