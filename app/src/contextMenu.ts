// デフォルトのコンテキストメニュー（WebView 実装による) は適切ではないので無効化する。
// macOS ではたいていの場所で reload とだけ表示される。入力欄は機能的なメニューが表示される。

export const setupContextMenu = () => {
  if (process.env.NODE_ENV === 'development') {
    return;
  }
  document.addEventListener('contextmenu', (e) => {
    // 入力欄では機能的なメニューが表示されるが inspect elements が含まれ、表示させたくないので全部非表示にする。
    // // if event target is input or textarea, show default context menu
    // if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
    //     return;
    // }
    e.preventDefault();
  });
};
