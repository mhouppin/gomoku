""" This module contains the class app"""

from typing import Callable
import pygame

from srcs.builders.menu_builder import MenuBuilder

from srcs.utils.constants import WINDOW_HEIGHT, WINDOW_WIDTH
from srcs.models import Coordinate, Menu


class App:
    """Class App is the main class of the app"""

    is_running: bool

    __screen: pygame.Surface
    __windows_size: Coordinate
    __has_menu: bool
    __menu_builder: MenuBuilder
    __menu: Menu

    def __init__(self) -> None:
        pygame.init()
        self.is_running = False
        self.__windows_size = Coordinate(x=WINDOW_WIDTH, y=WINDOW_HEIGHT)
        self.__menu_builder = MenuBuilder()
        self.__has_menu = False

    def set_windows_size(self, width: int, height: int) -> None:
        """Set the windows_size attribute"""
        self.__windows_size = Coordinate(x=width, y=height)

    def get_windows_width(self) -> int:
        """Get the windows width"""
        return self.__windows_size.x

    def get_windows_height(self) -> int:
        """Get the windows height"""
        return self.__windows_size.y

    def get_windows_size(self) -> tuple[int, int]:
        """Get the windows size"""
        return self.__windows_size.to_tuple()

    def init_menu(self) -> None:
        """Initialize the menu if does not exist, else do nothing"""
        if self.__has_menu is False:
            self.__has_menu = True

    def set_menu_title(self, title: str) -> None:
        """Set the menu_title attribute"""
        self.__menu_builder.set_title(title)

    def set_menu_dark_theme(self) -> None:
        """Set the menu_dark_theme attribute"""
        self.__menu_builder.set_dark_theme()

    def set_menu_light_theme(self) -> None:
        """Set the menu_light_theme attribute"""
        self.__menu_builder.set_light_theme()

    def add_menu_button(self, name: str, callback: Callable) -> None:
        """Add a button to the menu"""
        self.__menu_builder.add_button(name=name, callback=callback)

    def add_menu_quit_button(self) -> None:
        """Add a quit button to the menu"""
        self.__menu_builder.add_quit_button()

    def __render(self) -> None:
        """Render the application"""
        pygame.display.flip()
        if self.__has_menu:
            self.__menu.run()

    def run(self) -> None:
        """Run the application"""
        self.__screen = pygame.display.set_mode(size=self.get_windows_size())

        if self.__has_menu:
            self.__menu_builder.set_size(self.__windows_size.x, self.__windows_size.y)
            self.__menu_builder.set_surface(self.__screen)
            self.__menu = self.__menu_builder.build()

        self.is_running = True
        while self.is_running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.is_running = False
            self.__render()
        pygame.quit()
