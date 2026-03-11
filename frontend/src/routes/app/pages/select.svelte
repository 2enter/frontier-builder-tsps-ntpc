<script lang="ts">
    import {CargoKind} from '@/types/model';

    import {getSysState, getInputState} from '@/states';
    import {ImgBtn} from '@2enter/web-kit/components';

    const [sysState, inputState] = [getSysState(), getInputState()];

    const cargoKind = $derived(inputState.cargoKind);


    function choose() {
        sysState.navigate();
    }
</script>

<div class="center-content flex-wrap gap-3 px-14">
    <!--    <img src="/ui/texts/select_cargo.webp" alt="" onclick={choose}/>-->
    <!--    {#each CARGO_TYPES as type}-->
    <ImgBtn src="/cargoes/{cargoKind}_select.webp" class="w-2/5" onclick={ choose}></ImgBtn>
    <img src="/cargoes/{cargoKind}_description.webp" alt="" />


    <!--{/each}-->
</div>
