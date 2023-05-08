<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { open } from "@tauri-apps/api/dialog";
    import MultiSelect from "svelte-multiselect";

    import ImportProgressModal from "./ImportProgressModal.svelte";
    import { LibraryStyle, PossibleTags, Settings } from "../types";
    import { settings } from "../store";

    let displayModal = false;

    let libraryLocation: string;
    let libraryStyle: LibraryStyle;

    settings.subscribe((s) => {
        libraryLocation = s.library_location;
        libraryStyle = s.library_style;
    });

    const librarySelectFolder = async () => {
        await open({
            directory: true,
            multiple: false,
        }).then((v) => {
            if (!v) return;
            settings.update((x) => {
                x.library_location = v as string;
                return x;
            });
        });
    };
    const updateLibraryStyle = async () => {
        settings.update((x) => {
            x.library_style = libraryStyle;
            return x;
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
        bind:value={libraryLocation}
        on:click={librarySelectFolder}
    />
    <label for="libraryStyle">Library Style</label>
    <select
        name="libraryStyle"
        bind:value={libraryStyle}
        on:change={updateLibraryStyle}
    >
        <option value={LibraryStyle.Folder}>Folder</option>
        <option value={LibraryStyle.Metadata}>Metadata</option>
    </select>
    <!-- <fieldset>
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
    </fieldset> -->
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
