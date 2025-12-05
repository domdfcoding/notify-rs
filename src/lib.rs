use notify_rust::{Notification, NotificationHandle, Timeout, Urgency};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass(name="NotificationHandle")]
#[repr(transparent)]
#[derive(Debug)]
/// A wrapper around a [`NotificationHandle`] that can be converted to and from python with `pyo3`.
pub struct PyNotificationHandle(pub NotificationHandle);

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
    fn id(&self) -> PyResult<u32> {
        Ok(self.0.id())
    }
}

#[pyclass(name="Notification")]
#[repr(transparent)]
#[derive(Debug, Clone)]
/// A wrapper around a [`Notification`] that can be converted to and from python with `pyo3`.
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


    fn appname<'a>(mut slf: PyRefMut<'a, Self>, appname: &str) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.appname(appname);
        Ok(slf)
    }
    fn summary<'a>(mut slf: PyRefMut<'a, Self>, summary: &str) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.summary(summary);
        Ok(slf)
    }
    fn subtitle<'a>(
        mut slf: PyRefMut<'a, Self>,
        subtitle: &str,
    ) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.subtitle(subtitle);
        Ok(slf)
    }
    fn image_path<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.image_path(path);
        Ok(slf)
    }
    fn sound_name<'a>(mut slf: PyRefMut<'a, Self>, name: &str) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.sound_name(name);
        Ok(slf)
    }
    fn body<'a>(mut slf: PyRefMut<'a, Self>, body: &str) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.body(body);
        Ok(slf)
    }
    fn icon<'a>(mut slf: PyRefMut<'a, Self>, icon: &str) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.icon(icon);
        Ok(slf)
    }
    fn auto_icon<'a>(mut slf: PyRefMut<'a, Self>) -> PyResult<PyRefMut<'a, Self>> {
        slf.0.auto_icon();
        Ok(slf)
    }
    // TODO: fn hint<'a>(mut slf: PyRefMut<'a, Self>, hint: Hint) -> PyResult<PyRefMut<'a, Self>> {
    // 	self.0.hint(hint);
    // 	Ok(slf)
    // }
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
    fn show(slf: PyRef<Self>) -> PyResult<PyNotificationHandle> {
        match slf.0.show() {
            Err(error) => Err(PyValueError::new_err(error.to_string())),
            Ok(result) => Ok(PyNotificationHandle(result)),
        }
    }
    // TODO: async fn show_async(&self) -> Result<NotificationHandle>
    // TODO: async fn show_async_at_bus(
    // //     &self,
    // //     sub_bus: &str,
    // // ) -> Result<NotificationHandle>
}

#[pymodule]
fn notify_rust_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyNotification>().unwrap();
    m.add_class::<PyNotificationHandle>().unwrap();
    m.add("TIMEOUT_NEVER", -2).unwrap();
    m.add("TIMEOUT_DEFAULT", -1).unwrap();
    m.add("URGENCY_LOW", 0).unwrap();
    m.add("URGENCY_NORMAL", 1).unwrap();
    m.add("URGENCY_CRITICAL", 1).unwrap();

    Ok(())
}
