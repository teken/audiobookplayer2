<script lang="ts">
    import { app, shell, invoke } from "@tauri-apps/api";
    import type { Stats } from "../store";

    const dateRange = () => {
        const startDate = 2018;
        const endDate = new Date().getFullYear();
        if (startDate === endDate) {
            return startDate;
        } else {
            return `${startDate}-${endDate}`;
        }
    };

    const statsPromise: Promise<Stats> = invoke("library_stats");
</script>

<div class="container">
    <div class="info">
        {#await app.getName() then name}{name}{/await}: v{#await app.getVersion() then name}{name}{/await}<br
        />
        Created By Anna Haig<br />
        &copy; {dateRange()} Anna Haig All Rights Reserved<br />
    </div>
    <div class="links">
        <span
            class="link"
            on:click={() => shell.open("https://audiobookplayer.app")}
            >AudioBookPlayer.app</span
        >
        <span
            class="link"
            on:click={() =>
                shell.open("https://www.patreon.com/AudioBookPlayer")}
            >Patreon</span
        >
    </div>
    {#await statsPromise}
        Loading...
    {:then stats}
        <div class="stats">
            <span>Number of Authors: {stats.authors}</span>
            <span>Number of Series: {stats.series}</span>
            <span>Number of Books: {stats.books}</span>
            <span
                >Number of Books Not in a Series: {stats.booksNotInSeries}</span
            >
        </div>
    {/await}
</div>

<style>
    .container {
        display: flex;
        height: 100%;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        text-align: center;
    }

    .links {
        display: flex;
        gap: 2rem;
    }

    .link {
        cursor: pointer;
    }

    .stats {
        display: flex;
        flex-direction: column;
    }
</style>
