# Processes

## Build Process

<iframe
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20deps%3A%0A%20%20compile%3A%0A%20%20zip%3A%0A%20%20upload%3A%0Anode_infos%3A%0A%20%20deps%3A%20%20%20%20%7B%20emoji%3A%20%F0%9F%93%A5%2C%20name%3A%20%22Dependency%3Cbr%20%2F%3EDownload%22%20%7D%0A%20%20compile%3A%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3ECompile%22%20%7D%0A%20%20zip%3A%20%20%20%20%20%7B%20emoji%3A%20%F0%9F%93%81%2C%20name%3A%20%22Files%3Cbr%20%2F%3EZip%22%20%7D%0A%20%20upload%3A%20%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%7D%0Aedges%3A%0A%20%20compile__zip%3A%20%20%5Bcompile%2C%20zip%5D%0A%20%20zip__upload%3A%20%20%20%5Bzip%2C%20upload%5D%0A%20%20deps__compile%3A%20%5Bdeps%2C%20compile%5D%0A&diagram_only=true"
    width="500" height="100"
    style="border: 0; transform-origin: top left; scale: 1.4;">
</iframe>


## Deployment Process

<iframe
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20server%3A%0A%20%20upload%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20config%3A%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%20%20%7D%0Aedges%3A%0A%20%20server__upload%3A%20%5Bserver%2C%20upload%5D%0A%20%20upload__config%3A%20%5Bupload%2C%20config%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0A&diagram_only=true"
    width="500" height="100"
    style="border: 0; transform-origin: top left; scale: 1.4;">
</iframe>


1. In every workplace I've been, these two kinds of processes exist.
2. The first is to compile an application, and upload the binary somewhere.
3. The second is to deploy that application to some servers.
4. Who here has ever started a process, and immediatelly regretted it? like "aah, forgot to change that one thing!"
