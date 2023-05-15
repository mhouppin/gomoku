"""This module contains the class Board"""

from srcs.models.grid import Grid


class Board:
    """Class Board is the main class of the game"""

    grid: Grid

    def __init__(self) -> None:
        self.grid = Grid()
