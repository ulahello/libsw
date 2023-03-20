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
