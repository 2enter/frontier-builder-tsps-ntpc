<script lang="ts">
	import { CargoKind } from '@/types/model';

	import { getSysState, getInputState } from '@/states';
	import { ImgBtn } from '@2enter/web-kit/components';

	const [sysState, inputState] = [getSysState(), getInputState()];

	function choose(kind: CargoKind) {
		inputState.cargoKind = kind;
		sysState.navigate();
	}
</script>

<div class="flex-wrap gap-3 px-14 center-content">
	<!--    <img src="/ui/texts/select_cargo.webp" alt="" onclick={choose}/>-->
	<div class="flex flex-row flex-wrap w-[85vw] items-content justify-between gap-4">
		{#each Object.values(CargoKind) as kind}
			<div class="flex flex-col center-content gap-2">
				<ImgBtn src="/cargoes/{kind}_select.webp" class="w-[30vw]" onclick={() => choose(kind)}
				></ImgBtn>
				<img src="/cargoes/{kind}_description.webp" class="h-24 max-w-[40vw]" alt="" />
			</div>
		{/each}
	</div>
</div>
