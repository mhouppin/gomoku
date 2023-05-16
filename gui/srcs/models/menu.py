from typing import Callable, overload
import pygame
import pygame_menu

from srcs.models import Coordinate, Button
from srcs.utils.constants import WINDOW_HEIGHT, WINDOW_WIDTH
from srcs.utils.exceptions import InstanceMenuExecption


class Menu:
    """Class Menu is the main class of the menu"""

    is_running: bool

    __instance: pygame_menu.Menu | None
    __surface: pygame.Surface
    __tilte: str
    __size: Coordinate
    __theme: pygame_menu.themes.Theme
    __buttons: list[Button]

    def __init__(self) -> None:
        self.is_running = False
        self.__instance = None
        self.__title = ""
        self.__size = Coordinate(x=WINDOW_WIDTH, y=WINDOW_HEIGHT)
        self.__buttons = []

    def set_title(self, title: str) -> None:
        """Set the title attribute"""
        self.__title = title

    def set_size(self, width: int, height: int) -> None:
        """Set the size attribute"""
        self.__size = Coordinate(x=width, y=height)

    def set_theme(self, theme: pygame_menu.themes.Theme) -> None:
        """Set the theme attribute"""
        self.__theme = theme

    def set_surface(self, surface: pygame.Surface) -> None:
        """Set the surface attribute"""
        self.__surface = surface

    @overload
    def add_button(self, name: str, callback: pygame_menu.events.MenuAction) -> None:
        ...

    @overload
    def add_button(self, name: str, callback: Callable) -> None:
        self.__buttons.append(Button(name=name, callback=callback))

    def set_default_instance(self) -> None:
        """Create or update the instance of the menu"""
        self.__instance = pygame_menu.Menu(
            self.__title,
            self.__size.x,
            self.__size.y,
            theme=self.__theme,
            surface=self.__surface,
        )

        for button in self.__buttons:
            self.__instance.add.button(button.name, button.callback)

    def run(self) -> None:
        """Run the menu"""
        if self.__instance is None:
            raise InstanceMenuExecption("You need to create the instance first")
        self.__instance.mainloop()
