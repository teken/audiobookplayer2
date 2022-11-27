<script lang="ts">
    export let position: number = 0;
    export let positionUpdate: (value: number) => void = () => {};

    let barRef;
    let mouseX = 0;
    let indicatorPickedUp = false;
    document.addEventListener(
        "mousemove",
        (event: MouseEvent) => (mouseX = event.clientX)
    );
    const pickUp = () => {
        indicatorPickedUp = true;
    };
    const putDown = () => {
        if (indicatorPickedUp) {
            indicatorPickedUp = false;
            positionUpdate(mouseX / barRef?.clientWidth ?? 0);
        }
    };
    document.addEventListener("mouseup", putDown);

    const setPosition = (event: MouseEvent) => {
        mouseX = event.clientX;
        positionUpdate(mouseX / barRef?.clientWidth ?? 0);
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<svg
    width="100%"
    height="1rem"
    style="--color:rgb(26,137,255)"
    class="bar"
    bind:this={barRef}
    on:click={setPosition}
    on:mousedown={pickUp}
>
    <rect y="5" width="100%" height=".3rem" style="fill:var(--color)" />
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
        style="fill:var(--color)"
    />
</svg>

<style>
    .bar {
        cursor: pointer;
    }
</style>
