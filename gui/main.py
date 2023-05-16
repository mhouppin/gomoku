"""GUI commands"""
import click

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
        app_builder.set_windows_size(800, 600)
        .set_menu_title("Gomoku")
        .set_menu_dark_theme()
        .add_menu_button("1 vs 1", lambda: print("1 vs 1"))
        .add_menu_button("1 vs AI", lambda: print("1 vs AI"))
        .add_menu_button("AI vs AI", lambda: print("AI vs AI"))
        .add_menu_quit_button()
        .build()
    )

    app.run()


if __name__ == "__main__":
    gui()
