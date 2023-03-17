<script lang="ts">
    import { invoke, tauri, shell } from "@tauri-apps/api";
    import * as store from "../store";
    import type { Book } from "../types";
    import { Icon } from "svelte-fontawesome";
    import { faCaretUp } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";
    import { groupBy, secondsToFormatted } from "../util";
    import Loading from "./Loading.svelte";
    import Portal from "svelte-portal";
    import { push } from "svelte-spa-router";
    import type { Duration } from "../audioplayer";

    let searchText = "";
    store.search.subscribe((v) => (searchText = v));

    let displaySearchResults = false;

    let loadLibrary: Promise<Book[]> = Promise.resolve([]);
    let displayRightClickMenu = false;
    let rightClickedBook: Book;
    let loadRightClickedBookTime: Promise<number | null>;
    let rightClickXY = { x: 0, y: 0 };

    onMount(() => (loadLibrary = invoke("load_library")));

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
        else loadLibrary = invoke("load_library");
    };

    const startBook = async (book: Book) => {
        invoke("start_book", { workId: book.id });
    };

    const clearTime = async (book: Book) => {
        invoke("clear_book_time", { workId: book.id });
    };

    const openBook = (book: Book) => push(`/book/${book.id}`);

    const rightClickBook = async (event: MouseEvent, book: Book) => {
        rightClickedBook = book;
        displayRightClickMenu = true;
        rightClickXY = { x: event.clientX, y: event.y };
        loadRightClickedBookTime = invoke<number>("load_book_time", {
            workId: book.id,
        });

        // let maxY = event.clientY + rightClickMenu.clientHeight;
        //
        // if (maxY > rightClickOverlay.clientHeight) {
        //     rightClickXY = {
        //         x: event.clientX,
        //         y: rightClickOverlay.clientHeight - rightClickMenu.clientHeight,
        //     };
        //     console.log(
        //         event.clientY,
        //         rightClickMenu.clientHeight,
        //         rightClickOverlay.clientHeight
        //     );
        // } else {
        //     rightClickXY = { x: event.clientX, y: event.y };
        // }
    };

    const dispelRightClick = async () => {
        rightClickedBook = null;
        displayRightClickMenu = false;
        rightClickXY = { x: 0, y: 0 };
    };

    let libraryTopRef: HTMLDivElement;
</script>

<div bind:this={libraryTopRef}>
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
{#if displayRightClickMenu}
    <Portal>
        <button class="overlay" on:click={() => dispelRightClick()}>
            <div
                class="right-click-menu"
                style="left:{rightClickXY.x}px;top:{rightClickXY.y}px;"
            >
                <button on:click={() => startBook(rightClickedBook)}>
                    Play from Start
                </button>
                {#await loadRightClickedBookTime}
                    <button>Play from --:--:--</button>
                    <button>Clear Play time</button>
                {:then data}
                    {#if data != null}
                        <button on:click={() => startBook(rightClickedBook)}
                            >Play from {secondsToFormatted(data)}</button
                        >
                        <button on:click={() => clearTime(rightClickedBook)}
                            >Clear Play time</button
                        >
                    {/if}
                {/await}
                <button on:click={() => shell.open(rightClickedBook.path)}>
                    Show in File Explorer
                </button>
            </div>
        </button>
    </Portal>
{/if}

{#await loadLibrary}
    <div style="display: grid; place-items:center; height:80%;">
        <Loading />
    </div>
{:then data}
    <div class="library {displaySearchResults ? 'search-result' : ''}">
        {#if displaySearchResults}
            {#each data as book}
                <button
                    class="book-item"
                    on:dblclick={() => startBook(book)}
                    on:click={() => openBook(book)}
                    on:contextmenu|preventDefault={(e) =>
                        rightClickBook(e, book)}
                >
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
                </button>
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
                                    <button
                                        class="book-item"
                                        on:dblclick={() => startBook(book)}
                                        on:click={() => openBook(book)}
                                        on:contextmenu|preventDefault={(e) =>
                                            rightClickBook(e, book)}
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
                                    </button>
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
        box-shadow: 0 0 5px var(--color4); /* rgb(26, 137, 255);*/
        text-shadow: 0 0 5px var(--color4); /* rgb(26, 137, 255);*/
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
        height: 10rem;
        max-width: 10rem;
        max-height: 10rem;
        mask-image: 0.3rem;
        border-radius: 0.3rem;
        transition: height 0.2s ease-in-out, margin 0.2s ease-in-out;
    }

    .book-item:hover > .item-image {
        margin: 0.5rem;
        height: 9rem;
    }

    .return-to-top {
        position: fixed;
        right: 2rem;
        bottom: 4rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        background-color: var(--color4);
        color: var(--color1);
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        cursor: pointer;
        transition: border-color 0.25s;
    }

    .return-to-top:focus,
    .return-to-top:focus-visible {
        outline: 0.1rem solid var(--color1);
    }

    .return-to-top span {
        font-size: 0.8rem;
    }

    .overlay {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: rgb(0 0 0 / 46%);
    }

    .right-click-menu {
        position: absolute;
        padding: 0;
        margin: 0;
        background-color: var(--color5);
        z-index: 2;
        border-radius: 0.3rem;
        display: flex;
        flex-direction: column;
    }

    .right-click-menu button {
        padding: 0.3rem 1rem;
    }

    .right-click-menu button:hover {
        background-color: var(--color4);
        cursor: pointer;
    }

    .right-click-menu button:first-of-type:hover {
        border-radius: 0.3rem 0.3rem 0 0;
    }

    .right-click-menu button:last-of-type:hover {
        border-radius: 0 0 0.3rem 0.3rem;
    }
</style>
