export interface SvelteConfig {
  dataset_path: string;
  plate_format: number;
  well_name: string;
  add_meta_cols: string[] | null;
  negative_control: ControlSelection;
  add_controls: ControlSelection[] | null;
}

export interface ControlSelection {
  name: string;
  wells: string[];
}
