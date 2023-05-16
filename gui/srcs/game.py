""" This module contains the class Game"""

import pygame

from srcs.models.coordinate import Coordinate


class Game:
    """Class Game is the main class of the game"""

    screen: pygame.Surface
    running: bool
    windows_size: Coordinate

    def __init__(self, windows_size: Coordinate = Coordinate(x=1080, y=720)) -> None:
        pygame.init()
        self.windows_size = windows_size
        self.screen = pygame.display.set_mode(size=windows_size.to_tuple())
        self.running = True

    def run(self) -> None:
        """Run the game"""
        while self.running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.running = False
        pygame.quit()
