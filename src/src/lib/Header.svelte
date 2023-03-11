<script lang="ts">
    import {
        faBook,
        faGear,
        faQuestion,
        faWindowClose,
        faWindowMaximize,
        faWindowMinimize,
    } from "@fortawesome/free-solid-svg-icons";
    import { appWindow } from "@tauri-apps/api/window";
    import { Icon } from "svelte-fontawesome";
    import { push } from "svelte-spa-router";
</script>

<header data-tauri-drag-region>
    <span>
        <button on:click={() => push("/")}><Icon icon={faBook} /></button>
        <button on:click={() => push("/settings")}
            ><Icon icon={faGear} /></button
        >
        <button on:click={() => push("/about")}
            ><Icon icon={faQuestion} /></button
        >
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

<style>
    header {
        display: flex;
        justify-content: space-between;
        box-shadow: 0 0 1rem 1rem #00000017;
    }

    button {
        border: 0;
        background-color: transparent;
        padding: 0.6rem 0.7rem;
    }

    button:hover {
        border: 0;
        background-color: var(--color4);
    }

    button:focus {
        outline: 0;
    }

    button.close:hover {
        background-color: var(--color6);
    }
</style>
