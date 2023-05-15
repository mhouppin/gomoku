"""Constants used in the game.

This module contains the constants used in the game.

Attributes:
    BROWN (tuple[int, int, int]): The brown color.
    WHITE (tuple[int, int, int]): The white color.
    BLACK (tuple[int, int, int]): The black color.
    BOARD_WIDTH (int): The width of the board.
    BOARD_HEIGHT (int): The height of the board.
    WINDOW_WIDTH (int): The width of the window.
    WINDOW_HEIGHT (int): The height of the window.
    WINDOW_TITLE (str): The title of the window.
"""

# Color codes
BROWN: tuple[int, int, int] = (127, 63, 0)
WHITE: tuple[int, int, int] = (255, 255, 255)
BLACK: tuple[int, int, int] = (0, 0, 0)

# Game constants (in pixels)
BOARD_WIDTH: int = 19
BOARD_HEIGHT: int = 19
WINDOW_WIDTH: int = 1080
WINDOW_HEIGHT: int = 720

# Game constants
WINDOW_TITLE: str = "Gomoku"
