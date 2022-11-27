<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { playerState as playerStateStore, PlayerState } from "../store";

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

    let playerState = new PlayerState();
    playerStateStore.subscribe((x) => (playerState = x));

    const volumnIcon = () => {
        if (playerState.volumn <= 0) return faVolumeOff;
        else if (playerState.volumn <= 0.5) return faVolumeLow;
        return faVolumeHigh;
    };

    const positionUpdater = (value) => {
        playerStateStore.update((old: PlayerState) => {
            old.position = value * old.duration;
            return old;
        });
    };

    const volumnUpdater = (value) => {
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

    let timeIndicator;
    let bar;
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
        {#if displayTimeIndicator}
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
            position={playerState.positionAsPercentage}
            positionUpdate={positionUpdater}
        />
    </div>

    <span class="left">
        <button on:click={() => invoke("play")}
            ><Icon icon={faBackwardStep} /></button
        >
        <button on:click={() => invoke("pause")}
            ><Icon icon={playerState.playing ? faPause : faPlay} /></button
        >
        <button on:click={() => invoke("stop")}
            ><Icon icon={faForwardStep} /></button
        >
        <div>
            {playerState.positionFormatted} - {playerState.durationFormatted}
        </div>
    </span>
    <span class="right">
        <button>
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
        align-items: baseline;
        gap: 1rem;
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
    }

    button:hover {
        border: 0;
        background-color: rgba(26, 26, 26, 0.5);
    }

    button:focus {
        outline: 0;
    }

    .mouse-hover-indicator {
        position: absolute;
        top: -1.8rem;
        background-color: rgba(0, 0, 0, 0.5);
        padding: 0.1rem 0.3rem;
        border-radius: 0.3rem;
    }
</style>
