<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import Router from "svelte-spa-router";
  import { useRegisterSW } from "virtual:pwa-register/svelte";

  import About from "./lib/About.svelte";
  import BackgroundPlayer from "./lib/BackgroundPlayer.svelte";
  import Book from "./lib/Book.svelte";
  import Header from "./lib/Header.svelte";
  import Library from "./lib/Library.svelte";
  import Settings from "./lib/Settings.svelte";
  import SlimPlayer from "./lib/SlimPlayer.svelte";
  import { settings } from "./store";
  import type { Settings as SettingsType } from "./types";

  const routes = {
    "/": Library,
    "/book/:bookId": Book,
    "/settings": Settings,
    "/about": About,
    "/background-player": BackgroundPlayer,
  };

  onMount(() => invoke("close_splashscreen"));

  useRegisterSW({
    onRegistered(r) {
      r &&
        setInterval(() => {
          r.update();
        }, 60 * 60 * 1000);
    },
  });

  invoke("load_settings").then((v: SettingsType) => {
    settings.set(v);
  });

  settings.subscribe((v) => {
    if (!v) return;
    try {
      invoke("save_settings", { new_settings: v });
    } catch (e) {
      console.error(e);
    }
  });
</script>

<Header />
<main>
  <Router {routes} restoreScrollState={true} />
</main>

<SlimPlayer />

<style>
  main {
    overflow-y: auto;
    position: relative;
    padding-top: 1rem;
  }
</style>
