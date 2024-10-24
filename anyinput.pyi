class Session:
    def __init__(self, backend: str = "enigo"): ...
    def get_device(self, device: str) -> MouseDevice | KeyboardDevice | TouchDevice:
        """Get the device of the given type.

        Args:
            device: The type of device to get.

        Returns:
                The device of the given type.

        Raises:
            ValueError: If the device type was not found, this can happen if the device is not supported on the current platform, or is otherwise not available.
        """

class Device:
    """Base class for all devices."""

class MouseDevice(Device):
    def release(self, button: str):
        """Release the given mouse button.

        Args:
            button: The mouse button to release.
        """
    def press(self, button: str): ...
    def click(self, button: str): ...
    def hold(self, button: str, duration: float): ...
    def drag(self, button: str, dx: int, dy: int, duration: float): ...
    def move_abs(self, x: int, y: int, duration: float): ...
    def move_rel(self, dx: int, dy: int, duration: float): ...

class KeyboardDevice(Device): ...
class TouchDevice(Device): ...

# others...
