# 🔀 Simultaneity

> Parallelism and concurrency

<iframe
    id="diagram_in_progress_1"
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20app_server%3A%0A%20%20app_upload%3A%0A%20%20db%3A%0A%20%20%20%20db_server%3A%0A%20%20%20%20%20%20db_server_1%3A%0A%20%20%20%20%20%20db_server_2%3A%0A%20%20%20%20db_schema_wrapper%3A%0A%20%20%20%20%20%20db_schema%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20app_server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22App%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20app_upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20db%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_1%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_2%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_schema_wrapper%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22DB%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20db_schema%3A%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22DB%20Schema%3Cbr%20%2F%3ECreate%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%7D%0Aedges%3A%0A%20%20app_server__app_upload%3A%20%5Bapp_server%2C%20app_upload%5D%0A%20%20app_upload__config%3A%20%5Bapp_upload%2C%20config%5D%0A%20%20db_server__config%3A%20%5Bdb_server%2C%20config%5D%0A%20%20db_server__db_schema%3A%20%5Bdb_server%2C%20db_schema%5D%0A%20%20db_server_1__db_server_2%3A%20%5Bdb_server_1%2C%20db_server_2%5D%0A%20%20db_schema__start%3A%20%5Bdb_schema%2C%20start%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20db%3A%20hidden%0A%20%20db_server_1%3A%20hidden%0A%20%20db_server_2%3A%20hidden%0A%20%20db_server_1__db_server_2%3A%20hidden%0A%20%20db_schema_wrapper%3A%20hidden%0A%20%20app_server%3A%20%26blue%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-blue-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-blue-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer&diagram_only=true"
    width="630" height="240"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -187px; display: block;">
</iframe>

<iframe
    id="diagram_in_progress_2"
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20app_server%3A%0A%20%20app_upload%3A%0A%20%20db%3A%0A%20%20%20%20db_server%3A%0A%20%20%20%20%20%20db_server_1%3A%0A%20%20%20%20%20%20db_server_2%3A%0A%20%20%20%20db_schema_wrapper%3A%0A%20%20%20%20%20%20db_schema%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20app_server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22App%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20app_upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20db%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_1%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_2%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_schema_wrapper%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22DB%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20db_schema%3A%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22DB%20Schema%3Cbr%20%2F%3ECreate%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%7D%0Aedges%3A%0A%20%20app_server__app_upload%3A%20%5Bapp_server%2C%20app_upload%5D%0A%20%20app_upload__config%3A%20%5Bapp_upload%2C%20config%5D%0A%20%20db_server__config%3A%20%5Bdb_server%2C%20config%5D%0A%20%20db_server__db_schema%3A%20%5Bdb_server%2C%20db_schema%5D%0A%20%20db_server_1__db_server_2%3A%20%5Bdb_server_1%2C%20db_server_2%5D%0A%20%20db_schema__start%3A%20%5Bdb_schema%2C%20start%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20db%3A%20hidden%0A%20%20db_server_1%3A%20hidden%0A%20%20db_server_2%3A%20hidden%0A%20%20db_server_1__db_server_2%3A%20hidden%0A%20%20db_schema_wrapper%3A%20hidden%0A%20%20app_server%3A%20%26green%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-green-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-green-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_upload%3A%20%26blue%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-blue-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-blue-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_server__app_upload%3A%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afill-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20db_server%3A%20%2Ablue%0A&diagram_only=true"
    width="630" height="240"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -187px; display: none;">
</iframe>

