/// Wrapper around notify-rust to show desktop notifications
use notify_rust::{Notification, Timeout, Urgency};
use pyo3::{exceptions::PyValueError, prelude::*};

#[cfg(target_family = "unix")]
use notify_rust::NotificationHandle;

#[cfg(target_family = "unix")]
#[pyclass(name = "NotificationHandle")]
#[repr(transparent)]
#[derive(Debug)]
// A wrapper around a [`NotificationHandle`] that can be converted to and from python with `pyo3`.
/// A handle to a shown notification.
pub struct PyNotificationHandle(pub NotificationHandle);

#[cfg(target_family = "unix")]
impl From<PyNotificationHandle> for NotificationHandle {
	fn from(value: PyNotificationHandle) -> Self {
		value.0
	}
}

// impl PyNotificationHandle {
//     pub(crate) fn new(handle: NotificationHandle) -> Self {
//         PyNotificationHandle {0: handle}
//     }
// }

#[cfg(target_family = "unix")]
#[pymethods]
impl PyNotificationHandle {
	// #[new]
	// pub fn __init__() -> PyResult<Self> {
	//     Ok(PyNotificationHandle::new())
	// }

	// TODO: fn wait_for_action<F>(self, invocation_closure: F)
	// TODO: fn close<'a>(slf: PyRefMut<'a, Self>) {
	// 	slf.0.close();
	// }
	// TODO: fn on_close<A>(self, handler: impl CloseHandler<A>)
	// TODO: fn update(&mut self)
	// TODO: macOS
    #[cfg(target_os = "linux")]
	fn id(&self) -> PyResult<u32> {
		Ok(self.0.id())
	}
}

#[pyclass(name = "Notification")]
#[repr(transparent)]
#[derive(Debug, Clone)]
// A wrapper around a [`Notification`] that can be converted to and from python with `pyo3`.
/// Desktop notification.
///
/// Most fields are empty by default, only appname is initialized with the name of the current executable.
/// The appname is used by some desktop environments to group notifications.
pub struct PyNotification(pub Notification);

impl PyNotification {
	pub(crate) fn new() -> Self {
		PyNotification(Notification::new())
	}
}

impl From<PyNotification> for Notification {
	fn from(value: PyNotification) -> Self {
		value.0
	}
}

#[pymethods]
impl PyNotification {
	#[new]
	pub fn __init__() -> PyResult<Self> {
		Ok(PyNotification::new())
	}

