
export class UIState {
    viewMode = $state<'arrange' | 'mixer'>('arrange');
    meterMode = $state<'standard' | 'broadcast'>('standard');
}

export const uiState = new UIState();
