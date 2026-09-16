from kozo.core import App, Event


class WindowExample(App):
    """
    The application lifecycle is divided into several stages:
    initialization, startup, event processing, updating, rendering, and shutdown.
    """

    def __init__(self):
        """
        Initialize the application.
        This is called when the application instance is created and is
        responsible for initializing the base App state.
        """
        super().__init__()

    def on_start(self):
        """
        Called once when the application starts.
        Use this method to perform application-specific initialization,
        such as loading configuration, creating windows, initializing
        resources, and preparing game assets.
        """
        pass

    def on_event(self, event: Event):
        """
        Handle an event received by the application.
        This method is called whenever the system generates an event,
        such as window input, keyboard or mouse activity, or other
        engine-level events.
        """
        pass

    def on_update(self):
        """
        Update the application state.
        This method is intended for game logic, physics, state updates,
        and other CPU-side processing performed once per update cycle.
        """
        pass

    def on_render(self):
        """
        Render the current application state.
        This method is responsible for submitting rendering operations
        and is optimized for graphics-related workloads. Rendering
        behavior may be configured according to the application's needs.
        """
        pass

    def on_shutdown(self, event: Event):
        """
        Handle application shutdown.
        Called when the operating system or
        the game engine requests that the application terminate.
        Use this method to save application state, release resources,
        close windows, and perform other required cleanup.

        Args:
            event: The shutdown event that triggered the termination.
        """
        pass


if __name__ == "__main__":
    example = WindowExample()
    example.run()
