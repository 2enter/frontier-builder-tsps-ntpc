<script lang="ts">
    import type {CargoInput} from '@/types/model';
    import {ImgBtn} from '@2enter/web-kit/components';
    import {makeSubmit} from '@2enter/web-kit/browser';
    import {getInputState, getSysState} from '@/states';
    import {postCargo} from '@/api';

    const [inputState, sysState] = [getInputState(), getSysState()];

    const lightBeam = (node: HTMLImageElement) => {
        let num = 2;
        const interval = setInterval(() => {
            num += num;
            node.style.marginBottom = `${num}px`;
            if (num > window.innerHeight * 1.5) {
                sysState.navigate();
            }
        }, 100);

        return {
            destroy() {
                clearInterval(interval);
            }
        };
    };

    const submit = makeSubmit({
        state: sysState,
        process: async () => {
            if (!inputState.submittable) return '資料不足';

            const file = await inputState.getPaint();
            if (!file) return '圖片上傳失敗';

            // extract metadata from `inputState`
            const {kind: cargoKind, paintTime} = inputState.requestMetadata as CargoInput;
            const input = {paintTime, cargoKind, file};

            // upload cargo image
            const {data: result} = await postCargo(input);

            if (!result) return '資料處理錯誤';

            // assign result to state
            inputState.result = result;
        }
    });
</script>

{#if !inputState.result}
    <img src={inputState.resultImgUrl} alt="" class="pointer-events-none fixed h-auto w-full"/>
    {#if !sysState.processing}
        <ImgBtn src="/ui/buttons/upload.webp" class="z-[2000]" onclick={submit}/>
    {/if}
{:else}
    <img use:lightBeam class="h-[50vh] w-screen" src="/ui/animations/light_beam.webp" alt=""/>
{/if}
