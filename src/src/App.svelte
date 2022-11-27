<script lang="ts">
  import Library from "./lib/Library.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api";
  import Router from "svelte-spa-router";
  import SlimPlayer from "./lib/SlimPlayer.svelte";

  const routes = {
    "/": Library,
  };
</script>

<header data-tauri-drag-region>
  <span>
    <button on:click={() => invoke("scan").then(() => invoke("load"))}
      >Scan</button
    >
    <button on:click={() => invoke("clear").then(() => invoke("load"))}
      >Clear</button
    >
  </span>
  <span>
    <button on:click={() => appWindow.minimize()}
      ><img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      /></button
    >
    <button on:click={() => appWindow.toggleMaximize()}
      ><img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      /></button
    >
    <button on:click={() => appWindow.close()}
      ><img
        src="https://api.iconify.design/mdi:close.svg"
        alt="close"
      /></button
    >
  </span>
</header>

<main>
  <Router {routes} restoreScrollState={true} />
</main>

<SlimPlayer />

<style>
  header {
    display: flex;
    justify-content: space-between;
  }

  main {
    overflow-y: auto;
    position: relative;
  }
</style>
