import { useCallback, useEffect } from 'react';

export type Modifiers = {
  ctrl?: boolean;
  shift?: boolean;
};

export const isNoModifiers = (modifiers: Modifiers): boolean => {
  return !modifiers.shift && !modifiers.ctrl;
};

// callback should return true if the event is consumed.
export type UseKeyCallback = (code: string, modifiers: Modifiers) => boolean;

export const useKey = (callback: UseKeyCallback) => {
  const onKeydown = useCallback(
    (event: KeyboardEvent) => {
      const consumed = callback(event.code, {
        ctrl: event.ctrlKey,
        shift: event.shiftKey,
      });

      const e = event.composedPath()[0];
      if (
        consumed ||
        !(e instanceof HTMLInputElement || e instanceof HTMLAreaElement)
      ) {
        // 入力項目以外でpreventDefaultすれば大方解決するが、
        // 入力項目でキーボードショートカットにより非入力項目に移動した場合に対応するために
        // consumedをチェックしている
        applyBeepSoundWorkaround(event);
      }
    },
    [callback],
  );

  useEffect(() => {
    window.addEventListener('keydown', onKeydown);
    return () => {
      window.removeEventListener('keydown', onKeydown);
    };
  }, [onKeydown]);
};

// 非入力項目 (input/textarea 以外) でのキー入力でビープ音がなる問題。
// 現在は JavaScript 側で回避するしかない。
// https://github.com/tauri-apps/tauri/issues/2626
const applyBeepSoundWorkaround = (e: KeyboardEvent) => {
  e.preventDefault();
};
