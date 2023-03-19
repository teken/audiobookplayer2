<script lang="ts">
  import Portal from "svelte-portal";
  import { onDestroy, onMount } from "svelte";
  import { listen, once, type UnlistenFn } from "@tauri-apps/api/event";
  import { shell } from "@tauri-apps/api";

  import { Icon } from "svelte-fontawesome";
  import { faFolderOpen } from "@fortawesome/free-solid-svg-icons";

  export let legend = "";
  export let files: string[] = [];
</script>

{#if files.length > 0}
  <fieldset>
    <legend>{legend} ({files.length}):</legend>
    {#each files as file}
      <div>
        {file}<span class="actions"
          ><button
            on:click={() =>
              shell.open(
                file.slice(
                  0,
                  (file.lastIndexOf("/") !== -1
                    ? file.lastIndexOf("/")
                    : file.lastIndexOf("\\")) + 1
                )
              )}><Icon icon={faFolderOpen} /></button
          ></span
        >
      </div>
    {/each}
  </fieldset>
{/if}

<style>
  .actions {
    margin-left: 1rem;
  }

  .actions button {
    cursor: pointer;
  }
</style>
