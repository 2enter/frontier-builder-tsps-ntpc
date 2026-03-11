<script lang="ts">
	import * as THREE from 'three';
	import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

	import { getInputState } from '@/states';
	import { onDestroy, onMount } from 'svelte';

	import { ImgBtn } from '@2enter/web-kit/components';
	import type { MeshToonMaterialParameters } from 'three';

	const inputState = getInputState();

	const FRAME_RATE = 30;
	const MAX_IDLE_SECS = 30;

	const gltfLoader = new GLTFLoader();
	const textureLoader = new THREE.TextureLoader();
	const scene = new THREE.Scene();

	let threeDom = $state<HTMLDivElement>();
	let cargoModel: THREE.Object3D;
	let frame = 0;
	let idleClock: Timer;

	onMount(async () => {
		if (!inputState.result) {
			console.log('result not found');
			return;
		}

		let startTime = Date.now();
		idleClock = setInterval(() => {
			if (Date.now() - startTime > MAX_IDLE_SECS * 1000) {
				window.location.reload();
			}
		}, 1000);

		const { kind, id } = $state.snapshot(inputState.result);

		inputState.reset();

		cargoModel = await gltfLoader.loadAsync(`/cargoes/${kind}.glb`).then(({ scene }) => scene);
		console.log(cargoModel);
		const texture = await textureLoader.loadAsync(`/api/storage/texture/${id}.jpg`);
		const alphaTexture = await textureLoader.loadAsync(`/cargoes/butterfly_alpha.webp`);

		const camera = new THREE.PerspectiveCamera(
			75,
			window.innerWidth / window.innerHeight,
			0.1,
			1000
		);
		const renderer = new THREE.WebGLRenderer({ alpha: true });

		renderer.setSize(window.innerWidth, window.innerHeight);
		const light = new THREE.PointLight('white', 80);
		light.position.set(2, 2, 4.5);
		// light.lookAt(0, 0, 0);
		scene.add(light);

		if (threeDom) threeDom.appendChild(renderer.domElement);
		texture.flipY = false;
		alphaTexture.flipY = false;
		// alphaTexture.magFilter = THREE.NearestFilter;
		const material = new THREE.MeshToonMaterial(
			kind === 'butterfly'
				? ({
						transparent: true,
						map: texture,
						alphaMap: alphaTexture,
						alphaTest: 0.5
					} satisfies MeshToonMaterialParameters)
				: { map: texture }
		);

		const cargo = cargoModel.children[0];
		cargo.renderOrder = 1;
		if ('material' in cargo) {
			cargo.material = material;
		}

		if (kind === 'leaf') {
			cargo.rotation.y += Math.PI / 4;
			cargo.rotation.x += Math.PI / 2;
			camera.position.y = 6.5;
			camera.position.z = 3.6;
			camera.lookAt(-1.5, 0, 0);
		} else if (kind === 'butterfly') {
			cargo.rotation.y += Math.PI / 2;
			camera.position.y = 4.2;
			// camera.position.z = 3.6;
			camera.lookAt(0, 0, 0);
		} else {
			camera.position.y = 1.5;
			camera.position.z = 3.6;
			camera.lookAt(0, 0, 0);
		}

		scene.add(cargo);

		function animate() {
			setTimeout(() => {
				frame = requestAnimationFrame(animate);
				renderer.render(scene, camera);
				if (kind === 'leaf') {
					cargo.rotation.z += 1 / FRAME_RATE;
				} else {
					cargo.rotation.y += 1 / FRAME_RATE;
				}
				// cargo.rotation.x += 0.2 / FRAME_RATE;
			}, 1000 / FRAME_RATE);
		}

		animate();
	});

	onDestroy(() => {
		scene.clear();
		clearInterval(idleClock);
		cancelAnimationFrame(frame);
	});
</script>

<div bind:this={threeDom} class="z-[1000] full-screen"></div>

<div class="flex flex-col justify-between px-12 py-40 full-screen">
	<img src="/ui/texts/upload_success.webp" alt="" />
	<img src="/ui/texts/head_up.webp" alt="" />
</div>

<ImgBtn
	class="fixed bottom-12 z-[3000] w-56"
	src="/ui/buttons/restart.webp"
	onclick={() => {
		window.location.reload();
	}}
/>
