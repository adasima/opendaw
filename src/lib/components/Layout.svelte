<script lang="ts">
  import TitleBar from "./TitleBar.svelte";
  import BottomBar from "./BottomBar.svelte";
  import LeftSidebar from "./LeftSidebar.svelte";
  import RightSidebar from "./RightSidebar.svelte";
  import CenterArea from "./CenterArea.svelte";
  import SplitPane from "./SplitPane.svelte";
  import SettingsModal from "./SettingsModal.svelte";
  import { isSettingsOpen } from "../stores/settings";
  import { uiState } from "../stores/UIState.svelte";
  import Mixer from "./Mixer.svelte";
</script>

<div
  class="flex flex-col h-screen w-screen bg-theme-wall text-theme-hair overflow-hidden font-sans"
>
  <TitleBar />

  <div class="flex-1 overflow-hidden relative">
    {#if uiState.viewMode === 'arrange'}
        <SplitPane type="horizontal" primary="first" initialSize={192} minSize={50}>
        <div slot="first" class="h-full">
            <LeftSidebar />
        </div>
        <div slot="second" class="h-full">
            <SplitPane
            type="horizontal"
            primary="second"
            initialSize={256}
            minSize={100}
            >
            <div slot="first" class="h-full">
                <CenterArea />
            </div>
            <div slot="second" class="h-full">
                <RightSidebar />
            </div>
            </SplitPane>
        </div>
        </SplitPane>
    {:else if uiState.viewMode === 'mixer'}
        <Mixer />
    {/if}
  </div>

  <BottomBar />
  
  {#if $isSettingsOpen}
    <SettingsModal />
  {/if}
</div>