<iframe
    id="diagram_in_progress_3"
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20app_server%3A%0A%20%20app_upload%3A%0A%20%20db%3A%0A%20%20%20%20db_server%3A%0A%20%20%20%20%20%20db_server_1%3A%0A%20%20%20%20%20%20db_server_2%3A%0A%20%20%20%20db_schema_wrapper%3A%0A%20%20%20%20%20%20db_schema%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20app_server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22App%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20app_upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20db%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_1%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_2%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_schema_wrapper%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22DB%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20db_schema%3A%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22DB%20Schema%3Cbr%20%2F%3ECreate%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%7D%0Aedges%3A%0A%20%20app_server__app_upload%3A%20%5Bapp_server%2C%20app_upload%5D%0A%20%20app_upload__config%3A%20%5Bapp_upload%2C%20config%5D%0A%20%20db_server__config%3A%20%5Bdb_server%2C%20config%5D%0A%20%20db_server__db_schema%3A%20%5Bdb_server%2C%20db_schema%5D%0A%20%20db_server_1__db_server_2%3A%20%5Bdb_server_1%2C%20db_server_2%5D%0A%20%20db_schema__start%3A%20%5Bdb_schema%2C%20start%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20db%3A%20hidden%0A%20%20db_server_1%3A%20hidden%0A%20%20db_server_2%3A%20hidden%0A%20%20db_server_1__db_server_2%3A%20hidden%0A%20%20db_schema_wrapper%3A%20hidden%0A%20%20app_server%3A%20%26green%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-green-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-green-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_upload%3A%20%2Agreen%0A%20%20db_server%3A%20%2Agreen%0A%20%20config%3A%20%26blue%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-blue-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-blue-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20db_schema%3A%20%2Ablue%0A%20%20app_server__app_upload%3A%20%26green_arrow%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afill-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_upload__config%3A%20%2Agreen_arrow%0A%20%20db_server__config%3A%20%2Agreen_arrow%0A%20%20db_server__db_schema%3A%20%2Agreen_arrow%0A&diagram_only=true"
    width="630" height="240"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -187px; display: none;">
</iframe>

<iframe
    id="diagram_done_1"
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20app_server%3A%0A%20%20app_upload%3A%0A%20%20db%3A%0A%20%20%20%20db_server%3A%0A%20%20%20%20%20%20db_server_1%3A%0A%20%20%20%20%20%20db_server_2%3A%0A%20%20%20%20db_schema_wrapper%3A%0A%20%20%20%20%20%20db_schema%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20app_server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22App%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20app_upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20db%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_1%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_2%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_schema_wrapper%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22DB%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20db_schema%3A%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22DB%20Schema%3Cbr%20%2F%3ECreate%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%7D%0Aedges%3A%0A%20%20app_server__app_upload%3A%20%5Bapp_server%2C%20app_upload%5D%0A%20%20app_upload__config%3A%20%5Bapp_upload%2C%20config%5D%0A%20%20db_server__config%3A%20%5Bdb_server%2C%20config%5D%0A%20%20db_server__db_schema%3A%20%5Bdb_server%2C%20db_schema%5D%0A%20%20db_server_1__db_server_2%3A%20%5Bdb_server_1%2C%20db_server_2%5D%0A%20%20db_schema__start%3A%20%5Bdb_schema%2C%20start%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20db%3A%20hidden%0A%20%20db_server_1%3A%20hidden%0A%20%20db_server_2%3A%20hidden%0A%20%20db_server_1__db_server_2%3A%20hidden%0A%20%20db_schema_wrapper%3A%20hidden%0A%20%20app_server%3A%20%26green%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-green-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-green-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_server__app_upload%3A%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afill-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A&diagram_only=true"
    width="630" height="240"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -187px; display: none;">
</iframe>

<iframe
    id="diagram_done_2"
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20app_server%3A%0A%20%20app_upload%3A%0A%20%20db%3A%0A%20%20%20%20db_server%3A%0A%20%20%20%20%20%20db_server_1%3A%0A%20%20%20%20%20%20db_server_2%3A%0A%20%20%20%20db_schema_wrapper%3A%0A%20%20%20%20%20%20db_schema%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20app_server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22App%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20app_upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20db%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_1%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_2%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_schema_wrapper%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22DB%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20db_schema%3A%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22DB%20Schema%3Cbr%20%2F%3ECreate%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%7D%0Aedges%3A%0A%20%20app_server__app_upload%3A%20%5Bapp_server%2C%20app_upload%5D%0A%20%20app_upload__config%3A%20%5Bapp_upload%2C%20config%5D%0A%20%20db_server__config%3A%20%5Bdb_server%2C%20config%5D%0A%20%20db_server__db_schema%3A%20%5Bdb_server%2C%20db_schema%5D%0A%20%20db_server_1__db_server_2%3A%20%5Bdb_server_1%2C%20db_server_2%5D%0A%20%20db_schema__start%3A%20%5Bdb_schema%2C%20start%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20db%3A%20hidden%0A%20%20db_server_1%3A%20hidden%0A%20%20db_server_2%3A%20hidden%0A%20%20db_server_1__db_server_2%3A%20hidden%0A%20%20db_schema_wrapper%3A%20hidden%0A%20%20app_server%3A%20%26green%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-green-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-green-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_upload%3A%20%2Agreen%0A%20%20app_server__app_upload%3A%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afill-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20db_server%3A%20%2Agreen%0A&diagram_only=true"
    width="630" height="240"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -187px; display: none;">
