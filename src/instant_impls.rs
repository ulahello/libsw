// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

#[cfg(feature = "std_instant")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std_instant")))]
mod std_instant;

#[cfg(feature = "std_systemtime")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std_systemtime")))]
mod std_systemtime;

#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
mod tokio;

#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
mod time;

#[cfg(feature = "coarsetime")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "coarsetime")))]
mod coarsetime;
