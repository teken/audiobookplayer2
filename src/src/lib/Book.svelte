<script lang="ts">
  import { onMount } from "svelte";
  import { invoke, tauri, shell } from "@tauri-apps/api";
  
  export let params: { bookId:string };
  
  let loadBook: Promise<Book>;
  let loadMetadata: Promise<any>;
  onMount(() => {
    loadBook = invoke("load_work", {workId:params.bookId});
    loadMetadata = invoke("load_work_metadata", {workId:params.bookId});
  });
  
</script>

{#await loadBook}
  Loading...
{:then book}
  {#if book}
    <div class="container">
      <img
        class="item-image"
        src={tauri.convertFileSrc(book.image_files[0])}
        alt="cover"
        loading="lazy"
      />
      <div>
        Title: {book.name} <br/>
        Author: {book.author} <br/>
        Series: {book.series} <br/>
        Path: {book.path} <br/>
      </div>
      <div>
        All files: {book.files} <br/>
        Image files: {book.image_files} <br/>
      </div>
    </div>
  {/if}
{:catch error}
    Failed to load library: {error}
{/await}

<style>
.container {
  display: grid;
  grid-template: 1fr 1fr / 1fr 1fr;
}
</style>