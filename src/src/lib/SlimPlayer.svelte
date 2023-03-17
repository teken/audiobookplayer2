<script lang="ts">
    import { playerState as playerStateStore } from "../store";
    import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    import { Icon } from "svelte-fontawesome";
    import {
        faPlay,
        faPause,
        faForwardStep,
        faBackwardStep,
        faVolumeOff,
        faVolumeLow,
        faVolumeHigh,
        faVolumeXmark,
        faHeadphones,
        faAngleRight,
    } from "@fortawesome/free-solid-svg-icons";
    import ProgressBar from "./ProgressBar.svelte";
    import { secondsToFormatted } from "../util";
    import { PlayerState, type TrackMetadata } from "../audioplayer";
    import Loading from "./Loading.svelte";
    import type { Book } from "../types";

    let playerState = new PlayerState();
    playerStateStore.subscribe((x) => {
        playerState = x;
    });

    const volumnIcon = () => {
        if (playerState.volumn <= 0) return faVolumeOff;
        else if (playerState.volumn <= 0.5) return faVolumeLow;
        return faVolumeHigh;
    };

    const positionUpdater = (value) => {
        playerStateStore.update((old: PlayerState) => {
            old.updatePosition(value * old.duration);
            return old;
        });
    };

    const volumnUpdater = (value: number) => {
        playerStateStore.update((old: PlayerState) => {
            old.volumn = value * old.volumnMax;
            return old;
        });
    };

    let mouseX = 0;
    let displayTimeIndicator = false;
    const barMouseMove = (event: MouseEvent) => {
        mouseX = event.clientX;
    };

    let timeIndicator: HTMLDivElement;
    let bar: HTMLDivElement;

    const toggleMute = () => {
        playerStateStore.update((old) => {
            old.muted = !old.muted;
            return old;
        });
    };

    let unlisteners: UnlistenFn[] = [];

    onDestroy(() => {
        unlisteners.forEach((unlisten) => unlisten());
    });

    onMount(async () => {
        unlisteners = [
            await listen<Book>("work_loaded", (event) => {
                playerStateStore.update((old) => {
                    old.workId = event.payload.id;
                    old.ready = true;
                    return old;
                });
            }),
            await listen<TrackMetadata[]>("metadata_loaded", (event) => {
                playerStateStore.update((old) => {
                    old.fileMetadata = event.payload;
                    return old;
                });
            }),
            await listen("play", () =>
                playerStateStore.update((old) => {
                    old.playing = true;
                    return old;
                })
            ),
            await listen("pause", () =>
                playerStateStore.update((old) => {
                    old.playing = false;
                    return old;
                })
            ),
            await listen("stop", () =>
                playerStateStore.update((old) => {
                    old.ready = false;
                    return old;
                })
            ),
            await listen<{
                position: number;
                fileIndex: number;
            }>("update_file_position", (event) =>
                playerStateStore.update((old) => {
                    old.filePosition = event.payload.position;
                    old.fileIndexPosition = event.payload.fileIndex;
                    return old;
                })
            ),
        ];
    });
</script>

<footer>
    <div
        class="bar"
        on:mouseover={() => (displayTimeIndicator = true)}
        on:focus={() => (displayTimeIndicator = true)}
        on:mouseout={() => (displayTimeIndicator = false)}
        on:blur={() => (displayTimeIndicator = false)}
        on:mousemove={barMouseMove}
        bind:this={bar}
    >
        {#if playerState.ready && displayTimeIndicator}
            <div
                bind:this={timeIndicator}
                class="mouse-hover-indicator"
                style="left:{mouseX - (timeIndicator?.clientWidth ?? 0) / 2 < 0
                    ? 0
                    : mouseX >
                      (bar?.clientWidth ?? 0) -
                          (timeIndicator?.clientWidth ?? 0) / 2
                    ? (bar?.clientWidth ?? 0) -
                      (timeIndicator?.clientWidth ?? 0)
                    : mouseX - (timeIndicator?.clientWidth ?? 0) / 2}px;"
            >
                {secondsToFormatted(
                    (mouseX / (bar?.clientWidth ?? 0)) * playerState.duration
                )}
            </div>
        {/if}

        <ProgressBar
            disabled={!playerState.ready}
            position={playerState.positionAsPercentage}
            positionUpdate={positionUpdater}
            segments={playerState.chaptersAsSegments}
        />
    </div>

    <span class="left">
        <button
            disabled={!playerState.ready}
            on:click={() => emit("step_backward")}
            ><Icon icon={faBackwardStep} /></button
        >
        <button
            disabled={!playerState.ready}
            on:click={() =>
                playerState.playing ? emit("pause") : emit("play")}
            ><Icon icon={playerState.playing ? faPause : faPlay} /></button
        >
        <button
            disabled={!playerState.ready}
            on:click={() => emit("step_forward")}
            ><Icon icon={faForwardStep} /></button
        >
        {#if playerState.ready}
            <div>
                {playerState.positionFormatted} - {playerState.durationFormatted}
            </div>
        {/if}
    </span>
    <span class="right">
        <button on:click={() => toggleMute()}>
            <Icon
                icon={playerState.muted ? faVolumeXmark : volumnIcon()}
            /></button
        >
        <span style="width: 10rem;">
            <ProgressBar
                position={playerState.volumnAsPercentage}
                positionUpdate={volumnUpdater}
            />
        </span>
        <button><Icon icon={faHeadphones} /><Icon icon={faAngleRight} /></button
        >
    </span>
</footer>

<style>
    footer {
        display: grid;
        grid-template: 1rem auto / repeat(2, 1fr);
        grid-template-areas: "bar bar" "left right";
        justify-content: space-between;
    }

    .bar {
        grid-area: bar;
        margin-top: -0.39rem;
        z-index: 1;
        position: relative;
    }

    .left,
    .right {
        display: flex;
        align-items: center;
    }

    .left {
        grid-area: left;
    }

    .right {
        grid-area: right;
        justify-content: flex-end;
    }

    button {
        border: 0;
        background-color: transparent;
        padding: 0.6rem 0.7rem;
    }

    button:not(:disabled):hover {
        border: 0;
        background-color: var(--color4);
    }

    button:focus {
        outline: 0;
    }

    button:disabled {
        opacity: 0.5;
    }

    .mouse-hover-indicator {
        position: absolute;
        top: -1.8rem;
        background-color: rgba(0, 0, 0, 0.5);
        padding: 0.1rem 0.3rem;
        border-radius: 0.3rem;
    }
</style>
