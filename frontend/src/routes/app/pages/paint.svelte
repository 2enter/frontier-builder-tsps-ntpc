<script lang="ts">
    import type {ColorName} from '@/config';

    import moment from 'moment';
    import P5 from 'p5';
    import {onMount} from 'svelte';
    import {slide, fly, fade} from 'svelte/transition';
    import {randomItem} from '@2enter/web-kit/calc';
    import {ImgBtn} from '@2enter/web-kit/components';
    import {dexie} from '@/dexie';
    import {COLORS} from '@/config';
    import {getInputState, getSysState} from '@/states';

    const [inputState, sysState] = [getInputState(), getSysState()];

    const TOOLS = ['pen', 'brush', 'eraser'] as const;
    const WEIGHT_VALUES = [5, 20, 35, 50, 65, 80] as const;
    const MAX_VERSION = 10;

    type Tool = (typeof TOOLS)[number];

    let drawing = true;
    let p5: P5;

    let selectedTool = $state<Tool>('pen');
    let selectedWeight = $state(randomItem(Object.keys(WEIGHT_VALUES).map((i) => +i))[0]);
    let color = $state<ColorName>(randomItem(COLORS.map((c) => c.name))[0]);
    let weight = $derived(WEIGHT_VALUES[selectedWeight]);
    let trace = $state<[number, number][]>([]);
    let canvas = $state<HTMLCanvasElement>();
    let version = $state(0);
    let latestVersion = $state(0);

    let showUI = $state(true);
    let showManual = $state(true);

    const colorValue = $derived(COLORS.find((c) => c.name === color)?.value ?? '#262626');

    function takeScreenshot() {
        if (!canvas) return null;
        return canvas.toDataURL('image/png');
    }

    const noDraw = () => (drawing = false);

    function eraseAll() {
        if (!p5) return;
        p5.erase();
        p5.rect(0, 0, p5.windowWidth, p5.windowHeight);
        p5.noErase();
    }

    async function modifyVersion(action: -1 | 1) {
        if (!p5) return;
        version += action;
        if (version === 0) {
            eraseAll();
            return;
        }
        const dataUrl = await dexie.versions.get(version).then((data) => data?.value ?? null);
        if (!dataUrl) {
            console.log('image not found');
            version -= action;
            return;
        }
        p5.loadImage(dataUrl, (img) => {
            if (!p5) return;
            eraseAll();
            p5.image(img, 0, 0, p5.windowWidth, p5.windowHeight);
        });
    }

    const sketch = (p: P5) => {
        const SPRING = 0.8;
        const SPLIT_NUM = 100;
        const DIFF = 2.4;

        let [r, oldR, vx, vy, v] = [0, 0, 0, 0, 0.5];
        let [smallX, smallY] = [0, 0];
        let [oldX, oldY] = [0, 0];

        let friction = 0.45;

        p.setup = () => {
            p.createCanvas(p.windowWidth, p.windowHeight, 'p2d', canvas);
            p.frameRate(30);
        };

        p.draw = () => {
            p.strokeWeight(weight);
            p.stroke(colorValue);
        };

        p.touchMoved = () => {
            if (!drawing) return;
            showUI = false;

            let {mouseX: x, mouseY: y} = p;
            const last = trace.at(-1);
            trace.push([x, y]);
            if (!last) return;
            switch (selectedTool) {
                case 'pen':
                    p.line(...last, x, y);
                    break;
                case 'brush':
                    if (trace.length === 2) {
                        smallX = x;
                        smallY = y;
                    }
                    vx += (x - smallX) * SPRING;
                    vy += (y - smallY) * SPRING;
                    vx *= friction;
                    vy *= friction;

                    v += p.sqrt(vx * vx + vy * vy) - v;
                    v *= 0.55;

                    oldR = r;
                    r = weight - v;

                    for (let i = 0; i < SPLIT_NUM; i++) {
                        oldX = smallX;
                        oldY = smallY;
                        smallX += vx / SPLIT_NUM;
                        smallY += vy / SPLIT_NUM;
                        oldR += (r - oldR) / SPLIT_NUM;
                        if (oldR < 1) oldR = 1;
                        p.strokeWeight(oldR + DIFF);
                        p.line(
                            smallX + p.random(0, 2),
                            smallY + p.random(0, 2),
                            oldX + p.random(0, 2),
                            oldY + p.random(0, 2)
                        );
                        p.strokeWeight(oldR); // ADD
                        p.line(
                            smallX + DIFF * p.random(0.1, 2),
                            smallY + DIFF * p.random(0.1, 2),
                            oldX + DIFF * p.random(0.1, 2),
                            oldY + DIFF * p.random(0.1, 2)
                        );
                        p.line(
                            smallX - DIFF * p.random(0.1, 2),
                            smallY - DIFF * p.random(0.1, 2),
                            oldX - DIFF * p.random(0.1, 2),
                            oldY - DIFF * p.random(0.1, 2)
                        );
                    }
                    break;
                case 'eraser':
                    p.erase();
                    p.line(...last, x, y);
                    p.noErase();
                    break;
            }
        };

        // p.touchStarted = p.touchMoved;

        p.touchEnded = async () => {
            showUI = true;
            if (!drawing) {
                drawing = true;
                return;
            }
            if (trace.length < 2) return;
            if (latestVersion >= MAX_VERSION) {
                dexie.versions.delete(latestVersion - MAX_VERSION);
            }
            if (latestVersion > version) {
                for (let i = version + 1; i <= latestVersion; i++) {
                    dexie.versions.delete(i);
                }
                latestVersion = version;
            }
            trace = [];
            version++;
            latestVersion = Math.max(version, latestVersion);
            const screenshot = takeScreenshot();
            if (!screenshot) return;
            const old = await dexie.versions.get(version);
            if (old && screenshot) dexie.versions.update(version, {value: screenshot});
            else await dexie.versions.add({id: version, value: screenshot});
        };
    };

    onMount(() => {
        // dexie.versions.clear();
        p5 = new P5(sketch);

        const start = moment();

        return () => {
            p5?.remove();
            const timeDiff = moment().diff(start);
            inputState.paintTime = Math.floor(timeDiff / 1000);
            inputState.resultImgUrl = takeScreenshot();
            dexie.versions.clear();

            // for (let i = 0; i < 1000; ++i) {
            // 	if (i !== version) dexie.versions.delete(i);
            // }
        };
    });
