# 🗺️ Strategy

<iframe
    id="diagram_in_progress"
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20app_server%3A%0A%20%20app_upload%3A%0A%20%20db%3A%0A%20%20%20%20db_server%3A%0A%20%20%20%20%20%20db_server_1%3A%0A%20%20%20%20%20%20db_server_2%3A%0A%20%20%20%20db_schema_wrapper%3A%0A%20%20%20%20%20%20db_schema%3A%0A%20%20config%3A%0A%20%20start%3A%0Anode_infos%3A%0A%20%20app_server%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22App%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20app_upload%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20db%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_1%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server_2%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_schema_wrapper%3A%20%7B%20name%3A%20%22%22%20%7D%0A%20%20db_server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22DB%20Server%3Cbr%20%2F%3ELaunch%22%20%7D%0A%20%20db_schema%3A%20%20%7B%20emoji%3A%20%E2%9A%99%EF%B8%8F%2C%20name%3A%20%22DB%20Schema%3Cbr%20%2F%3ECreate%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22App%3Cbr%20%2F%3EConfigure%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22App%3Cbr%20%2F%3EStart%22%20%7D%0Aedges%3A%0A%20%20app_server__app_upload%3A%20%5Bapp_server%2C%20app_upload%5D%0A%20%20app_upload__config%3A%20%5Bapp_upload%2C%20config%5D%0A%20%20db_server__config%3A%20%5Bdb_server%2C%20config%5D%0A%20%20db_server__db_schema%3A%20%5Bdb_server%2C%20db_schema%5D%0A%20%20db_server_1__db_server_2%3A%20%5Bdb_server_1%2C%20db_server_2%5D%0A%20%20db_schema__start%3A%20%5Bdb_schema%2C%20start%5D%0A%20%20config__start%3A%20%20%5Bconfig%2C%20start%5D%0Atailwind_classes%3A%0A%20%20db%3A%20hidden%0A%20%20db_server_1%3A%20hidden%0A%20%20db_server_2%3A%20hidden%0A%20%20db_server_1__db_server_2%3A%20hidden%0A%20%20db_schema_wrapper%3A%20hidden%0A%20%20app_server%3A%20%26blue%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-blue-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-blue-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer&diagram_only=true"
    width="630" height="240"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -187px; display: block;">
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
const INTERRUPT = 1;
const STOP_1 = 2;
const STOP_2 = 3;
const STOP_3 = 4;
function visibility_update(variant) {
    let diagram_in_progress = 'none';
    let diagram_done_1 = 'none';
    let diagram_done_2 = 'none';
    let diagram_done_3 = 'none';
    let interruption_point = '0';
    let stopping_point_1 = '0';
    let stopping_point_2 = '0';
    let stopping_point_3 = '0';
    switch (variant) {
        case RESET:
            diagram_in_progress = 'block';
            break;
        case INTERRUPT:
            diagram_in_progress = 'block';
            interruption_point = '1.0';
            break;
        case STOP_1:
            diagram_done_1 = 'block';
            interruption_point = '1.0';
            stopping_point_1 = '1.0';
            break;
        case STOP_2:
            diagram_done_2 = 'block';
            interruption_point = '1.0';
            stopping_point_2 = '1.0';
            break;
        case STOP_3:
            diagram_done_3 = 'block';
            interruption_point = '1.0';
            stopping_point_3 = '1.0';
            break;
    }
    document
        .getElementById('diagram_in_progress')
        .style
        .setProperty('display', diagram_in_progress);
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
        .getElementById('interruption_point')
        .style
        .setProperty('opacity', interruption_point);
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
    <div id="interruption_point" style="
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
    <!-- Stopping points -->
    <div id="stopping_point_1" style="
        position: relative;
        left: 72px;
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
        left: 137px;
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
        left: 253px;
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
        onclick="visibility_update(INTERRUPT);"
    ></input>
    <input
        type="button"
        value="Finish 1"
        onclick="visibility_update(STOP_1);"
    ></input>
    <input
        type="button"
        value="Finish 2"
        onclick="visibility_update(STOP_2);"
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

```rust ,ignore
/// How to poll an underlying stream when an interruption is received.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptStrategy {
    /// On interrupt, keep going.
    IgnoreInterruptions,
    /// On interrupt, wait for the current future's to complete and yield its
    /// output, but do not poll the underlying stream for any more futures.
    FinishCurrent,
    /// On interrupt, continue polling the stream for the next `n` futures.
    ///
    /// `n` is an upper bound, so fewer than `n` futures may be yielded if the
    /// underlying stream ends early.
    PollNextN(u64),
}
```



1. Think back to the bus example.
2. What if, we want to press the stop button, but we want the bus to stop not at the immediate next stop, but a number of stops down the line.
3. Like before you get on the bus, you schedule that you want the bus to stop 8 stops away -- because that's where your workplace is.
4. That way, you don't have to be alert to press the stop button just before your stop.
5. When writing complex automation with hundreds of steps, this is what we want to for testing.

