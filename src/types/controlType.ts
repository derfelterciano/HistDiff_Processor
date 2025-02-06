export type ControlDefinition = {
	wells: string[];
	title: string | null;
	id: number;
};

export const EMPTY_CONTROL: ControlDefinition = {
	wells: [],
	title: null,
	id: -1,
};
