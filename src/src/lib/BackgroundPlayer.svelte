<script lang="ts">
    import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    import { AudioPlayer } from "../audioplayer";

    let loadUnlistener: UnlistenFn;
    let playUnlistener: UnlistenFn;
    let pauseUnlistener: UnlistenFn;
    let stopUnlistener: UnlistenFn;
    let forwardUnlistener: UnlistenFn;
    let backwardUnlistener: UnlistenFn;
    let setPositionUnlistener: UnlistenFn;
    let setVolumnUnlistener: UnlistenFn;

    onDestroy(() => {
        if (loadUnlistener) loadUnlistener();
        if (playUnlistener) playUnlistener();
        if (pauseUnlistener) pauseUnlistener();
        if (stopUnlistener) stopUnlistener();
        if (forwardUnlistener) forwardUnlistener();
        if (backwardUnlistener) backwardUnlistener();
        if (setPositionUnlistener) setPositionUnlistener();
        if (setVolumnUnlistener) setVolumnUnlistener();
    });

    onMount(async () => {
        await emit("background_player_ready");

        const player = new AudioPlayer();

        loadUnlistener = await listen<string[]>("load", ({ payload }) => {
            player.load(payload);
        });

        playUnlistener = await listen("play", () => player.play());
        pauseUnlistener = await listen("pause", () => player.pause());

        stopUnlistener = await listen("stop", () => player.unload());

        forwardUnlistener = await listen("step_forward", () => player.foward());
        backwardUnlistener = await listen("step_backward", () =>
            player.backward()
        );

        setPositionUnlistener = await listen<{
            position: number;
            index: number;
        }>("set_file_position", ({ payload }) => {
            player.setPosition(payload);
        });

        setVolumnUnlistener = await listen<number>(
            "set_volumn",
            ({ payload }) => player.setVolumn(payload)
        );
    });
</script>

This is a background window, you must know what you are doing to see this.
