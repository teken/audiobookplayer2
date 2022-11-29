<script>
    import { invoke } from "@tauri-apps/api";
</script>

<div>
    Libaray Actions:
    <button on:click={() => invoke("scan")}>Scan</button>
    <button on:click={() => invoke("clear")}>Clear</button>
</div>
<form>
    <label for="libraryLocation">Library Location</label>
    <input name="libraryLocation" />
</form>
