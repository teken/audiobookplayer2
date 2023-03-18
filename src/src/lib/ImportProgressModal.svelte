<script lang="ts">
  import Portal from "svelte-portal";
  import { onDestroy, onMount } from "svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import ProgressBar from "./ProgressBar.svelte";
  
  export let show = false;
  
  let unlisteners: UnlistenFn[] = [];
  
  let state: "waiting" | "started" | "finsished" = "started";
  let filesToScan = 100;
  let filesCompleted = 20;
  let filesFailed = 30;
  let filesFailedPath = new Map<string, string[]>([
    ["fileRead",["sdd"]],
    ["tagRead",["dff"]],
    ["authorRead",["sdf"]],
    ["albumRead",["xc"]]
  ]);

  onDestroy(() => {
      unlisteners.forEach((unlisten) => unlisten());
  });

  onMount(async () => {
      unlisteners = [
        await listen("scan_metadata_finding_files", () => state = "started"),
        await listen("scan_metadata_files_found", ({payload}) => filesToScan = payload),
        await listen("scan_metadata_file_failed_read", ({payload}) => filesFailedPath.get("fileRead").push(payload)),
        await listen("scan_metadata_file_failed_tag_read", ({payload}) => filesFailedPath.get("tagRead").push(payload)),
        await listen("scan_metadata_file_failed_author_read", ({payload}) => filesFailedPath.get("authorRead").push(payload)),
        await listen("scan_metadata_file_failed_album_read", ({payload}) => filesFailedPath.get("albumRead").push(payload)),
        await listen("scan_metadata_file_complete", () => filesCompleted++),
        await listen("scan_metadata_complete", () => state = "finished"),
      ];
  });
</script>

{#if show}
    <Portal>
        <div class="overlay">
            <div class="modal">
              {#if (state == 'waiting')}
                waiting
              {:else if (state == 'started')}
                <progress value={filesCompleted+filesFailed} max={filesToScan}> 32% </progress>
                <span>{filesCompleted}+{filesFailed}</span><span>/{filesToScan}</span>
              {:else}
                finished
              {/if}
              
              <div>
                  {#if filesFailedPath.get("fileRead").length > 0}
                    <fieldset>
                      <legend>Failed to read file:</legend>
                      {#each filesFailedPath.get("fileRead") as file}
                        <div>{file}</div>
                      {/each}
                    </fieldset>
                  {/if}
                  {#if filesFailedPath.get("tagRead").length > 0}
                    <fieldset>
                      <legend>Failed to read tags:</legend>
                      {#each filesFailedPath.get("tagRead") as file}
                        <div>{file}</div>
                      {/each}
                    </fieldset>
                  {/if}
                  {#if filesFailedPath.get("authorRead").length > 0}
                    <fieldset>
                      <legend>Failed to read author tag:</legend>
                      {#each filesFailedPath.get("authorRead") as file}
                        <div>{file}</div>
                      {/each}
                    </fieldset>
                  {/if}
                  {#if filesFailedPath.get("albumRead").length > 0}
                    <fieldset>
                      <legend>Failed to read album tag:</legend>
                      {#each filesFailedPath.get("albumRead") as file}
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
        display:grid;
        place-items:center center;
    }
    
    .modal {
      background-color: var(--color5);
      padding:1rem;
    }
</style>