</iframe>

<iframe
    id="diagram_done_3"
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20app_server%3A%0A%20%20app_upload%3A%0A%20%20db%3A%0A%20%20%20%20db_server%3A%0A%20%20%20%20%20%20db_server_1%3A%0A%20%20%20%20%20%20db_server_2%3A%0A%20%20%20%20db_schema_wrapper%3A%0A%20%20%20%20%20%20db_schema%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20app_server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22App%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20app_upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20db%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_1%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_2%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_schema_wrapper%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22DB%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20db_schema%3A%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22DB%20Schema%3Cbr%20%2F%3ECreate%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%7D%0Aedges%3A%0A%20%20app_server__app_upload%3A%20%5Bapp_server%2C%20app_upload%5D%0A%20%20app_upload__config%3A%20%5Bapp_upload%2C%20config%5D%0A%20%20db_server__config%3A%20%5Bdb_server%2C%20config%5D%0A%20%20db_server__db_schema%3A%20%5Bdb_server%2C%20db_schema%5D%0A%20%20db_server_1__db_server_2%3A%20%5Bdb_server_1%2C%20db_server_2%5D%0A%20%20db_schema__start%3A%20%5Bdb_schema%2C%20start%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20db%3A%20hidden%0A%20%20db_server_1%3A%20hidden%0A%20%20db_server_2%3A%20hidden%0A%20%20db_server_1__db_server_2%3A%20hidden%0A%20%20db_schema_wrapper%3A%20hidden%0A%20%20app_server%3A%20%26green%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-green-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-green-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_upload%3A%20%2Agreen%0A%20%20db_server%3A%20%2Agreen%0A%20%20config%3A%20%2Agreen%0A%20%20db_schema%3A%20%2Agreen%0A%20%20app_server__app_upload%3A%20%26green_arrow%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afill-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20app_upload__config%3A%20%2Agreen_arrow%0A%20%20db_server__config%3A%20%2Agreen_arrow%0A%20%20db_server__db_schema%3A%20%2Agreen_arrow%0A&diagram_only=true"
    width="630" height="240"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -187px; display: none;">
</iframe>

<script type="text/javascript">
const RESET = 0;
const INTERRUPT_1 = 1;
const INTERRUPT_2 = 2;
const INTERRUPT_3 = 3;
const STOP_1 = 4;
const STOP_2 = 5;
const STOP_3 = 6;
function visibility_update(variant) {
    let diagram_in_progress_1 = 'none';
    let diagram_in_progress_2 = 'none';
    let diagram_in_progress_3 = 'none';
    let diagram_done_1 = 'none';
    let diagram_done_2 = 'none';
    let diagram_done_3 = 'none';
    let interruption_point_1 = '0';
    let interruption_point_2 = '0';
    let interruption_point_3 = '0';
    let stopping_point_1 = '0';
    let stopping_point_2 = '0';
    let stopping_point_3 = '0';
    switch (variant) {
        case RESET:
            diagram_in_progress_1 = 'block';
            break;
        case INTERRUPT_1:
            diagram_in_progress_1 = 'block';
            interruption_point_1 = '1.0';
            break;
        case INTERRUPT_2:
            diagram_in_progress_2 = 'block';
            interruption_point_2 = '1.0';
            break;
        case INTERRUPT_3:
            diagram_in_progress_3 = 'block';
            interruption_point_3 = '1.0';
            break;
        case STOP_1:
            diagram_done_1 = 'block';
            interruption_point_1 = '1.0';
            stopping_point_1 = '1.0';
            break;
        case STOP_2:
            diagram_done_2 = 'block';
            interruption_point_2 = '1.0';
            stopping_point_2 = '1.0';
            break;
        case STOP_3:
            diagram_done_3 = 'block';
            interruption_point_3 = '1.0';
            stopping_point_3 = '1.0';
            break;
    }
    document
        .getElementById('diagram_in_progress_1')
        .style
        .setProperty('display', diagram_in_progress_1);
    document
        .getElementById('diagram_in_progress_2')
        .style
        .setProperty('display', diagram_in_progress_2);
    document
        .getElementById('diagram_in_progress_3')
        .style
        .setProperty('display', diagram_in_progress_3);
    document
        .getElementById('diagram_done_1')
        .style
        .setProperty('display', diagram_done_1);
    document
        .getElementById('diagram_done_2')
        .style
        .setProperty('display', diagram_done_2);
    document
        .getElementById('diagram_done_3')
        .style
        .setProperty('display', diagram_done_3);
    document
        .getElementById('interruption_point_1')
        .style
        .setProperty('opacity', interruption_point_1);
    document
        .getElementById('interruption_point_2')
        .style
        .setProperty('opacity', interruption_point_2);
    document
        .getElementById('interruption_point_3')
        .style
        .setProperty('opacity', interruption_point_3);
    document
        .getElementById('stopping_point_1')
        .style
        .setProperty('opacity', stopping_point_1);
    document
        .getElementById('stopping_point_2')
        .style
        .setProperty('opacity', stopping_point_2);
    document
        .getElementById('stopping_point_3')
        .style
        .setProperty('opacity', stopping_point_3);
}
</script>

