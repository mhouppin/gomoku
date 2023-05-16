from typing import Callable, overload

import pygame_menu


class Button:
    """This class is used to create a button object."""

    name: str
    callback: Callable | pygame_menu.events.MenuAction

    @overload
    def __init__(self, name: str, callback: pygame_menu.events.MenuAction) -> None:
        ...

    @overload
    def __init__(self, name: str, callback: Callable) -> None:
        ...

    def __init__(
        self, name: str, callback: Callable | pygame_menu.events.MenuAction
    ) -> None:
        self.name = name
        self.callback = callback
