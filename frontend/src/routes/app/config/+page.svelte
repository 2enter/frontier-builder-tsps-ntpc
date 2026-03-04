<script lang="ts">
    import {CargoKind} from '@/types/model';
    import {setInputState} from '@/states';

    let inputState = setInputState();

    function doIt(kind: CargoKind) {
        inputState.setCargoKind(kind);
        window.location.href = '/app';
    }
</script>

<div class="gap-3 full-screen center-content">
    {#each Object.values(CargoKind) as kind}
        <button class="btn" onclick={() => doIt(kind)}>
            {kind}
        </button>
    {/each}
</div>
