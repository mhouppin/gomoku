"""This module contains the class Board"""

from srcs.utils.constants import BOARD_HEIGHT, BOARD_WIDTH


class Board:
    """Class Board is the main class of the game"""

    grid: list[list[int]]

    def __init__(self) -> None:
        self.grid = [[0 for _ in range(BOARD_WIDTH)] for _ in range(BOARD_HEIGHT)]
