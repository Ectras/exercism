from typing import Any


class BufferFullException(BufferError):
    """Exception raised when CircularBuffer is full.

    message: explanation of the error.

    """

    def __init__(self, message: str):
        pass


class BufferEmptyException(BufferError):
    """Exception raised when CircularBuffer is empty.

    message: explanation of the error.

    """

    def __init__(self, message: str):
        pass


class CircularBuffer:
    def __init__(self, capacity: int):
        # Use list of capacity + 1 to discriminate empty and full buffer, see Wikipedia.
        self.buffer = [None] * (capacity + 1)
        self.read_idx = 0
        self.write_idx = 0

    def _advance_read(self):
        self.read_idx = (self.read_idx + 1) % len(self.buffer)

    def _advance_write(self):
        self.write_idx = (self.write_idx + 1) % len(self.buffer)

    def read(self) -> Any:
        if self.read_idx == self.write_idx:
            raise BufferEmptyException("Circular buffer is empty")
        item = self.buffer[self.read_idx]
        self._advance_read()
        return item

    def write(self, data: Any):
        if (self.write_idx + 1) % len(self.buffer) == self.read_idx:
            raise BufferFullException("Circular buffer is full")
        self.buffer[self.write_idx] = data
        self._advance_write()

    def overwrite(self, data: Any):
        self.buffer[self.write_idx] = data
        self._advance_write()
        if self.write_idx == self.read_idx:
            self._advance_read()

    def clear(self):
        self.read_idx = 0
        self.write_idx = 0
