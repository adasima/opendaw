<script lang="ts">
    import SplitPane from "./SplitPane.svelte";
    import PianoRoll from "./PianoRoll.svelte";
    import WaveEditor from "./WaveEditor.svelte";
    import TrackList from "./TrackList.svelte";
</script>

<div class="flex-1 bg-zinc-900 relative overflow-hidden h-full">
    <SplitPane type="vertical" primary="second" initialSize={150}>
        <div slot="first" class="w-full h-full">
            <SplitPane type="horizontal" primary="first" initialSize={200}>
                <div
                    slot="first"
                    class="w-full h-full border-r border-theme-wall-shadow"
                >
                    <TrackList />
                </div>
                <div slot="second" class="w-full h-full">
                    <PianoRoll />
                </div>
            </SplitPane>
        </div>
        <div slot="second" class="w-full h-full">
            <WaveEditor />
        </div>
    </SplitPane>
</div>
