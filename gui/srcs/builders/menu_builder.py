from typing import Callable
import pygame
import pygame_menu
from srcs.models.menu import Menu


class MenuBuilder:
    """This class is used to create a menu object."""

    menu: Menu

    def __init__(self) -> None:
        self.menu = Menu()

    def reset(self) -> "MenuBuilder":
        """Reset the menu attribute"""
        self.menu = Menu()
        return self

    def set_title(self, title: str) -> "MenuBuilder":
        """Set the title attribute"""
        self.menu.set_title(title)
        return self

    def set_size(self, width: int, height: int) -> "MenuBuilder":
        """Set the size attribute"""
        self.menu.set_size(width, height)
        return self

    def set_theme(self, theme: pygame_menu.themes.Theme) -> "MenuBuilder":
        """Set the theme attribute"""
        self.menu.set_theme(theme)
        return self

    def set_dark_theme(self) -> "MenuBuilder":
        """Set the dark theme"""
        self.menu.set_theme(pygame_menu.themes.THEME_DARK)
        return self

    def set_light_theme(self) -> "MenuBuilder":
        """Set the light theme"""
        self.menu.set_theme(pygame_menu.themes.THEME_DEFAULT)
        return self

    def set_surface(self, surface: pygame.Surface) -> "MenuBuilder":
        """Set the surface attribute"""
        self.menu.set_surface(surface)
        return self

    def add_button(self, name: str, callback: Callable, *args) -> "MenuBuilder":
        """Add a button to the menu"""
        self.menu.add_button(name, callback, *args)
        return self

    def add_quit_button(self) -> "MenuBuilder":
        """Add a quit button to the menu"""
        self.menu.add_button("Quit", pygame_menu.events.EXIT)
        return self

    def build(self) -> Menu:
        """Build the menu"""
        self.menu.set_default_instance()
        return self.menu
