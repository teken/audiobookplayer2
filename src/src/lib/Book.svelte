<script lang="ts">
  import { onMount } from "svelte";
  import { invoke, tauri, shell } from "@tauri-apps/api";
  import type { Book, TrackMetadata } from "../types";
  import { secondsToFormatted } from "../util";

  export let params: { bookId: string };

  let loadBook: Promise<Book>;
  let loadMetadata: Promise<TrackMetadata[]>;
  onMount(() => {
    loadBook = invoke("load_work", { workId: params.bookId }).then(
      (x: Book) => {
        loadMetadata = invoke("load_work_metadata", { workId: params.bookId });
        return x;
      }
    );
  });
</script>

{#await loadBook}
  Loading...
{:then book}
  {#if book}
    <div class="container">
      {#if book.image_files.length > 0}
        <img
          class="item-image"
          src={tauri.convertFileSrc(book.image_files[0])}
          alt="cover"
          loading="lazy"
        />
      {/if}
      <div>
        Title: {book.name} <br />
        Author: {book.author} <br />
        {#if book.series}
          Series: {book.series} <br />
        {/if}
        Path: {book.path} <br />
        Duration: {#await loadMetadata then metas}{secondsToFormatted(
            metas.reduce((a, v) => a + v.duration.secs, 0)
          )}{/await}
      </div>
      <div class="files">
        All files: <br />
        {#each book.files as file}
          {file} <br />
        {/each}
        <!-- Image files: {book.image_files} <br /> -->
      </div>
    </div>
  {/if}
{:catch error}
  Failed to load library: {error}
{/await}

<style>
  .container {
    display: grid;
    grid-template: auto auto / 1fr 1fr;
  }

  .files {
    grid-column: 1 / -1;
  }
</style>
