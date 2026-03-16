<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { navigating } from '$app/state';
	import { dev } from '$app/environment';
	import { setSysState, setInputState } from '@/states';
	import { getLaunchCountDown } from '@/time';
	import { Pages } from './pages';

	const sysState = setSysState();
	const inputState = setInputState();

	const Page = $derived(Pages[sysState.pageNum]);

	if (!inputState.cargoKind) {
		inputState.goToConfig();
	}


	onMount(async () => {
		if (dev) {
			const eruda = (await import('eruda')).default;
			eruda.init();
		}


		setInterval(() => {
			detectSWUpdate();
		}, 20000);

		return {
			destroy() {
				clearInterval(interval);
			}
		};
	});

	async function detectSWUpdate() {
		const registration = await navigator.serviceWorker.ready;

		registration.addEventListener('updatefound', () => {
			const newSW = registration.installing;

			newSW?.addEventListener('statechange', () => {
				if (newSW.state === 'installed') {
					if (confirm('New update available! Reload to update?')) {
						newSW.postMessage({ type: 'SKIP_WAITING' });

						window.location.reload();
					}
				}
			});
		});
	}
</script>


<div class="fixed bottom-3 z-[1000] w-full center-content">
	<img src="/ui/texts/2enter.webp" class="h-10" alt="" />
</div>

<div
	class="bg-cover bg-center bg-no-repeat full-screen center-content"
	style:background-image="url({sysState.bg})"
>
	{#key sysState.pageNum}
		<div in:fade class="flex-col center-content" class:mix-blend-multiply={sysState.pageNum !== 5}>
			<Page />
		</div>
	{/key}
</div>

<dialog bind:this={sysState.dialog} class="modal modal-middle">
	<div class="modal-box">
		<h1>Error</h1>
		<p>{sysState.errorMessage}</p>
		<div class="modal-action">
			<form method="dialog">
				<button class="btn btn-secondary" onclick={sysState.closeError}> Close</button>
			</form>
		</div>
	</div>
</dialog>

{#if sysState.processing || navigating.to}
	<div transition:fade class="z-[100000] bg-black/10 backdrop-blur-sm full-screen center-content">
		<i class="fa-solid fa-loader fa-duotone animate-spin text-8xl"></i>
	</div>
{/if}

{#if dev}
	<button
		class="btn btn-secondary fixed left-0 top-0 z-[3000]"
		onclick={() => window.location.reload()}
		>reload
	</button>
{/if}
