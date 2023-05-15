"""Coordinate model for the GUI."""
from pydantic import BaseModel, validator


class Coordinate(BaseModel):
    """Coordinate model for the GUI."""

    x: int
    y: int

    @validator("x", "y")
    @classmethod
    def check_coordinate(cls, value: int) -> int:
        """Check if the coordinate is positive."""
        if value < 0:
            raise ValueError("Coordinate must be positive")
        return value

    def to_tuple(self) -> tuple[int, int]:
        """Return the coordinate as a tuple."""
        return (self.x, self.y)