</script>

<canvas bind:this={canvas}></canvas>

<div
        in:slide={{ axis: 'y' }}
        class="fixed bottom-0 flex h-[12vh] w-screen items-center justify-evenly bg-cover bg-center bg-no-repeat px-10 py-5"
        style:background-image="url(/ui/paint/tools.webp)"
>
    {#each TOOLS as tool}
        <input id={tool} type="radio" value={tool} hidden bind:group={selectedTool}/>
        <label
                for={tool}
                class="bg-contain bg-center bg-no-repeat"
                style:background-image="url({tool === selectedTool ? '/ui/paint/frame.webp' : ''})"
                ontouchstart={noDraw}
        >
            <img src="/ui/paint/{tool}.webp" class="" alt=""/>
        </label>
    {/each}
    <ImgBtn src="/ui/paint/undo.webp" class="" onclick={() => modifyVersion(-1)} ontouchstart={noDraw}
    ></ImgBtn>
    <ImgBtn src="/ui/paint/redo.webp" class="" onclick={() => modifyVersion(1)} ontouchstart={noDraw}
    ></ImgBtn>
    <ImgBtn
            src="/ui/paint/help.webp"
            class=""
            onclick={() => (showManual = true)}
            ontouchstart={noDraw}
    ></ImgBtn>
</div>

{#if showUI}
    <div transition:fade class="fixed right-0 top-0">
        {#if version !== 0}
            <ImgBtn
                    src="/ui/buttons/done.webp"
                    class="w-[30vw]"
                    ontouchstart={noDraw}
                    onclick={sysState.navigate}
            />
        {/if}
    </div>

    <div
            transition:fly={{ x: -100 }}
            class="pointer-events-none fixed left-1 z-[1000] flex flex-col justify-start gap-3"
    >
        <div class="flex w-fit flex-col items-start pl-2">
            {#each COLORS as {name}}
                <input type="radio" bind:group={color} value={name} id="color-{name}" hidden/>
                <label
                        ontouchstart={noDraw}
                        for="color-{name}"
                        class="pointer-events-auto transition-transform duration-100"
                        style:transform="scale({name === color ? 1.3 : 1})"
                >
                    <img class="size-18" src="/ui/paint/colors/{name}.webp" alt=""/>
                </label>
            {/each}
        </div>

        <div
                class="center-content mt-10 w-40 bg-cover bg-no-repeat"
                style:background-image="url(/ui/paint/bold/bg.webp)"
        >
            <ImgBtn
                    src="/ui/paint/bold/{selectedWeight + 1}.webp"
                    ontouchstart={noDraw}
                    onclick={() => (selectedWeight = (selectedWeight + 1) % WEIGHT_VALUES.length)}
                    class="pointer-events-auto"
            />
        </div>
    </div>
{/if}

<div
        class="full-screen center-content pointer-events-none bg-contain bg-center bg-no-repeat"
        style:background-image="url(/cargoes/{inputState.cargoKind}_canva.webp)"
></div>

{#if showManual}
    <button
            transition:fade
            class="full-screen center-content z-[2000] bg-black/60 px-8"
            ontouchstart={noDraw}
            onclick={() => (showManual = false)}
    >
        <img src="/ui/paint/manual.webp" alt=""/>
    </button>
{/if}
