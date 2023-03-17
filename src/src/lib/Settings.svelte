<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { Store } from "tauri-plugin-store-api";
    import { open } from "@tauri-apps/api/dialog";

    const store = new Store("settings.abp");

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
</script>

<div>
    Libaray Actions:
    <button on:click={() => invoke("scan")}>Scan</button>
    <button on:click={() => invoke("clear")}>Clear</button>
</div>
<form>
    <label for="libraryLocation">Library Location</label>
    <input
        name="libraryLocation"
        value={libraryPath}
        on:click={librarySelectFolder}
    />
</form>
