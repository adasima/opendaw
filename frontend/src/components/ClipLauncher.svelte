<script>
  // ダミーデータ: クリップとシーン
  let scenes = $state([
    { id: 1, name: "Scene 1" },
    { id: 2, name: "Scene 2" },
    { id: 3, name: "Scene 3" },
  ]);

  let tracks = $state([
    {
      id: 1,
      name: "Drums",
      color: "var(--primary)",
      clips: [
        { id: 101, sceneId: 1, name: "Beat A", playing: false },
        { id: 102, sceneId: 2, name: "Beat B", playing: false },
      ]
    },
    {
      id: 2,
      name: "Bass",
      color: "#4ade80",
      clips: [
        { id: 201, sceneId: 1, name: "Sub 1", playing: false },
        { id: 202, sceneId: 3, name: "Sub 2", playing: false },
      ]
    },
    {
      id: 3,
      name: "Keys",
      color: "#f472b6",
      clips: [
        { id: 301, sceneId: 2, name: "Chords", playing: false },
      ]
    }
  ]);

  function getClipForScene(track, sceneId) {
    return track.clips.find(c => c.sceneId === sceneId);
  }

  function playClip(clipId) {
    // 該当するクリップを再生状態にする（ダミー動作）
    tracks = tracks.map(track => {
      return {
        ...track,
        clips: track.clips.map(c => ({
          ...c,
          playing: c.id === clipId ? true : false
        }))
      };
    });
    console.log("Playing clip:", clipId);
  }

  function playScene(sceneId) {
    // シーンの全クリップを再生状態にする（ダミー動作）
    tracks = tracks.map(track => {
      const clipForScene = getClipForScene(track, sceneId);
      return {
        ...track,
        clips: track.clips.map(c => ({
          ...c,
          playing: clipForScene && c.id === clipForScene.id ? true : false
        }))
      };
    });
    console.log("Playing scene:", sceneId);
  }
</script>

<div class="clip-launcher-container glass-panel">
  <div class="launcher-header">
    <h3>Clip Launcher</h3>
  </div>

  <div class="launcher-grid">
    <!-- トラックヘッダー -->
    <div class="grid-row header-row">
      {#each tracks as track}
        <div class="grid-cell track-header" style="border-top-color: {track.color}">
          {track.name}
        </div>
      {/each}
      <div class="grid-cell scene-header">Master</div>
    </div>

    <!-- シーン行 -->
    {#each scenes as scene}
      <div class="grid-row">
        {#each tracks as track}
          <div class="grid-cell clip-slot">
            {#if getClipForScene(track, scene.id)}
              {@const clip = getClipForScene(track, scene.id)}
              <button
                class="clip-btn {clip.playing ? 'playing' : ''}"
                style="--clip-color: {track.color}"
                onclick={() => playClip(clip.id)}>
                <span class="play-icon">{clip.playing ? '⏹' : '▶'}</span>
                <span class="clip-name">{clip.name}</span>
              </button>
            {:else}
              <div class="empty-slot"></div>
            {/if}
          </div>
        {/each}
        <div class="grid-cell scene-launch">
          <button class="scene-btn" onclick={() => playScene(scene.id)}>
            <span class="play-icon">▶</span>
            {scene.name}
          </button>
        </div>
      </div>
    {/each}

    <!-- 追加クリップ用ダミー行 -->
    <div class="grid-row">
      {#each tracks as track}
        <div class="grid-cell clip-slot">
           <button class="empty-slot-btn">+</button>
        </div>
      {/each}
      <div class="grid-cell scene-launch"></div>
    </div>
  </div>
</div>

<style>
  .clip-launcher-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    border-radius: 12px;
    overflow: hidden;
  }

  .glass-panel {
    background: rgba(20, 20, 25, 0.4);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .launcher-header {
    padding: 12px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .launcher-header h3 {
    margin: 0;
    font-size: 14px;
    color: var(--on-surface);
  }

  .launcher-grid {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    overflow: auto;
    padding: 8px;
    gap: 4px;
  }

  .grid-row {
    display: flex;
    gap: 4px;
  }

  .header-row {
    margin-bottom: 8px;
  }

  .grid-cell {
    flex: 1;
    min-width: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .track-header {
    font-size: 12px;
    font-weight: bold;
    color: var(--on-surface);
    background: rgba(0, 0, 0, 0.2);
    padding: 8px;
    border-radius: 4px;
    border-top: 3px solid transparent;
    text-align: center;
  }

  .scene-header {
    font-size: 12px;
    color: var(--on-surface-variant);
    background: transparent;
    padding: 8px;
    max-width: 80px;
  }

  .clip-slot, .scene-launch {
    height: 40px;
  }

  .scene-launch {
      max-width: 80px;
  }

  .empty-slot {
    width: 100%;
    height: 100%;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 4px;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .empty-slot-btn {
    width: 100%;
    height: 100%;
    background: rgba(255, 255, 255, 0.02);
    border-radius: 4px;
    border: 1px dashed rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.3);
    cursor: pointer;
    transition: all 0.2s;
  }

  .empty-slot-btn:hover {
      background: rgba(255, 255, 255, 0.05);
      border-color: rgba(255, 255, 255, 0.2);
      color: rgba(255, 255, 255, 0.6);
  }

  .clip-btn {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    padding: 0 8px;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid var(--clip-color);
    border-radius: 4px;
    color: var(--on-surface);
    cursor: pointer;
    transition: all 0.2s;
    overflow: hidden;
    position: relative;
  }

  .clip-btn::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    width: 4px;
    background-color: var(--clip-color);
  }

  .clip-btn:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .clip-btn.playing {
    background: var(--clip-color);
    color: #000;
    font-weight: bold;
    box-shadow: 0 0 10px var(--clip-color);
  }

  .scene-btn {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: var(--on-surface);
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .scene-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .play-icon {
    font-size: 10px;
    margin-right: 4px;
  }

  .clip-btn.playing .play-icon {
      color: rgba(0, 0, 0, 0.6);
  }

  .clip-name {
      font-size: 11px;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
  }
</style>
