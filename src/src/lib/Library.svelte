<script lang="ts">
    import { invoke, tauri } from "@tauri-apps/api";
    import * as store from "../store";
    import type { Book } from "../types";
    import { Icon } from "svelte-fontawesome";
    import { faCaretUp } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";
    import { groupBy } from "../util";
    import Loading from "./Loading.svelte";

    let searchText = "";
    store.search.subscribe((v) => (searchText = v));

    let displaySearchResults = false;

    let loadLibrary: Promise<Book[]> = Promise.resolve([]);

    onMount(() => (loadLibrary = invoke("load")));

    const sortLibrary = (data: Book[]) => {
        const library = data.sort(
            (a: Book, b: Book) =>
                a.author.localeCompare(b.author) ||
                (a.series && b.series && a.series.localeCompare(b.series)) ||
                a.name.localeCompare(b.name)
        );
        const groupedByAuthor = groupBy<string, Book>(library, (x) => x.author);
        return [...groupedByAuthor.entries()]
            .sort((a, b) => a[0].localeCompare(b[0]))
            .map((x) => [
                x[0],
                [...groupBy<string, Book>(x[1], (y) => y.series).entries()],
            ]) as [string, [String, Book[]][]][];
    };

    const submit = (e) => {
        const formData = new FormData(e.target);

        store.search.update(() => formData.get("search").toString());
        displaySearchResults = searchText.length != 0;

        if (searchText) loadLibrary = invoke("search", { search: searchText });
        else loadLibrary = invoke("load");
    };

    const startBook = async (book: Book) => {
        console.log("start_book", { book });
        invoke("start_book", { book });
    };

    let libraryTopRef: HTMLDivElement;
</script>

<div bind:this={libraryTopRef}>
    <!-- <button on:click={() => (loadLibrary = invoke("load"))}>load</button> -->

    <form on:submit|preventDefault={submit} class="search-form">
        <input
            value={searchText}
            type="search"
            name="search"
            autocomplete="off"
            placeholder="book name..."
            style="padding: 1rem;font-size: larger;"
        />
    </form>
</div>

{#await loadLibrary}
    <Loading />
{:then data}
    <div class="library {displaySearchResults ? 'search-result' : ''}">
        {#if displaySearchResults}
            {#each data as book}
                <div class="book-item" on:dblclick={() => startBook(book)}>
                    {#if book.image_files.length > 0}
                        <img
                            class="item-image"
                            src={tauri.convertFileSrc(book.image_files[0])}
                            alt="cover"
                            loading="lazy"
                        />
                    {:else}
                        <img
                            class="item-image"
                            src="https://via.placeholder.com/160"
                            alt="cover-placeholder"
                            loading="lazy"
                        />
                    {/if}
                    <span class="label">{book.name}</span>
                    <span class="label">{book.series ?? ""}</span>
                    <span class="label">{book.author}</span>
                </div>
            {/each}
        {:else}
            {#each sortLibrary(data) as [author, series]}
                <div class="author-item">
                    <span class="label">{author}</span>
                    <div class="item-works">
                        {#each series as [serie, books]}
                            <div class="series-item">
                                {#if serie}
                                    <span class="label">{serie}</span>
                                {/if}
                                {#each books as book}
                                    <div
                                        class="book-item"
                                        on:dblclick={() => startBook(book)}
                                    >
                                        {#if book.image_files.length > 0}
                                            <img
                                                class="item-image"
                                                src={tauri.convertFileSrc(
                                                    book.image_files[0]
                                                )}
                                                alt="cover"
                                                loading="lazy"
                                            />
                                        {:else}
                                            <img
                                                class="item-image"
                                                src="https://via.placeholder.com/160"
                                                alt="cover-placeholder"
                                                loading="lazy"
                                            />
                                        {/if}
                                        <span class="label">{book.name}</span>
                                    </div>
                                {/each}
                            </div>
                        {/each}
                    </div>
                </div>
            {/each}
        {/if}
    </div>
    <button
        class="return-to-top"
        on:click={() => libraryTopRef.scrollIntoView()}
        ><Icon icon={faCaretUp} /><span>Return to Top</span></button
    >
{:catch error}
    Failed to load library: {error}
{/await}

<style>
    .search-form {
        width: 100%;
        display: flex;
        justify-content: center;
    }

    .search-form > input {
        width: 80%;
    }

    .library {
        display: grid;
        gap: 1rem;
        padding: 1rem;
    }

    .library.search-result {
        grid-template-columns: repeat(auto-fill, 10rem);
    }

    .author-item {
        display: inline-grid;
        grid-template-rows: fit-content auto;
        gap: 1rem;
    }

    .author-item > .label {
        padding-top: 1rem;
        padding-bottom: 0.8rem;
        font-size: xx-large;
    }

    .item-works {
        display: inline-flex;
        flex-direction: column;
    }

    .series-item {
        grid-row-start: 1;
        grid-row-end: -1;
        display: inline-grid;
        grid-template-columns: repeat(auto-fill, 10rem);
        gap: 1rem;
    }

    .series-item > .label {
        font-size: large;
        grid-row-start: 1;
        grid-row-end: -1;
    }

    .book-item:hover {
        /* color: #ffee10; */
        box-shadow: 0 0 5px rgb(26, 137, 255);
        text-shadow: 0 0 5px rgb(26, 137, 255);
    }

    .book-item {
        display: inline-grid;
        justify-items: center;
        cursor: pointer;
        border-radius: 0.3rem;
        box-shadow: 0 0 5px rgba(26, 137, 255, 0);
        text-shadow: 0 0 5px rgba(26, 137, 255, 0);
        transition: box-shadow 0.2s ease-in-out, text-shadow 0.2s ease-in-out;
    }

    .book-item > .label {
        text-align: center;
        padding: 0.5rem 0.2rem;
    }

    .book-item > .item-image {
        height: 160px;
        max-width: 160px;
        max-height: 160px;
        mask-image: 0.3rem;
        border-radius: 0.3rem;
        transition: height 0.2s ease-in-out, padding 0.2s ease-in-out;
    }

    .book-item:hover > .item-image {
        padding: 0.3rem;
        height: 150px;
    }

    .return-to-top {
        position: fixed;
        right: 2rem;
        bottom: 4rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        background-color: rgba(26, 26, 26, 0.8);
    }

    .return-to-top span {
        font-size: 0.8rem;
    }
</style>
