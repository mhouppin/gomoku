"""This module contains the Grid class."""
from pydantic import BaseModel, Field


WIDTH: int = 19
HEIGHT: int = 19


class Grid(BaseModel):
    """Class Grid is the model of the grid of the game."""

    rows: list[list[int]] = Field(
        default=[[0 for _ in range(WIDTH)] for _ in range(HEIGHT)]
    )

    def __init__(self) -> None:
        pass
