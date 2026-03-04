<script lang="ts">
    import {CargoKind} from '@/types/model';

    import {getSysState} from '@/states';
    import {ImgBtn} from '@2enter/web-kit/components';

    const sysState = getSysState();
    const CARGO_TYPES = Object.values(CargoKind);

    function choose() {
        sysState.navigate();
    }
</script>

<div class="center-content flex-wrap gap-3 px-14">
    <img src="/ui/texts/select_cargo.webp" alt="" onclick={choose}/>
    {#each CARGO_TYPES as type}
        <ImgBtn src="/cargoes/{type}_select.webp" class="w-2/5" onclick={() => choose(type)}></ImgBtn>
    {/each}
</div>