	/// Filled by default with executable name.
	// #[getter]
	fn get_appname<'a>(slf: PyRefMut<'a, Self>) -> PyResult<String> {
		Ok(slf.0.appname.clone())
	}
	/// Single line to summarize the content.
	// #[getter]
	fn get_summary<'a>(slf: PyRefMut<'a, Self>) -> PyResult<String> {
		Ok(slf.0.summary.clone())
	}
	/// Subtitle for macOS
	// #[getter]
	fn get_subtitle<'a>(slf: PyRefMut<'a, Self>) -> PyResult<Option<String>> {
		Ok(slf.0.subtitle.clone())
	}
	/// Multiple lines possible, may support simple markup, check out get_capabilities() -> body-markup and body-hyperlinks.
	// #[getter]
	fn get_body<'a>(slf: PyRefMut<'a, Self>) -> PyResult<String> {
		Ok(slf.0.body.clone())
	}
	/// Use a file:// URI or a name in an icon theme, must be compliant freedesktop.org.
	// #[getter]
	fn get_icon<'a>(slf: PyRefMut<'a, Self>) -> PyResult<String> {
		Ok(slf.0.icon.clone())
	}
	// #[getter]
	// TODO: fn get_hints<'a>(slf: PyRefMut<'a, Self>) -> PyResult<HashSet<Hint>>{
	// 	Ok(slf.0.hints)
	// }
	// #[getter]
	// TODO: fn get_actions<'a>(slf: PyRefMut<'a, Self>) -> PyResult<Vec<String>>{
	// 	Ok(slf.0.actions)
	// }
	/// Lifetime of the Notification in ms. Often not respected by server, sorry.
	// #[getter]
	fn get_timeout<'a>(slf: PyRefMut<'a, Self>) -> PyResult<i32> {
		match slf.0.timeout {
			Timeout::Never => Ok(-2),
			Timeout::Default => Ok(-1),
			_ => Ok(slf.0.timeout.into()),
		}
	}

	/// Overwrite the appname field.
	fn appname<'a>(mut slf: PyRefMut<'a, Self>, appname: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.appname(appname);
		Ok(slf)
	}
	/// Set the summary.
	///
	/// Often acts as title of the notification. For more elaborate content use the body field.
	fn summary<'a>(mut slf: PyRefMut<'a, Self>, summary: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.summary(summary);
		Ok(slf)
	}
	/// Set the subtitle.
	///
	/// This is only useful on macOS; It’s not part of the XDG specification.
	fn subtitle<'a>(mut slf: PyRefMut<'a, Self>, subtitle: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.subtitle(subtitle);
		Ok(slf)
	}
	#[cfg(not(target_os = "macos"))]
    fn image_path<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.image_path(path);
		Ok(slf)
	}
	fn sound_name<'a>(mut slf: PyRefMut<'a, Self>, name: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.sound_name(name);
		Ok(slf)
	}
	/// Set the content of the body field.
	///
	/// Multiline textual content of the notification. Each line should be treated as a paragraph. Simple html markup should be supported, depending on the server implementation.
	fn body<'a>(mut slf: PyRefMut<'a, Self>, body: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.body(body);
		Ok(slf)
	}
	/// Set the icon field.
	///
	/// You can use common icon names here; usually those in /usr/share/icons can all be used.
	/// You can also use an absolute path to a file.
	///
	/// .. note:: macOS does not have support manually setting the icon
	fn icon<'a>(mut slf: PyRefMut<'a, Self>, icon: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.icon(icon);
		Ok(slf)
	}
	/// Set the icon field automatically.
	///
	/// This looks at your binary’s name and uses it to set the icon.
	///
	/// .. note:: macOS does not have support manually setting the icon
	fn auto_icon<'a>(mut slf: PyRefMut<'a, Self>) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.auto_icon();
		Ok(slf)
	}
	// TODO: fn hint<'a>(mut slf: PyRefMut<'a, Self>, hint: Hint) -> PyResult<PyRefMut<'a, Self>> {
	// 	self.0.hint(hint);
	// 	Ok(slf)
	// }
	/// Set the timeout.
	fn timeout<'a>(mut slf: PyRefMut<'a, Self>, timeout: i32) -> PyResult<PyRefMut<'a, Self>> {
		match timeout {
			-1 => slf.0.timeout(Timeout::Default),
			-2 => slf.0.timeout(Timeout::Never),
			_ => {
				if timeout >= 0 {
					slf.0.timeout(Timeout::Never)
				} else {
					return Err(PyValueError::new_err(format!(
						"Invalid timeout value {timeout}"
					)));
				}
			}
		};
		Ok(slf)
	}

	#[cfg(target_os = "linux")]
	fn urgency<'a>(mut slf: PyRefMut<'a, Self>, urgency: i32) -> PyResult<PyRefMut<'a, Self>> {
		match urgency {
			0 => slf.0.urgency(Urgency::Low),
			1 => slf.0.urgency(Urgency::Normal),
			2 => slf.0.urgency(Urgency::Critical),
			_ => {
				return Err(PyValueError::new_err(format!(
					"Invalid urgency value {urgency}"
				)));
			}
		};

		Ok(slf)
	}
	// TODO: fn action<'a>(mut slf: PyRefMut<'a, Self>, identifier: &str, label: &str) -> PyResult<PyRefMut<'a, Self>> {
	// 	self.0.action(identifier, label);
	// 	Ok(slf)
	// }
	fn id<'a>(mut slf: PyRefMut<'a, Self>, id: u32) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.id(id);
		Ok(slf)
	}
	fn finalize(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
		slf.0.finalize();
		Ok(slf)
	}

	/// Shows the notification.
	#[cfg(target_family = "unix")]
	fn show(slf: PyRef<Self>) -> PyResult<PyNotificationHandle> {
		match slf.0.show() {
			Err(error) => Err(PyValueError::new_err(error.to_string())),
			Ok(result) => Ok(PyNotificationHandle(result)),
		}
	}

	#[cfg(not(target_family = "unix"))]
	fn show(slf: PyRef<Self>) -> PyResult<()> {
		match slf.0.show() {
			Err(error) => Err(PyValueError::new_err(error.to_string())),
			Ok(_) => Ok(()),
		}
	}
	// TODO: async fn show_async(&self) -> Result<NotificationHandle>
	// TODO: async fn show_async_at_bus(
	// //     &self,
	// //     sub_bus: &str,
	// // ) -> Result<NotificationHandle>
}

#[pymodule]
fn notify_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<PyNotification>().unwrap();

	#[cfg(target_family = "unix")]
	m.add_class::<PyNotificationHandle>().unwrap();

	m.add("TIMEOUT_NEVER", -2).unwrap();
	m.add("TIMEOUT_DEFAULT", -1).unwrap();
	m.add("URGENCY_LOW", 0).unwrap();
	m.add("URGENCY_NORMAL", 1).unwrap();
	m.add("URGENCY_CRITICAL", 2).unwrap();

	Ok(())
}
