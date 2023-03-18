<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { Store } from "tauri-plugin-store-api";
    import { open } from "@tauri-apps/api/dialog";
    import ImportProgressModal from "./ImportProgressModal.svelte";

    const store = new Store("settings.abp");
    let displayModal = true;

    let libraryPath = "";
    store.get("libraryPath").then((v: string) => {
        libraryPath = v ?? "";
    });

    const librarySelectFolder = async () => {
        await open({
            directory: true,
            multiple: false,
        }).then((v) => {
            if (!v) return;
            libraryPath = v as string;
            store.set("libraryPath", libraryPath);
            store.save();
        });
    };

    const invoke_cmd = async (cmd: string) => {
        invoke(cmd);
        displayModal = true;
    };
</script>

<ImportProgressModal show={displayModal}/>
<div>
    Libaray Actions:
    <button on:click={() => invoke_cmd("scan_folder")}>Scan Folder</button>
    <button on:click={() => invoke_cmd("scan_metadata")}>Scan Metadata</button>
    <button on:click={() => invoke_cmd("clear_library")}>Clear Library</button>
    <button on:click={() => invoke_cmd("clear_times")}>Clear Times</button>
</div>
<form>
    <label for="libraryLocation">Library Location</label>
    <input
        name="libraryLocation"
        value={libraryPath}
        on:click={librarySelectFolder}
    />
</form>
