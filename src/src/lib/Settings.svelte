<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { Store } from "tauri-plugin-store-api";
    import { open } from "@tauri-apps/api/dialog";
    import MultiSelect from "svelte-multiselect";

    import ImportProgressModal from "./ImportProgressModal.svelte";
    import { PossibleTags } from "../types";

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

    let selectedAuthorTags = [];
    let selectedBookTags = [];
</script>

<ImportProgressModal show={displayModal} />
<fieldset class="actions">
    <legend>Libaray Actions</legend>
    <button on:click={() => invoke_modal_cmd("scan_folder")}>Scan Folder</button
    >
    <button on:click={() => invoke_modal_cmd("scan_metadata")}
        >Scan Metadata</button
    >
    <button on:click={() => invoke_cmd("clear_library")}>Clear Library</button>
    <button on:click={() => invoke_cmd("clear_times")}>Clear Times</button>
</fieldset>
<form>
    <label for="libraryLocation">Library Location</label>
    <input
        name="libraryLocation"
        value={libraryPath}
        on:click={librarySelectFolder}
    />
    <fieldset>
        <legend>Metadata Scan Settings</legend>
        <label for="authorTagSelect">Possible Author Tags</label>
        <MultiSelect
            id="authorTagSelect"
            bind:selectedAuthorTags
            options={PossibleTags}
        />
        <label for="bookTagSelect">Possible Book Tags</label>
        <MultiSelect
            id="bookTagSelect"
            bind:selectedBookTags
            options={PossibleTags}
        />
    </fieldset>
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
