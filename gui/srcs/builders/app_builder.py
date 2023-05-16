from typing import Callable
from srcs.models import App


class AppBuilder:
    """This class is used to create a app object."""

    app: App

    def __init__(self) -> None:
        self.app = App()

    def reset(self) -> None:
        """Reset the app attribute"""
        self.app = App()

    def set_windows_size(self, width: int, height: int) -> "AppBuilder":
        """Set the windows_size attribute"""
        self.app.set_windows_size(width, height)
        return self

    def set_menu_title(self, title: str) -> "AppBuilder":
        """Set the menu_title attribute"""
        self.app.init_menu()
        self.app.set_menu_title(title)
        return self

    def set_menu_dark_theme(self) -> "AppBuilder":
        """Set the dark theme"""
        self.app.init_menu()
        self.app.set_menu_dark_theme()
        return self

    def set_menu_light_theme(self) -> "AppBuilder":
        """Set the light theme"""
        self.app.init_menu()
        self.app.set_menu_light_theme()
        return self

    def add_menu_button(self, name: str, callback: Callable) -> "AppBuilder":
        """Add a button to the menu"""
        self.app.init_menu()
        self.app.add_menu_button(name, callback)
        return self

    def add_menu_quit_button(self) -> "AppBuilder":
        """Add a quit button to the menu"""
        self.app.init_menu()
        self.app.add_menu_quit_button()
        return self

    def build(self) -> App:
        """Return the built app"""
        return self.app
