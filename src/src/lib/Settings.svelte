<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { Store } from "tauri-plugin-store-api";
    import { open } from "@tauri-apps/api/dialog";
    import ImportProgressModal from "./ImportProgressModal.svelte";

    const store = new Store("settings.abp");
    let displayModal = false;

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
    };

    const invoke_modal_cmd = async (cmd: string) => {
        displayModal = true;
        invoke(cmd);
    };
</script>

<ImportProgressModal show={displayModal} />
<div class="actions">
    <span>Libaray Actions</span>
    <button on:click={() => invoke_modal_cmd("scan_folder")}>Scan Folder</button
    >
    <button on:click={() => invoke_modal_cmd("scan_metadata")}
        >Scan Metadata</button
    >
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

<style>
    .actions button {
        background-color: var(--color4);
        color: var(--color1);
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        cursor: pointer;
    }
</style>