<div style="
    width: 100%;
" inert>
    <!-- Interruption points -->
    <div id="interruption_point_1" style="
        position: relative;
        left: 77px;
        top: -50px;
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 210px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🛑 Interrupt</div>
    </div>
    <div id="interruption_point_2" style="
        position: relative;
        left: 163px;
        top: -50px;
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 210px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🛑 Interrupt</div>
    </div>
    <div id="interruption_point_3" style="
        position: relative;
        left: 208px;
        top: -50px;
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 210px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🛑 Interrupt</div>
    </div>
    <!-- Stopping points -->
    <div id="stopping_point_1" style="
        position: relative;
        left: -174px;
        display: inline-flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 265px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🚏 Stop</div>
    </div>
    <div id="stopping_point_2" style="
        position: relative;
        left: -105px;
        display: inline-flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 265px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🚏 Stop</div>
    </div>
    <div id="stopping_point_3" style="
        position: relative;
        left: 7px;
        display: inline-flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 265px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🚏 Stop</div>
    </div>
</div>

<div style="text-align: right;">
    <input
        type="button"
        value="Interrupt 1"
        onclick="visibility_update(INTERRUPT_1);"
    ></input>
    <input
        type="button"
        value="Finish 1"
        onclick="visibility_update(STOP_1);"
    ></input>
    <input
        type="button"
        value="Interrupt 2"
        onclick="visibility_update(INTERRUPT_2);"
    ></input>
    <input
        type="button"
        value="Finish 2"
        onclick="visibility_update(STOP_2);"
    ></input>
    <input
        type="button"
        value="Interrupt 3"
        onclick="visibility_update(INTERRUPT_3);"
    ></input>
    <input
        type="button"
        value="Finish 3"
        onclick="visibility_update(STOP_3);"
    ></input>
    <input
        type="button"
        value="Reset"
        onclick="visibility_update(RESET);"
    ></input>
</div>

## Safe Interruption Rules

1. 🔵 Finish everything in progress.
2. ⚫ Don't start anything new.

<details>
<summary>See <code>fn_graph</code> on Github.</summary>

* [Queuer](https://github.com/azriel91/fn_graph/blob/1ef048a6f3827d64fd4eca5dd90a871798bf25ea/src/fn_graph.rs#L1529-L1536):
    - Sends IDs of steps that can be executed.
    - Receives IDs of steps that are complete.
    - Checks for interruption.
* [Scheduler](https://github.com/azriel91/fn_graph/blob/1ef048a6f3827d64fd4eca5dd90a871798bf25ea/src/fn_graph.rs#L1550-L1575)
    - Receives IDs of steps that can be executed.
    - Sends IDs of steps that are complete.

</details>

