pub mod backends;
pub mod core;
pub mod device;

use core::{KeyboardHandler, MouseHandler, TouchHandler};
use std::sync::Arc;

use backends::backend::Backend;
use backends::enigo_backend::EnigoBackend;
use device::keyboard::KeyboardDevice;
use device::mouse::MouseDevice;
use device::touch::TouchDevice;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;

// Define a Rust struct
#[pyclass]
struct Session {
    mouse_device: MouseDevice,
    keyboard_device: KeyboardDevice,
    touch_device: TouchDevice,
}

#[pymethods]
impl Session {
    #[new]
    fn new(py: Python<'_>, backend: Option<&str>) -> Self {
        let backend = Arc::new(match backend {
            Some("engio") => EnigoBackend::default(),
            _ => EnigoBackend::default(),
        });
        let mouse_handler = Arc::clone(&backend) as Arc<dyn MouseHandler + Send + Sync>;
        let mouse_device = MouseDevice::new(mouse_handler);

        let keyboard_handler = Arc::clone(&backend) as Arc<dyn KeyboardHandler + Send + Sync>;
        let keyboard_device = KeyboardDevice::new(keyboard_handler);

        // this is unlikely to use the same backend?
        let touch_handler = Arc::clone(&backend) as Arc<dyn TouchHandler + Send + Sync>;
        let touch_device = TouchDevice::new(touch_handler);

        // other devices?
        return Session {
            mouse_device,
            keyboard_device,
            touch_device,
        };
    }

    fn get_device<'py>(&'py self, py: Python<'py>, device: &str) -> PyResult<Py<PyAny>> {
        let device = match device {
            // this clone is cheap, its only a clone of Arc
            "mouse" => Ok(self.mouse_device.clone().into_py(py)),
            "keyboard" => Ok(self.keyboard_device.clone().into_py(py)),
            "touch" => Ok(self.touch_device.clone().into_py(py)),
            _ => Err(PyValueError::new_err("Device not found")),
        }?;
        return Ok(device);
    }
}

#[pymodule]
fn anyinput(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Session>()?;
    m.add_class::<MouseDevice>()?;
    m.add_class::<KeyboardDevice>()?;
    m.add_class::<TouchDevice>()?;
    // m.add_function(wrap_pyfunction!(svg_to_numpy, m)?)?;
    Ok(())
}
