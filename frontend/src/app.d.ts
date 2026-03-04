// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

import type { ParseEnum } from '@2enter/web-kit/types';
import type { CargoKind } from '@/types/model';

declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
	type PageNum = 0 | 1 | 2 | 3 | 4 | 5 | 6;
	type WSData = {
		message?: string;
		data?:
			| {
					type: 'cargo';
					cargo_kind: ParseEnum<CargoKind>;
					id: string;
					directory: string;
			  }
			| {
					type: 'launch';
					cargo_amount: number;
			  }
			| {
					type: 'weather';
					raining: boolean;
			  }
			| {
					type: 'population';
					amount: number;
			  };
	};
}

export {};
