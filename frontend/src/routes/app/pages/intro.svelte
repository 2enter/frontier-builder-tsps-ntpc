<script lang="ts">
    import {ImgBtn} from '@2enter/web-kit/components';
    import {getSysState, getInputState} from '@/states';

    const sysState = getSysState();
    const inputState = getInputState();

</script>

<img class="pointer-events-none fixed w-full" src="/ui/layouts/intro_page.webp" alt=""/>
<ImgBtn src="/ui/buttons/start.webp" class="fixed bottom-[30vh] w-1/2" onclick={sysState.navigate}
></ImgBtn>

<div
        class="fixed bottom-0 left-0 size-3"
        aria-label="Double click to configure"
        role="button"
        tabindex="0"
        ondblclick={inputState.goToConfig}
></div>
