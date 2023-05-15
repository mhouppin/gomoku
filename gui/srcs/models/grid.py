"""This module contains the Grid class."""
from pydantic import BaseModel, Field

from srcs.utils.constants import BOARD_HEIGHT, BOARD_WIDTH


class Grid(BaseModel):
    """Class Grid is the model of the grid of the game."""

    rows: list[list[int]] = Field(
        default=[[0 for _ in range(BOARD_WIDTH)] for _ in range(BOARD_HEIGHT)]
    )
