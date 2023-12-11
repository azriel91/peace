# Execution

## Manual

<object
    type="image/svg+xml"
    data="execution/manual.svg"
    style="transform-origin: top left; scale: 1.4;"></object>


## Automatic

<object
    type="image/svg+xml"
    data="execution/automatic.svg"
    style="transform-origin: top left; scale: 1.4;"></object>

<!--
1. Let's look at the deployment process, and we'll look at both manual and automatic executions.
2. In the manual execution, we ask our infrastructure team for some servers.
3. Upload the application to the server.
4. Set up the configuration.
5. Then start the application.
6. If we need to change the configuration, we can go back to this step, fix it, and restart the application.
7. If we have a new application version, we can go back to this step, upload it, maybe update the configuration for that new version, then start it.
8. In the automatic execution, we have some software that does all of the steps, and we feed parameters to that software.
9. The automation software may be a shell script, cloud automation tool, or something built in-house.
10. If we make a mistake with our parameters, we tend to have to wait for the execution to finish, clean up, and start again.
11. Wouldn't it be ideal, if we could say, "hey, finish what you're doing, but wait here."
12. Then we fix our parameters, press go, and have the automation start where it stopped.
13. There are two concepts here -- stopping and starting.
14. Interruptibility is about stopping safely, and resumability is about starting, where you stopped.
15. Let's get into it.
-->
