<script lang="ts">
  import Library from "./lib/Library.svelte";
  import Book from "./lib/Book.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api";
  import Router, { push } from "svelte-spa-router";
  import SlimPlayer from "./lib/SlimPlayer.svelte";
  import BackgroundPlayer from "./lib/BackgroundPlayer.svelte";
  import { Icon } from "svelte-fontawesome";
  import {
    faBook,
    faGear,
    faQuestion,
    faWindowMinimize,
    faWindowMaximize,
    faWindowClose,
  } from "@fortawesome/free-solid-svg-icons";
  import Settings from "./lib/Settings.svelte";
  import About from "./lib/About.svelte";
  import { onMount } from "svelte";

  const routes = {
    "/": Library,
    "/book/:bookId": Book,
    "/settings": Settings,
    "/about": About,
    "/background-player": BackgroundPlayer,
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
      ><Icon icon={faWindowMinimize} /></button
    >
    <button on:click={() => appWindow.toggleMaximize()}
      ><Icon icon={faWindowMaximize} /></button
    >
    <button class="close" on:click={() => appWindow.close()}
      ><Icon icon={faWindowClose} /></button
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

  button {
    border: 0;
    background-color: transparent;
    padding: 0.6rem 0.7rem;
  }

  button:hover {
    border: 0;
    background-color: var(--color4); /*rgba(26, 26, 26, 0.5);*/
  }

  button:focus {
    outline: 0;
  }

  button.close:hover {
    background-color: rgb(197 0 0 / 85%);
  }
</style>
