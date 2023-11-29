# Execution

## Manual

<iframe
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20server%3A%0A%20%20upload%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20config%3A%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%20%20%7D%0Aedges%3A%0A%20%20server__upload%3A%20%5Bserver%2C%20upload%5D%0A%20%20upload__config%3A%20%5Bupload%2C%20config%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20edge_defaults%3A%20%27%5B%26%3E%2A%5D%3Ahidden%27%0A&diagram_only=true"
    width="500" height="100"
    style="border: 0; transform-origin: top left; scale: 1.4;">
</iframe>


## Automatic

<iframe
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20params%3A%0A%20%20tool%3A%0A%20%20process%3A%0A%20%20%20%20server%3A%0A%20%20%20%20upload%3A%0A%20%20%20%20config%3A%0A%20%20%20%20start%3A%0Anode_infos%3A%0A%20%20params%3A%20%20%7B%20emoji%3A%20%F0%9F%93%9D%2C%20name%3A%20%22Parameters%3Cbr%20%2F%3E%22%20%7D%0A%20%20tool%3A%20%20%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22Automation%3Cbr%20%2F%3ESoftware%22%20%7D%0A%20%20process%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20upload%3A%20%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%20%20%7D%0Aedges%3A%0A%20%20upload__params%3A%20%5Bupload%2C%20params%5D%0A%20%20params__upload%3A%20%5Bparams%2C%20upload%5D%0A%20%20params__tool%3A%20%5Bparams%2C%20tool%5D%0A%20%20tool__params%3A%20%5Btool%2C%20params%5D%0A%20%20tool__server%3A%20%5Btool%2C%20server%5D%0A%20%20server__tool%3A%20%5Bserver%2C%20tool%5D%0A%20%20tool__upload%3A%20%5Btool%2C%20upload%5D%0A%20%20tool__config%3A%20%5Btool%2C%20config%5D%0A%20%20tool__start%3A%20%20%5Btool%2C%20start%5D%0A%20%20server__upload%3A%20%5Bserver%2C%20upload%5D%0A%20%20upload__config%3A%20%5Bupload%2C%20config%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20upload__params%3A%20hidden%0A%20%20params__upload%3A%20hidden%0A%20%20server__tool%3A%20hidden%0A%20%20tool__params%3A%20hidden%0A%20%20process%3A%20%3E-%0A%20%20%20%20%5B%26%3E%2A%5D%3Ahidden%0A%20%20tool__server%3A%20%26dashed%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-2%0A%20%20%20%20%5B%26%3Epath%5D%3A%5Bstroke-dasharray%3A2%5D%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afill-blue-400%0A%20%20%20%20%5B%26%3E%2A%5D%3Astroke-blue-500%0A%20%20%20%20%5B%26%3E%2A%5D%3Afocus%3Astroke-emerald-400%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3A%5Bstroke-dasharray%3A4%5D%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afocus%3Afill-emerald-300%0A%20%20%20%20%5B%26%3E%2A%5D%3Ahover%3Astroke-emerald-300%0A%20%20%20%20%5B%26%3Epolygon%5D%3Ahover%3Afill-emerald-200%0A%20%20%20%20cursor-pointer%0A%20%20tool__upload%3A%20%2Adashed%0A%20%20tool__config%3A%20%2Adashed%0A%20%20tool__start%3A%20%20%2Adashed%0A&diagram_only=true"
    width="600" height="220"
    style="border: 0; transform-origin: top left; scale: 1.4;">
</iframe>


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
14. Today's talk is about interruptibility, which is stopping safely.
