import type { ParseEnum } from '@2enter/web-kit/types';
import type { Cargo, CargoKind } from '@/types/model';

import { getContext, setContext } from 'svelte';
import axios from 'axios';

class InputState {
	cargoKind = $state<ParseEnum<CargoKind> | null>(null);
	paintTime = $state(0);

	result = $state<Cargo | null>(null);
	resultImgUrl = $state<string | null>(null);

	readonly submittable = $derived(this.cargoKind && this.paintTime);
	readonly requestMetadata = $derived({ kind: this.cargoKind, paintTime: this.paintTime });

	reset = () => {
		this.cargoKind = null;
		this.paintTime = 0;
		this.result = null;
		this.resultImgUrl = null;
	};

	async getPaint() {
		const url = this.resultImgUrl;
		if (!url) return null;
		return await axios.get<Blob>(url, { responseType: 'blob' }).then((res) => res.data);
		// return new File([blob], 'paint.png');
	}
}

const INPUT_STATE_CTX = 'INPUT_STATE';

function setInputState() {
	return setContext(INPUT_STATE_CTX, new InputState());
}

function getInputState() {
	return getContext<ReturnType<typeof setInputState>>(INPUT_STATE_CTX);
}

export { setInputState, getInputState };
