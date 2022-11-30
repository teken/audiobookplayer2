<script lang="ts">
  import Library from "./lib/Library.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api";
  import Router, { push } from "svelte-spa-router";
  import SlimPlayer from "./lib/SlimPlayer.svelte";
  import { Icon } from "svelte-fontawesome";
  import {
    faBook,
    faGear,
    faQuestion,
  } from "@fortawesome/free-solid-svg-icons";
  import Settings from "./lib/Settings.svelte";
  import About from "./lib/About.svelte";
  import { onMount } from "svelte";

  const routes = {
    "/": Library,
    "/settings": Settings,
    "/about": About,
  };

  onMount(() => invoke("close_splashscreen"));
</script>

<header data-tauri-drag-region>
  <span>
    <button on:click={() => push("/")}><Icon icon={faBook} /></button>
    <button on:click={() => push("/settings")}><Icon icon={faGear} /></button>
    <button on:click={() => push("/about")}><Icon icon={faQuestion} /></button>
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
    box-shadow: 0 0 1rem 1rem #00000017;
  }

  main {
    overflow-y: auto;
    position: relative;
    padding-top: 1rem;
  }
</style>
