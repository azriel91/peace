# Clean

## Partial Clean

Even though it is possible to clean up the server without uninstalling the application, it is recommended that the `CleanOpSpec` is implemented for every item.

<!-- Value -->

Imagine one has a goal to test fresh installations of different versions of an application. One approach is to deploy each version in a separate environment, and clean up the whole environment, . have a command that only uninstalls the application from the server, .

Least-work approach, but needs additional engineering.

<!-- reverts the state of the system to a particular point in the graph -->
