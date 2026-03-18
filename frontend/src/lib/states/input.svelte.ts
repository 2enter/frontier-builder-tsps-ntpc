import type {ParseEnum} from '@2enter/web-kit/types';
import type {Cargo, CargoKind} from '@/types/model';

import {getContext, setContext} from 'svelte';
import axios from 'axios';

// import { getCargoes } from '@/api';

class InputState {
    cargoKind = $state<ParseEnum<CargoKind>>();
    paintTime = $state(0);

    result = $state<Cargo | null>(null);
    resultImgUrl = $state<string | null>(null);

    readonly submittable = $derived(this.cargoKind && this.paintTime);
    readonly requestMetadata = $derived({kind: this.cargoKind, paintTime: this.paintTime});

    constructor() {
        this.cargoKind = this.loadCargoKind();
    }

    reset = () => {
        this.cargoKind = this.loadCargoKind();
        this.paintTime = 0;
        this.result = null;
        this.resultImgUrl = null;
    };

    async getPaint() {
        const url = this.resultImgUrl;
        if (!url) return null;
        return await axios.get<Blob>(url, {responseType: 'blob'}).then((res) => res.data);
        // return new File([blob], 'paint.png');
    }

    loadCargoKind = () => {
        const result = localStorage.getItem('cargo-kind') as ParseEnum<CargoKind> | null;
        if (!result) {
            return;
        }
        this.cargoKind = result;
        return result;
    };

    setCargoKind = (kind: ParseEnum<CargoKind>) => {
        localStorage.setItem('cargo-kind', kind.toString());
    };

    goToConfig = () => {
        window.location.href = '/config';
    };
}

const INPUT_STATE_CTX = 'INPUT_STATE';

function setInputState() {
    return setContext(INPUT_STATE_CTX, new InputState());
}

function getInputState() {
    return getContext<ReturnType<typeof setInputState>>(INPUT_STATE_CTX);
}

export {setInputState, getInputState};
