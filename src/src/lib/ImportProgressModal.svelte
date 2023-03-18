<script lang="ts">
  import Portal from "svelte-portal";
  import { onDestroy, onMount } from "svelte";
  import { listen, once, type UnlistenFn } from "@tauri-apps/api/event";
  import { shell } from "@tauri-apps/api";

  export let show = false;

  let unlisteners: UnlistenFn[] = [];

  let state: "waiting" | "started" | "finished" = "waiting";
  let filesToScan = 0;
  let filesCompleted = 0;
  let filesFailed = 0;
  let filesFailedPath = {
    fileRead: [],
    tagRead: [],
    authorRead: [],
    albumRead: [],
  };

  onDestroy(() => {
    unlisteners.forEach((unlisten) => unlisten());
  });

  onMount(async () => {
    unlisteners = [
      await listen("scan_metadata_finding_files", () => {
        state = "started";
        filesToScan = 0;
        filesCompleted = 0;
        filesFailed = 0;
        filesFailedPath = {
          fileRead: [],
          tagRead: [],
          authorRead: [],
          albumRead: [],
        };
      }),
      await listen(
        "scan_metadata_files_found",
        ({ payload }) => (filesToScan = payload as number)
      ),
      await listen("scan_metadata_file_failed_read", ({ payload }) => {
        filesFailedPath.fileRead = [
          ...filesFailedPath.fileRead,
          payload as string,
        ];
      }),
      await listen("scan_metadata_file_failed_tag_read", ({ payload }) => {
        filesFailedPath.tagRead = [
          ...filesFailedPath.tagRead,
          payload as string,
        ];
      }),
      await listen("scan_metadata_file_failed_author_read", ({ payload }) => {
        filesFailedPath.authorRead = [
          ...filesFailedPath.authorRead,
          payload as string,
        ];
      }),
      await listen("scan_metadata_file_failed_album_read", ({ payload }) => {
        filesFailedPath.albumRead = [
          ...filesFailedPath.albumRead,
          payload as string,
        ];
      }),
      await listen("scan_metadata_file_complete", () => filesCompleted++),
      await listen("scan_metadata_complete", () => (state = "finished")),
    ];
  });
</script>

{#if show}
  <Portal>
    <div class="overlay">
      <div class="modal">
        {#if state == "waiting"}
          waiting
        {:else if state == "started"}
          <progress value={filesCompleted + filesFailed} max={filesToScan}>
            32%
          </progress>
          <span>{filesCompleted}+{filesFailed}</span><span>/{filesToScan}</span>
        {:else}
          finished
          <button on:click={() => (show = false)}>Close modal</button>
        {/if}

        <div>
          {#if filesFailedPath.fileRead.length > 0}
            <fieldset>
              <legend>Failed to read file:</legend>
              {#each filesFailedPath.fileRead as file}
                <div>{file}</div>
              {/each}
            </fieldset>
          {/if}
          {#if filesFailedPath.tagRead.length > 0}
            <fieldset>
              <legend>Failed to read tags:</legend>
              {#each filesFailedPath.tagRead as file}
                <div>{file}</div>
              {/each}
            </fieldset>
          {/if}
          {#if filesFailedPath.authorRead.length > 0}
            <fieldset>
              <legend>Failed to read author tag:</legend>
              {#each filesFailedPath.authorRead as file}
                <div>{file}</div>
              {/each}
            </fieldset>
          {/if}
          {#if filesFailedPath.albumRead.length > 0}
            <fieldset>
              <legend>Failed to read album tag:</legend>
              {#each filesFailedPath.albumRead as file}
                <div>{file}</div>
              {/each}
            </fieldset>
          {/if}
        </div>
      </div>
    </div>
  </Portal>
{/if}

<style>
  .overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgb(0 0 0 / 46%);
    display: grid;
    place-items: center center;
  }

  .modal {
    background-color: var(--color5);
    padding: 1rem;
  }

  .modal > div {
    max-height: 60vh;
    overflow-y: auto;
  }
</style>
