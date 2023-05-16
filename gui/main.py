"""GUI commands"""
import click

from srcs.utils.constants import WINDOW_HEIGHT, WINDOW_WIDTH
from srcs.builders import AppBuilder
from srcs.models import App


@click.group()
def gui() -> None:
    """GUI commands"""


@gui.command()
def run() -> None:
    """Run the GUI"""
    app_builder: AppBuilder = AppBuilder()

    app: App = (
        app_builder.set_windows_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .set_menu_title("Gomoku")
        .set_dark_theme()
        .add_menu_button("1 vs 1", lambda menu: menu.disable())
        .add_menu_button("1 vs AI", lambda menu: print("1 vs AI"))
        .add_menu_button("AI vs AI", lambda menu: print("AI vs AI"))
        .add_menu_quit_button()
        .build()
    )

    app.run()


if __name__ == "__main__":
    gui()
