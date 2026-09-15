from kozo.core import App, Event


class WindowExample(App):
    def __init__(self) -> None:
        super().__init__()  # default values initialization

    def on_start(self) -> bool:  # returns True if initialization was successful
        # loading config, setup the engine, creating window, loading assets, etc.
        print("function 'on_start' called from Python")
        return True

    def on_event(
        self, event: Event
    ) -> None:  # calling this function whenever any event happens on the system
        print("function 'on_event' called from Python")

    def on_update(self) -> None:
        # update game logic, physics, and CPU loading stuff
        print("function 'on_update' called from Python")

    def on_render(self) -> None:  # optimized for rendering and can be configured
        print("function 'on_render' called from Python")

    def on_shutdown(
        self, event: Event
    ) -> None:  # calling when memory out, regular shutdown, PC shutdown
        # save states, free resource, close the window
        print("function 'on_shutdown' called from Python")


if __name__ == "__main__":
    example = WindowExample()
    example.run()
