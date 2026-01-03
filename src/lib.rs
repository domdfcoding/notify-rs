/// Wrapper around notify-rust to show desktop notifications
use pyo3::prelude::*;
mod notification;
mod notification_handle;
use crate::notification::PyNotification;
use crate::notification_handle::PyNotificationHandle;

#[pymodule]
fn notify_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<PyNotification>().unwrap();
	m.add_class::<PyNotificationHandle>().unwrap();

	m.add("TIMEOUT_NEVER", -2).unwrap();
	m.add("TIMEOUT_DEFAULT", -1).unwrap();
	m.add("URGENCY_LOW", 0).unwrap();
	m.add("URGENCY_NORMAL", 1).unwrap();
	m.add("URGENCY_CRITICAL", 2).unwrap();

	Ok(())
}
