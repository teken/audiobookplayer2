<script lang="ts">
    import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    import { AudioPlayer } from "../audioplayer";

    let unlisteners: UnlistenFn[] = [];

    onDestroy(() => {
        unlisteners.forEach((unlisten) => unlisten());
    });

    onMount(async () => {
        await emit("background_player_ready");

        const player = new AudioPlayer();

        unlisteners = [
            await listen<string[]>("work_loaded", ({ payload }) => {
                player.load(payload);
            }),

            await listen("play", () => player.play()),
            await listen("pause", () => player.pause()),

            await listen("stop", () => player.unload()),

            await listen("step_forward", () => player.foward()),
            await listen("step_backward", () => player.backward()),

            await listen<{
                position: number;
                index: number;
            }>("set_file_position", ({ payload }) => {
                player.setPosition(payload);
            }),

            await listen<number>("set_volumn", ({ payload }) =>
                player.setVolumn(payload)
            ),
        ];
    });
</script>

This is a background window, you must know what you are doing to see this.
