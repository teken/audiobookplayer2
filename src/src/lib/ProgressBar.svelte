<script lang="ts">
    import type { Segment } from "../types";

    export let disabled: boolean = false;
    export let position: number = 0;
    export let positionUpdate: (value: number) => void = () => {};
    export let segments: Segment[] = [];

    let barRef;

    let mouseX = 0;
    let indicatorPickedUp = false;
    document.addEventListener(
        "mousemove",
        (event: MouseEvent) => (mouseX = event.clientX)
    );
    const pickUp = (event: MouseEvent) => {
        if (disabled || event.button !== 1) return;
        indicatorPickedUp = true;
    };
    const putDown = () => {
        if (indicatorPickedUp) {
            indicatorPickedUp = false;
            const docX = barRef?.getBoundingClientRect()?.x ?? 0;
            positionUpdate(
                Math.min(
                    Math.max((mouseX - docX) / barRef?.clientWidth ?? 0, 0),
                    1
                )
            );
        }
    };
    document.addEventListener("mouseup", putDown);

    const setPosition = (event: MouseEvent) => {
        if (disabled || event.button !== 1) return;
        mouseX = event.clientX;
        const docX = barRef?.getBoundingClientRect()?.x ?? 0;
        positionUpdate(
            Math.min(Math.max((mouseX - docX) / barRef?.clientWidth ?? 0, 0), 1)
        );
    };
    const segmentBarGap = 0.5;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<svg
    width="100%"
    height="1rem"
    style="--color:var(--color1); box-sizing:border-box; overflow:visible;"
    bind:this={barRef}
    on:click={setPosition}
    on:mousedown={pickUp}
    class:ready={!disabled}
>
    <g>
        {#if segments.length == 0}
            <rect y="5" width="100%" height=".3rem" style="fill:var(--color)" />
        {:else}
            {#each segments as segment, index}
                {#if index === segments.length - 1}
                    <rect
                        y="5"
                        x="{segment.startPosition}%"
                        width="{segment.endPosition - segment.startPosition}%"
                        height=".3rem"
                        style="fill:var(--color)"
                    />
                {:else}
                    <rect
                        y="5"
                        x="{segment.startPosition}%"
                        width="{segment.endPosition -
                            segment.startPosition -
                            segmentBarGap / 2}%"
                        height=".3rem"
                        style="fill:var(--color)"
                    />
                {/if}
            {/each}
        {/if}
    </g>
    {#if !disabled}
        <circle
            class="handle"
            cx={indicatorPickedUp
                ? mouseX - (barRef?.getBoundingClientRect()?.x ?? 0) >
                  barRef.clientWidth
                    ? barRef.clientWidth
                    : mouseX - (barRef?.getBoundingClientRect()?.x ?? 0) < 0
                    ? 0
                    : mouseX - (barRef?.getBoundingClientRect()?.x ?? 0)
                : (position > 100 ? 100 : position < 0 ? 0 : position) + "%"}
            cy="7.5"
            r="5"
            style="fill:var(--color1); stroke:var(--color2); stroke-width:1"
        />
    {/if}
</svg>

<style>
    svg.ready {
        cursor: pointer;
    }

    svg g rect {
        height: 0.3rem;
    }

    /* svg g rect:hover {
        height: 0.5rem;
    } */
</style>
