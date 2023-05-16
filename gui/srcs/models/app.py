""" This module contains the class app"""

from typing import Callable
import pygame
import pygame_menu

from srcs.builders.menu_builder import MenuBuilder

from srcs.utils.constants import (
    BOARD_HEIGHT,
    BOARD_WIDTH,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
)
from srcs.models import Coordinate, Menu


class App:
    """Class App is the main class of the app"""

    is_running: bool

    __screen: pygame.Surface
    __windows_size: Coordinate
    __has_menu: bool
    __menu_builder: MenuBuilder
    __menu: Menu
    __theme: pygame_menu.themes.Theme

    def __init__(self) -> None:
        pygame.init()
        self.is_running = False
        self.__windows_size = Coordinate(x=WINDOW_WIDTH, y=WINDOW_HEIGHT)
        self.__menu_builder = MenuBuilder()
        self.__has_menu = False
        self.__theme = pygame_menu.themes.THEME_DEFAULT

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

    def set_dark_theme(self) -> None:
        """Set the menu_dark_theme attribute"""
        self.__theme = pygame_menu.themes.THEME_DARK
        self.__menu_builder.set_dark_theme()

    def set_light_theme(self) -> None:
        """Set the menu_light_theme attribute"""
        self.__theme = pygame_menu.themes.THEME_DEFAULT
        self.__menu_builder.set_light_theme()

    def add_menu_button(self, name: str, callback: Callable) -> None:
        """Add a button to the menu"""
        self.__menu_builder.add_button(name=name, callback=callback)

    def add_menu_quit_button(self) -> None:
        """Add a quit button to the menu"""
        self.__menu_builder.add_quit_button()

    def __render(self) -> None:
        """Render the application"""
        if self.__menu.is_enabled():
            self.__menu.render()

        pygame.display.update()

    def draw_grid(self) -> None:
        block_height = int(WINDOW_HEIGHT * 0.9 / (BOARD_WIDTH - 1))
        block_width = int(WINDOW_WIDTH * 0.9 / (BOARD_HEIGHT - 1))
        block_size = min(block_width, block_height)

        remaining_height = WINDOW_HEIGHT - block_size * (BOARD_WIDTH - 1)
        remaining_width = WINDOW_WIDTH - block_size * (BOARD_WIDTH - 1)
        self.__screen.fill(self.__theme.background_color)
        large_rect: pygame.Rect = pygame.Rect(
            remaining_width / 2 - 1,
            remaining_height / 2 - 1,
            block_size * (BOARD_WIDTH - 1) + 2,
            block_size * (BOARD_HEIGHT - 1) + 2,
        )
        for x in range(BOARD_WIDTH - 1):
            for y in range(BOARD_HEIGHT - 1):
                rect = pygame.Rect(
                    x * block_size + remaining_width / 2,
                    y * block_size + remaining_height / 2,
                    block_size,
                    block_size,
                )
                pygame.draw.rect(self.__screen, self.__theme.widget_font_color, rect, 1)
        pygame.draw.rect(self.__screen, self.__theme.widget_font_color, large_rect, 1)

    def run(self) -> None:
        """Run the application"""
        self.__screen = pygame.display.set_mode(size=self.get_windows_size())
        clock: pygame.time.Clock = pygame.time.Clock()
        dt: float = 0

        if self.__has_menu:
            self.__menu_builder.set_size(self.__windows_size.x, self.__windows_size.y)
            self.__menu_builder.set_surface(self.__screen)
            self.__menu = self.__menu_builder.build()

        self.is_running = True
        while self.is_running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.is_running = False

            keys = pygame.key.get_pressed()
            if keys[pygame.K_ESCAPE]:
                self.__menu.enable()

            self.draw_grid()

            self.__render()

            dt = clock.tick(60) / 1000
        pygame.quit()
