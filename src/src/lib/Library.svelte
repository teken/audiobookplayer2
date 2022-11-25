<script lang="ts">
    import { invoke, tauri } from "@tauri-apps/api";
    import * as store from "../store";
    import type { Book } from "../types";

    let searchText = "";
    store.search.subscribe((v) => (searchText = v));

    let displaySearchResults = false;
    store.displaySearchResults.subscribe((x) => (displaySearchResults = x));

    let loadLibrary: Promise<Book[]> = invoke("load");

    const sortLibrary = (data) => {
        const library = data.sort(
            (a, b) =>
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
            ]);
    };

    const groupBy = <K, T>(list: T[], selector: (x) => K) => {
        return list.reduce((a, v) => {
            const sv = selector(v);
            if (!a.has(sv)) a.set(sv, [v]);
            else {
                let nv = a.get(sv);
                a.set(sv, [...nv, v]);
            }

            return a;
        }, new Map<K, T[]>());
    };

    const submit = (e) => {
        const formData = new FormData(e.target);

        store.search.update(() => formData.get("search").toString());
        store.displaySearchResults.update(() => searchText.length != 0);

        if (searchText) loadLibrary = invoke("search", { search: searchText });
        else loadLibrary = invoke("load");
    };

    const startBook = async (book: Book) => {
        console.log("start_book", { book });
        invoke("start_book", { book });
    };
</script>

<button on:click={() => (loadLibrary = invoke("load"))}>load</button>

<form on:submit|preventDefault={submit}>
    <input value={searchText} type="search" name="search" autocomplete="off" />
</form>

{#await loadLibrary}
    Loading ...
{:then data}
    library
    <div class="library {displaySearchResults ? 'search-result' : ''}">
        {#if displaySearchResults}
            {#each data as book}
                <div class="book-item" on:dblclick={() => startBook(book)}>
                    <img
                        src="https://via.placeholder.com/160"
                        alt="cover-placeholder"
                    />
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
                                            />
                                        {:else}
                                            <img
                                                class="item-image"
                                                src="https://via.placeholder.com/160"
                                                alt="cover-placeholder"
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
{:catch error}
    Failed to load library: {error}
{/await}

<style>
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

    .book-item {
        display: inline-grid;
        justify-items: center;
        cursor: pointer;
    }

    .book-item > .label {
        text-align: center;
        padding: 0.5rem 0.2rem;
    }

    .book-item > .item-image {
        height: 160px;
        max-width: 160px;
        max-height: 160px;
    }
</style>
